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

#[allow(unused_imports)]
use crate::{bind_fn, opaque_struct, opaque_struct_method, opaque_struct_function};
use crate::event::EventDescription;
use crate::bank::Bank;
use crate::enums::LoadMemoryMode;

opaque_struct!(Studio, "Studio", "System");

impl Studio {
    opaque_struct_function!(Studio, create, Result<Self, magnus::Error>;);
    opaque_struct_method!(Studio, is_valid, Result<(), magnus::Error>;);
    
    fn init(&self, maxchannels: i32, studioflags: std::ffi::c_uint, flags: std::ffi::c_uint) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        self.0.initialize(maxchannels, studioflags, flags, None).wrap_fmod()
    }

    opaque_struct_method!(Studio, update, Result<(), magnus::Error>;);
    opaque_struct_method!(Studio, release, Result<(), magnus::Error>;);
    opaque_struct_method!(Studio, get_core_system, Result<System, magnus::Error>;);
    opaque_struct_method!(Studio, get_event, Result<EventDescription, magnus::Error>; (String: ref));
    opaque_struct_method!(Studio, get_bank, Result<Bank, magnus::Error>; (String: ref));
    opaque_struct_method!(Studio, load_bank_file, Result<Bank, magnus::Error>; (String: ref), (std::ffi::c_uint));
    opaque_struct_method!(Studio, load_bank_memory, Result<Bank, magnus::Error>; (String: ref), (i32), (&LoadMemoryMode), (std::ffi::c_uint));
    opaque_struct_method!(Studio, get_event_by_id, Result<EventDescription, magnus::Error>; (RStruct));
    opaque_struct_method!(Studio, get_bank_by_id, Result<Bank, magnus::Error>; (RStruct));
    opaque_struct_method!(Studio, lookup_id, Result<RStruct, magnus::Error>; (String: ref));
    opaque_struct_method!(Studio, unload_all, Result<(), magnus::Error>;);
    opaque_struct_method!(Studio, flush_commands, Result<(), magnus::Error>;);
    opaque_struct_method!(Studio, flush_sample_loading, Result<(), magnus::Error>;);
    opaque_struct_method!(Studio, start_command_capture, Result<(), magnus::Error>; (String: ref), (std::ffi::c_uint));
    opaque_struct_method!(Studio, stop_command_capture, Result<(), magnus::Error>;);
    opaque_struct_method!(Studio, get_bank_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(Studio, get_parameter_description_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(Studio, get_cpu_usage, Result<(RStruct, RStruct), magnus::Error>;);
    opaque_struct_method!(Studio, get_buffer_usage, Result<RStruct, magnus::Error>;);
    opaque_struct_method!(Studio, reset_buffer_usage, Result<(), magnus::Error>;);

    bind_fn! {
        Studio, "System";
        (create, singleton_method, 0),
        (is_valid, method, 0),
        (init, method, 3),
        (update, method, 0),
        (release, method, 0),
        (get_core_system, method, 0),
        (get_event, method, 1),
        (get_bank, method, 1),
        (load_bank_file, method, 2),
        (load_bank_memory, method, 4),
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

pub fn bind_system(core: impl magnus::Module, studio: impl magnus::Module) -> Result<(), magnus::Error> {
    System::bind(core)?;
    Studio::bind(studio)?;

    Ok(())
}
