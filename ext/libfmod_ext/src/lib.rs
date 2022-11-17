#![warn(rust_2018_idioms, clippy::all)]
#![feature(macro_metavar_expr)]

use gvl::spawn_rb_thread;
use magnus::Module;

mod bank;
mod bus;
mod command_replay;
mod enums;
mod event;
mod gvl;
mod studio;
mod transparent_struct;
mod vca;
mod wrap;

mod callback;

#[macro_use]
mod macros;

fn parse_id(path: String) -> Result<magnus::RStruct, magnus::Error> {
    unsafe {
        use crate::wrap::WrapFMOD;

        let mut id = libfmod::ffi::FMOD_GUID::default();
        let path = std::ffi::CString::new(path).unwrap();
        match libfmod::ffi::FMOD_Studio_ParseID(path.as_ptr(), &mut id) {
            libfmod::ffi::FMOD_OK => Ok(libfmod::Guid::try_from(id).unwrap().wrap_fmod()),
            error => Err(err_fmod!("FMOD_Studio_System_LookupID", error)),
        }
    }
}

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    unsafe {
        rb_sys::rb_ext_ractor_safe(true);
    }

    magnus::define_global_const("FMOD_CALLBACKS", magnus::RHash::new())?;

    let top = magnus::define_module("FMOD")?;

    let _core = top.define_module("Core")?;
    let studio = top.define_module("Studio")?;
    studio.define_module_function("parse_id", magnus::function!(parse_id, 1))?;
    let enums = top.define_module("Enum")?;

    bank::bind(studio)?;
    bus::bind(studio)?;
    command_replay::bind(studio)?;
    event::bind(studio)?;
    studio::bind_system(studio)?;
    vca::bind(studio)?;

    enums::bind_enums(enums)?;
    transparent_struct::bind(top)?;

    let callback_thread =
        unsafe { magnus::rb_sys::value_from_raw(spawn_rb_thread(callback::callback_thread, ())) };
    top.const_set("EventThread", callback_thread)?;

    Ok(())
}
