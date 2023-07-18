#![warn(rust_2018_idioms, clippy::all)]
#![feature(macro_metavar_expr)]

use magnus::{rb_sys::FromRawValue, Module};
use thread::spawn_rb_thread;

mod callback;
mod enums;
mod thread;
mod transparent_struct;
mod wrap;

mod studio {
    pub mod bank;
    pub mod bus;
    pub mod command_replay;
    pub mod event;
    pub mod system;
    pub mod vca;
}

mod core {
    pub mod system;
}

#[macro_use]
mod macros;

fn parse_id(path: String) -> Result<magnus::RStruct, magnus::Error> {
    unsafe {
        use crate::wrap::WrapFMOD;
        use std::mem::MaybeUninit;

        let mut id = MaybeUninit::uninit();
        let path = std::ffi::CString::new(path).unwrap();
        match libfmod::FMOD_Studio_ParseID(path.as_ptr(), id.as_mut_ptr()) {
            libfmod::FMOD_RESULT::FMOD_OK => {
                let id = id.assume_init();
                Ok(id.wrap_fmod())
            }
            error => Err(error.wrap_fmod()),
        }
    }
}

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    unsafe {
        rb_sys::rb_ext_ractor_safe(true);
    }

    let top = magnus::define_module("FMOD")?;

    let core = top.define_module("Core")?;
    let studio = top.define_module("Studio")?;
    studio.define_module_function("parse_id", magnus::function!(parse_id, 1))?;
    let enums = top.define_module("Enum")?;

    enums::bind_enums(enums)?;
    transparent_struct::bind(top)?;

    studio::bank::bind(studio)?;
    studio::bus::bind(studio)?;
    studio::command_replay::bind(studio)?;
    studio::event::bind(studio)?;
    studio::system::bind_system(studio)?;
    studio::vca::bind(studio)?;

    core::system::bind(core)?;

    unsafe {
        let callback_thread = magnus::Value::from_raw(spawn_rb_thread(callback::callback_thread));

        top.const_set("EventThread", callback_thread)?;
    }

    Ok(())
}
