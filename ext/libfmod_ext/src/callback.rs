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

use std::{
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender},
};

use magnus::Module;
use once_cell::sync::Lazy;
use parking_lot::{Condvar, Mutex};

pub(crate) trait Callback {
    fn call(&self);
}

static QUEUE: Lazy<Mutex<VecDeque<Box<dyn Callback + Send>>>> = Lazy::new(Default::default);
static ABORT: Mutex<bool> = Mutex::new(false);
static CONDVAR: Condvar = Condvar::new();

unsafe extern "C" fn call_callback(callback: *mut std::ffi::c_void) -> u64 {
    let callback = *Box::from_raw(callback as *mut Box<dyn Callback + Send>);

    callback.call();

    rb_sys::Qnil.into()
}

unsafe extern "C" fn wait_for_callback(_data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let mut queue = QUEUE.lock();
    CONDVAR.wait(&mut queue);

    std::ptr::null_mut()
}

unsafe extern "C" fn stop_waiting(_data: *mut std::ffi::c_void) {
    *ABORT.lock() = true;

    CONDVAR.notify_all();
}

pub(crate) fn add_callback(callback: Box<dyn Callback + Send>) {
    QUEUE.lock().push_back(callback);

    CONDVAR.notify_all();
}

pub unsafe extern "C" fn callback_thread(_: *mut std::ffi::c_void) -> u64 {
    loop {
        rb_sys::rb_thread_call_without_gvl(
            Some(wait_for_callback),
            std::ptr::null_mut(),
            Some(stop_waiting),
            std::ptr::null_mut(),
        );

        if *ABORT.lock() {
            break;
        }

        if let Some(callback) = QUEUE.lock().pop_front() {
            let callback = Box::into_raw(Box::new(callback));
            rb_sys::rb_thread_create(Some(call_callback), callback as _);
        }
    }

    rb_sys::Qnil.into()
}

pub(crate) struct StudioSystemCallback {
    system: crate::system::Studio,
    type_: u32,
    data: Option<crate::bank::Bank>,
    sender: Sender<i32>,
}

impl StudioSystemCallback {
    pub fn create(
        system: crate::system::Studio,
        type_: u32,
        data: Option<crate::bank::Bank>,
    ) -> Receiver<i32> {
        let (sender, reciever) = channel();

        let callback = Box::new(Self {
            system,
            type_,
            data,

            sender,
        });

        add_callback(callback);

        reciever
    }
}

impl Callback for StudioSystemCallback {
    fn call(&self) {
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

        self.sender.send(result).unwrap();
    }
}
