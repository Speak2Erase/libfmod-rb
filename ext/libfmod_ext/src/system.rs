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

use magnus::RStruct;

use crate::bank::Bank;
use crate::enums::LoadMemoryMode;
use crate::err_fmod;
use crate::event::EventDescription;
#[allow(unused_imports)]
use crate::{bind_fn, opaque_struct, opaque_struct_function, opaque_struct_method};

opaque_struct!(Studio, "Studio", "System");

impl Studio {
    opaque_struct_function!(Studio, create, Result<Self, magnus::Error>;);

    // We define this manually because libfmod gets it wrong.
    fn is_valid(&self) -> bool {
        unsafe { libfmod::ffi::FMOD_Studio_System_IsValid(self.0.as_mut_ptr()) != 0 }
    }

    opaque_struct_method!(set_advanced_settings, Result<(), magnus::Error>; (RStruct));
    opaque_struct_method!(get_advanced_settings, Result<RStruct, magnus::Error>;);

    fn init(
        &self,
        maxchannels: i32,
        studioflags: std::ffi::c_uint,
        flags: std::ffi::c_uint,
    ) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        self.0
            .initialize(maxchannels, studioflags, flags, None)
            .wrap_fmod()
    }

    opaque_struct_method!(update, Result<(), magnus::Error>;);
    opaque_struct_method!(release, Result<(), magnus::Error>;);
    opaque_struct_method!(get_core_system, Result<System, magnus::Error>;);
    opaque_struct_method!(get_event, Result<EventDescription, magnus::Error>; (String: ref));
    opaque_struct_method!(get_bank, Result<Bank, magnus::Error>; (String: ref));
    opaque_struct_method!(load_bank_file, Result<Bank, magnus::Error>; (String: ref), (std::ffi::c_uint));

    // libfmod does NOT define this function correctly.
    // Because of this we have to write it ourselves-
    fn load_bank_memory(
        &self,
        data: Vec<u8>,
        mode: &LoadMemoryMode,
        flags: std::ffi::c_uint,
    ) -> Result<Bank, magnus::Error> {
        use crate::wrap::UnwrapFMOD;
        use crate::wrap::WrapFMOD;

        unsafe {
            let mut bank = std::ptr::null_mut();

            let result = libfmod::ffi::FMOD_Studio_System_LoadBankMemory(
                self.0.as_mut_ptr(),
                data.as_ptr() as *const i8,
                data.len() as i32,
                mode.unwrap_fmod().into(),
                flags,
                &mut bank,
            );
            match result {
                libfmod::ffi::FMOD_OK => Ok(libfmod::Bank::from(bank).wrap_fmod()),
                error => Err(err_fmod!("", error)),
            }
        }
    }

    opaque_struct_method!(get_event_by_id, Result<EventDescription, magnus::Error>; (RStruct));
    opaque_struct_method!(get_bank_by_id, Result<Bank, magnus::Error>; (RStruct));
    opaque_struct_method!(lookup_id, Result<RStruct, magnus::Error>; (String: ref));
    opaque_struct_method!(unload_all, Result<(), magnus::Error>;);
    opaque_struct_method!(flush_commands, Result<(), magnus::Error>;);
    opaque_struct_method!(flush_sample_loading, Result<(), magnus::Error>;);
    opaque_struct_method!(start_command_capture, Result<(), magnus::Error>; (String: ref), (std::ffi::c_uint));
    opaque_struct_method!(stop_command_capture, Result<(), magnus::Error>;);
    opaque_struct_method!(get_bank_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_parameter_description_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_cpu_usage, Result<(RStruct, RStruct), magnus::Error>;);
    opaque_struct_method!(get_buffer_usage, Result<RStruct, magnus::Error>;);
    opaque_struct_method!(reset_buffer_usage, Result<(), magnus::Error>;);

    bind_fn! {
        Studio, "System";
        (create, singleton_method, 0),
        (is_valid, method, 0),
        (set_advanced_settings, method, 1),
        (get_advanced_settings, method, 0),
        (init, method, 3),
        (update, method, 0),
        (release, method, 0),
        (get_core_system, method, 0),
        (get_event, method, 1),
        (get_bank, method, 1),
        (load_bank_file, method, 2),
        (load_bank_memory, method, 3),
        (get_event_by_id, method, 1),
        (get_bank_by_id, method, 1),
        (lookup_id, method, 1),
        (unload_all, method, 0),
        (flush_commands, method, 0),
        (flush_sample_loading, method, 0),
        (start_command_capture, method, 2),
        (stop_command_capture, method, 0),
        (get_bank_count, method, 0),
        (get_parameter_description_count, method, 0),
        (get_cpu_usage, method, 0),
        (get_buffer_usage, method, 0),
        (reset_buffer_usage, method, 0)
    }
}

opaque_struct!(System, "Core", "System");

impl System {
    bind_fn! {
        System, "System";
    }
}

pub fn bind_system(
    core: impl magnus::Module,
    studio: impl magnus::Module,
) -> Result<(), magnus::Error> {
    System::bind(core)?;
    Studio::bind(studio)?;

    Ok(())
}
