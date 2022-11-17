// Copyright (C) 2022 Lily Lyons
//
// This file is part of libfmod.
//
// libfmod is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// libfmod is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with libfmod.  If not, see <http://www.gnu.org/licenses/>.

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use magnus::Module;
use once_cell::sync::Lazy;

use crate::gvl::{spawn_rb_thread, without_gvl};

pub(crate) trait Callback {
    fn call(&self);
}

type BoxedCallback = Box<dyn Callback + Send>;
type CallbackSender = Sender<Option<BoxedCallback>>;
type CallbackReceiver = Receiver<Option<BoxedCallback>>;

static CHANNEL: Lazy<(CallbackSender, CallbackReceiver)> = Lazy::new(unbounded);

fn add_callback(callback: BoxedCallback) {
    #[cfg(feature = "track-callbacks")]
    println!("Adding callback to queue");

    CHANNEL.0.send(Some(callback)).unwrap();
}

// Unsafety galore!
pub fn callback_thread(_: ()) -> u64 {
    unsafe {
        loop {
            let callback = without_gvl(
                |_| {
                    #[cfg(feature = "track-callbacks")]
                    println!("Waiting for a callback to run...");

                    CHANNEL.1.recv().unwrap()
                },
                (),
                |_| {
                    #[cfg(feature = "track-callbacks")]
                    println!("Aborting callback thread...");

                    // Send a `None` to let notify that we're aborting.
                    CHANNEL.0.send(None).unwrap();
                },
                (),
            );

            #[cfg(feature = "track-callbacks")]
            println!("A callback needs to be run.");

            // Get the callback we need to run.
            if let Some(callback) = callback {
                #[cfg(feature = "track-callbacks")]
                println!("Spawning a thread to run callback");
                // We need to box it again to pass it over the ffi boundary...
                spawn_rb_thread(
                    |callback| {
                        #[cfg(feature = "track-callbacks")]
                        println!("Attempting ro run callback...");
                        // ..Then we call it.
                        callback.call();
                        // The callback should be dropped and we don't have to worry about a memory leak. Hooray!

                        rb_sys::Qnil.into()
                    },
                    callback,
                );
            } else {
                println!("Callback EventThread termination requested");
                break;
            }
        }
    }

    rb_sys::Qnil.into()
}

pub(crate) struct StudioSystemCallback {
    system: crate::studio::Studio,
    type_: u32,
    data: Option<crate::bank::Bank>,
    sender: Sender<i32>,
}

impl StudioSystemCallback {
    pub fn create(
        system: crate::studio::Studio,
        type_: u32,
        data: Option<crate::bank::Bank>,
    ) -> Receiver<i32> {
        let (sender, reciever) = bounded(1);

        let callback = Box::new(Self {
            system,
            type_,
            data,

            sender,
        });

        #[cfg(feature = "track-callbacks")]
        println!("System callback created");

        add_callback(callback);

        reciever
    }
}

#[cfg(feature = "track-callbacks")]
impl Drop for StudioSystemCallback {
    fn drop(&mut self) {
        println!("Callback has been dropped")
    }
}

impl Callback for StudioSystemCallback {
    fn call(&self) {
        #[cfg(feature = "track-callbacks")]
        println!("Running callback...");
        let result = magnus::class::object()
            .const_get::<_, magnus::RHash>("FMOD_CALLBACKS")
            .unwrap()
            .aref::<_, magnus::Value>("fmod_studio_system_callback")
            .unwrap()
            .funcall("call", (self.system, self.type_, self.data))
            .map_err(|e| {
                eprintln!("WARNING RUBY ERROR IN CALLBACK: {e}");
                e
            })
            .unwrap_or(0);

        #[cfg(feature = "track-callbacks")]
        println!("Callback finished with result {result}");

        self.sender.send(result).unwrap();
    }
}
