#![warn(rust_2018_idioms, clippy::all)]
#![feature(macro_metavar_expr)]

use magnus::Module;

mod enums;
mod system;
mod event;
mod bank;
mod wrap;

#[macro_use]
mod macros;


#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    let top = magnus::define_module("FMOD")?;

    let core = top.define_module("Core")?;
    let studio = top.define_module("Studio")?;
    let enums = top.define_module("Enum")?;

    system::bind_system(core, studio)?;
    bank::bind(studio)?;
    event::bind(studio)?;
    enums::bind_enums(enums)?;

    Ok(())
}
