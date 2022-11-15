#![warn(rust_2018_idioms, clippy::all)]
#![feature(macro_metavar_expr)]

use magnus::Module;

mod bank;
mod enums;
mod event;
mod system;
mod transparent_struct;
mod wrap;

mod callback;

#[macro_use]
mod macros;

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    unsafe {
        rb_sys::rb_ext_ractor_safe(true);
    }

    magnus::define_global_const("FMOD_CALLBACKS", magnus::RHash::new())?;

    let top = magnus::define_module("FMOD")?;

    let core = top.define_module("Core")?;
    let studio = top.define_module("Studio")?;
    let enums = top.define_module("Enum")?;

    system::bind_system(core, studio)?;
    bank::bind(studio)?;
    event::bind(studio)?;
    enums::bind_enums(enums)?;
    transparent_struct::bind(top)?;

    unsafe {
        rb_sys::rb_thread_create(Some(callback::callback_thread), std::ptr::null_mut());
    }

    Ok(())
}
