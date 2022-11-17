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

use std::ffi::c_void;

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use magnus::Module;
use once_cell::sync::Lazy;

pub(crate) trait Callback {
    fn call(&self);
}

type BoxedCallback = Box<dyn Callback + Send>;
type CallbackSender = Sender<Option<BoxedCallback>>;
type CallbackReceiver = Receiver<Option<BoxedCallback>>;

static CHANNEL: Lazy<(CallbackSender, CallbackReceiver)> = Lazy::new(unbounded);

unsafe extern "C" fn call_callback(callback: *mut c_void) -> u64 {
    // Here we get the callback from the pointer (Remember its double boxed so we can pass it around)
    let callback = *Box::from_raw(callback as *mut BoxedCallback);

    #[cfg(feature = "track-callbacks")]
    println!("Attempting ro run callback...");
    // ..Then we call it.
    callback.call();
    // The callback should be dropped and we don't have to worry about a memory leak. Hooray!

    rb_sys::Qnil.into()
}

unsafe extern "C" fn wait_for_callback(_data: *mut c_void) -> *mut c_void {
    #[cfg(feature = "track-callbacks")]
    println!("Waiting for a callback to run...");

    let callback = CHANNEL.1.recv().unwrap();

    #[cfg(feature = "track-callbacks")]
    println!("A callback needs to be run.");

    Box::into_raw(Box::new(callback)) as *mut _
}

unsafe extern "C" fn stop_waiting(_data: *mut c_void) {
    #[cfg(feature = "track-callbacks")]
    println!("Aborting callback thread...");

    // Send a `None` to let notify that we're aborting.
    CHANNEL.0.send(None).unwrap();
}

fn add_callback(callback: BoxedCallback) {
    #[cfg(feature = "track-callbacks")]
    println!("Adding callback to queue");

    CHANNEL.0.send(Some(callback)).unwrap();
}

// Unsafety galore!
pub unsafe extern "C" fn callback_thread(_: *mut c_void) -> u64 {
    loop {
        // Wait until we need to run a callback.
        // This runs wait_for_callback and returns the result it returns.
        let callback = rb_sys::rb_thread_call_without_gvl(
            Some(wait_for_callback),
            std::ptr::null_mut(),
            Some(stop_waiting),
            std::ptr::null_mut(),
        );

        //? SAFETY:
        //? BoxedCallback is a trait object so we need to Box it to pass it around over the ffi boundary.
        //? The Box we get from wait_for_callback should ALWAYS be valid as it returns a pointer from Box::raw.
        let callback = *Box::from_raw(callback as *mut Option<BoxedCallback>);

        // Get the callback we need to run.
        if let Some(callback) = callback {
            #[cfg(feature = "track-callbacks")]
            println!("Spawning a thread to run callback");
            // We need to box it again to pass it over the ffi boundary...
            let callback = Box::into_raw(Box::new(callback));
            // And then we spawn a thread to run the callback so we don't block this one.
            rb_sys::rb_thread_create(Some(call_callback), callback as _);
        } else {
            println!("Callback EventThread termination requested");
            break;
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
