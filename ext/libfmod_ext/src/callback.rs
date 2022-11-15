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

use std::{collections::VecDeque, sync::Arc};

use once_cell::sync::Lazy;
use parking_lot::{Condvar, Mutex};

use crate::wrap::WrapFMOD;

#[derive(Debug)]
struct Callback {
    callback: magnus::Value,
    output: Arc<(Condvar, Mutex<magnus::Value>)>,
    type_: CallbackType,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum CallbackType {
    StudioSystem {
        system: crate::system::Studio,
        type_: u32,
        data: Option<crate::bank::Bank>,
    },
}

impl WrapFMOD<magnus::RArray> for CallbackType {
    fn wrap_fmod(self) -> magnus::RArray {
        let array = magnus::RArray::new();

        match self {
            Self::StudioSystem {
                system,
                type_,
                data,
            } => {
                array.push(system).unwrap();
                array.push(type_).unwrap();
                array.push(data).unwrap();
            }
        }

        array
    }
}

static QUEUE: Lazy<Mutex<VecDeque<Callback>>> = Lazy::new(Default::default);
static ABORT: Mutex<bool> = Mutex::new(false);
static CONDVAR: Condvar = Condvar::new();

unsafe extern "C" fn call_callback(callback: *mut std::ffi::c_void) -> u64 {
    let callback = (callback as *mut Callback).as_mut().unwrap();

    println!("Calling callback");
    let result = callback
        .callback
        .funcall::<_, _, magnus::Value>("call", callback.type_.wrap_fmod().as_slice())
        .unwrap_or(*magnus::value::QNIL);
    println!("Returning result");

    let (condvar, output) = callback.output.as_ref();

    *output.lock() = result;
    println!("Notified: {}", condvar.notify_all());

    println!("Returning result finished");

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

pub(crate) fn add_callback(
    callback: magnus::Value,
    type_: CallbackType,
) -> Arc<(Condvar, Mutex<magnus::Value>)> {
    let output = Arc::default();

    let callback = Callback {
        callback,
        output: Arc::clone(&output),
        type_,
    };

    QUEUE.lock().push_back(callback);

    CONDVAR.notify_all();

    output
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

        if let Some(mut callback) = QUEUE.lock().pop_front() {
            rb_sys::rb_thread_create(
                Some(call_callback),
                &mut callback as *mut _ as *mut std::ffi::c_void,
            );
        }
    }

    rb_sys::Qnil.into()
}
