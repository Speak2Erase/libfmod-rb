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

#[allow(unused_imports)]
use crate::{bind_fn, opaque_struct, opaque_struct_function, opaque_struct_method};
use crate::{enums::StopMode, err_fmod};

opaque_struct!(Bus, "Studio", "Bus");

impl Bus {
    fn is_valid(&self) -> bool {
        unsafe { libfmod::ffi::FMOD_Studio_Bus_IsValid(self.0.as_mut_ptr()) != 0 }
    }

    opaque_struct_method!(get_id, magnus::RStruct;);

    fn get_path(&self) -> Result<String, magnus::Error> {
        // TODO: Make macro
        unsafe {
            let mut retrieved = 0;

            let result = libfmod::ffi::FMOD_Studio_Bus_GetPath(
                self.0.as_mut_ptr(),
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let mut buffer = vec![0; retrieved as _];

                    match libfmod::ffi::FMOD_Studio_Bus_GetPath(
                        self.0.as_mut_ptr(),
                        buffer.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => Ok(String::from_utf8(buffer).unwrap()),
                        err => Err(err_fmod!("FMOD_Studio_Bus_GetPath", err)),
                    }
                }
                err => Err(err_fmod!("FMOD_Studio_Bus_GetPath", err)),
            }
        }
    }

    opaque_struct_method!(get_volume, (f32, f32););
    opaque_struct_method!(set_volume, (); (f32));
    opaque_struct_method!(get_paused, bool;);
    opaque_struct_method!(set_paused, (); (bool));
    opaque_struct_method!(get_mute, bool;);
    opaque_struct_method!(set_mute, (); (bool));
    opaque_struct_method!(stop_all_events, (); (&StopMode));
    opaque_struct_method!(get_port_index, u64;);
    opaque_struct_method!(set_port_index, (); (u64));
    opaque_struct_method!(lock_channel_group, (););
    opaque_struct_method!(unlock_channel_group, (););

    // TODO: ChannelGroup

    opaque_struct_method!(get_cpu_usage, (u32, u32););
    opaque_struct_method!(get_memory_usage, magnus::RStruct;);

    bind_fn! {
        Bus, "Bus";
        (is_valid, method, 0),
        (get_id, method, 0),
        (get_path, method, 0),
        (get_volume, method, 0),
        (set_volume, method, 1),
        (get_paused, method, 0),
        (set_paused, method, 1),
        (get_mute, method, 0),
        (set_mute, method, 1),
        (stop_all_events, method, 1),
        (get_port_index, method, 0),
        (set_port_index, method, 1),
        (lock_channel_group, method, 0),
        (unlock_channel_group, method, 0),
        (get_cpu_usage, method, 0),
        (get_memory_usage, method, 0)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    Bus::bind(module)
}
