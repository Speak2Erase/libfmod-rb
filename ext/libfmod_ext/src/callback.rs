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
use once_cell::sync::Lazy;

use crate::bank::Bank;
use crate::command_replay::{CommandCallbackType, CommandUserData};
use crate::event::{EventCallbackParameterType, EventInstance, EventUserData};
use crate::studio::StudioUserData;
use crate::thread::{spawn_rb_thread, without_gvl};

pub(crate) trait Callback {
    fn call(self: Box<Self>);
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
    loop {
        let callback = unsafe {
            without_gvl(
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
            )
        };

        #[cfg(feature = "track-callbacks")]
        println!("A callback needs to be run.");

        #[cfg(feature = "track-callbacks")]
        println!("Attempting to spawn a thread for the callback.");

        // Get the callback we need to run.
        if let Some(callback) = callback {
            #[cfg(feature = "track-callbacks")]
            println!("Spawning a thread to run callback");

            unsafe {
                // This function handles passing the callback over the ffi boundary.
                // It boxes it...
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
            }
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
    // FIXME: 'static lifetime here is BAD.
    user_data: &'static mut StudioUserData,
}

impl StudioSystemCallback {
    pub fn create(
        system: crate::studio::Studio,
        type_: u32,
        data: Option<crate::bank::Bank>,
        user_data: &'static mut StudioUserData,
    ) -> Receiver<i32> {
        let (sender, reciever) = bounded(1);

        let callback = Box::new(Self {
            system,
            type_,
            data,
            sender,
            user_data,
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
    fn call(self: Box<Self>) {
        #[cfg(feature = "track-callbacks")]
        println!("Running callback...");
        let callback = self.user_data.callback.as_deref().copied().unwrap();

        let result = callback
            .funcall(
                "call",
                (
                    self.system,
                    self.type_,
                    self.data,
                    self.user_data.userdata.as_deref().copied(),
                ),
            )
            .unwrap_or_else(|e| {
                println!("WARNING RUBY ERROR IN CALLBACK: {e}");
                0
            });

        #[cfg(feature = "track-callbacks")]
        println!("Callback finished with result {result}");

        self.sender.send(result).unwrap();
    }
}

pub(crate) struct EventCallback {
    event: crate::event::EventInstance,
    type_: u32,
    sender: Sender<i32>,
    parameter: EventCallbackParameterType,
    user_data: &'static mut EventUserData,
}

// I am not happy doing this.
unsafe impl Send for EventCallback {}

impl EventCallback {
    pub fn create(
        event: crate::event::EventInstance,
        type_: u32,
        parameter: EventCallbackParameterType,
        user_data: &'static mut EventUserData,
    ) -> Receiver<i32> {
        let (sender, reciever) = bounded(1);

        let callback = Box::new(Self {
            event,
            type_,
            parameter,
            user_data,
            sender,
        });

        add_callback(callback);

        reciever
    }
}

impl Callback for EventCallback {
    fn call(self: Box<Self>) {
        use crate::wrap::WrapFMOD;

        let callback = self.user_data.callback.as_deref().copied().unwrap();

        let result = callback
            .funcall(
                "call",
                (self.event, self.type_, self.parameter.clone().wrap_fmod()),
            )
            .unwrap_or_else(|e| {
                println!("WARNING RUBY ERROR IN CALLBACK: {e}");
                0
            });

        self.sender.send(result).unwrap();
    }
}

pub(crate) struct CommandReplayCallback {
    type_: CommandCallbackType,
    userdata: &'static mut CommandUserData,
    sender: Sender<i32>,
}

// Augh please don't let this bite me in the ass later
unsafe impl Send for CommandReplayCallback {}

impl CommandReplayCallback {
    pub fn create(
        type_: CommandCallbackType,
        userdata: &'static mut CommandUserData,
    ) -> Receiver<i32> {
        let (sender, reciever) = bounded(1);

        let callback = Self {
            type_,
            userdata,
            sender,
        };

        add_callback(Box::new(callback));

        reciever
    }
}

impl Callback for CommandReplayCallback {
    fn call(self: Box<Self>) {
        use crate::wrap::UnwrapFMOD;
        use crate::wrap::WrapFMOD;

        let callback = self.userdata.create_instance.as_deref().copied().unwrap();
        let userdata = self.userdata.userdata.as_deref().copied();

        let result = match self.type_ {
            CommandCallbackType::Instance {
                replay,
                commandindex,
                description,
                instance,
            } => {
                let result: Result<(i32, Option<&EventInstance>), magnus::Error> = callback
                    .funcall(
                        "call",
                        (
                            replay.wrap_fmod(),
                            commandindex,
                            description.wrap_fmod(),
                            userdata,
                        ),
                    );

                match result {
                    Ok((result, rb_instance)) => {
                        unsafe {
                            if let Some(rb_instance) = rb_instance {
                                *instance = rb_instance.unwrap_fmod().as_mut_ptr();
                            }
                        }

                        result
                    }
                    Err(e) => {
                        println!("WARNING RUBY ERROR IN CALLBACK: {e}");

                        0
                    }
                }
            }
            CommandCallbackType::Frame {
                replay,
                commandindex,
                time,
            } => {
                let result =
                    callback.funcall("call", (replay.wrap_fmod(), commandindex, time, userdata));

                result.unwrap_or_else(|e| {
                    println!("WARNING RUBY ERROR IN CALLBACK: {e}");

                    0
                })
            }
            CommandCallbackType::Bank {
                replay,
                commandindex,
                guid,
                bankfilename,
                flags,
                bank,
            } => {
                //
                let result: Result<(i32, Option<&Bank>), magnus::Error> = callback.funcall(
                    "call",
                    (
                        replay.wrap_fmod(),
                        commandindex,
                        guid.wrap_fmod(),
                        bankfilename,
                        flags,
                        userdata,
                    ),
                );

                match result {
                    Ok((result, rb_bank)) => {
                        unsafe {
                            if let Some(rb_bank) = rb_bank {
                                *bank = rb_bank.unwrap_fmod().as_mut_ptr();
                            }
                        }

                        result
                    }
                    Err(e) => {
                        println!("WARNING RUBY ERROR IN CALLBACK: {e}");

                        0
                    }
                }
            }
        };

        self.sender.send(result).unwrap();
    }
}
