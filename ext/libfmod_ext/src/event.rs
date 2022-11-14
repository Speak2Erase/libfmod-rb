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
use crate::{bind_fn, opaque_struct, opaque_struct_function, opaque_struct_method};
use crate::{enums::LoadingState, err_fmod};

opaque_struct!(EventDescription, "Studio", "EventDescription");

impl EventDescription {
    fn is_valid(&self) -> bool {
        unsafe { libfmod::ffi::FMOD_Studio_EventDescription_IsValid(self.0.as_mut_ptr()) != 0 }
    }

    opaque_struct_method!(get_id, Result<magnus::RStruct, magnus::Error>;);

    fn get_path(&self) -> Result<String, magnus::Error> {
        unsafe {
            let mut retrieved = 0;

            let result = libfmod::ffi::FMOD_Studio_EventDescription_GetPath(
                self.0.as_mut_ptr(),
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize]);

                    match libfmod::ffi::FMOD_Studio_EventDescription_GetPath(
                        self.0.as_mut_ptr(),
                        cstr.as_ptr() as _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => {
                            use crate::wrap::WrapFMOD;
                            cstr.into_string()
                                .map_err(|e| libfmod::Error::String(e).wrap_fmod())
                        }
                        err => Err(err_fmod!("FMOD_Studio_Bank_GetPath", err)),
                    }
                }
                err => Err(err_fmod!("FMOD_Studio_Bank_GetPath", err)),
            }
        }
    }

    opaque_struct_method!(get_parameter_description_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_parameter_description_by_index, Result<RStruct, magnus::Error>; (i32));
    opaque_struct_method!(get_parameter_description_by_name, Result<RStruct, magnus::Error>; (String: ref));

    opaque_struct_method!(get_user_property_count, Result<i32, magnus::Error>;);

    opaque_struct_method!(get_length, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_min_max_distance, Result<(f32, f32), magnus::Error>;);
    opaque_struct_method!(get_sound_size, Result<f32, magnus::Error>;);
    opaque_struct_method!(is_snapshot, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_oneshot, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_stream, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_3d, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_doppler_enabled, Result<bool, magnus::Error>;);
    opaque_struct_method!(has_sustain_point, Result<bool, magnus::Error>;);

    opaque_struct_method!(get_instance_count, Result<i32, magnus::Error>;);

    opaque_struct_method!(load_sample_data, Result<(), magnus::Error>;);
    opaque_struct_method!(unload_sample_data, Result<(), magnus::Error>;);
    opaque_struct_method!(get_sample_loading_state, Result<LoadingState, magnus::Error>;);
    opaque_struct_method!(release_all_instances, Result<(), magnus::Error>;);

    bind_fn! {
        EventDescription, "EventDescription";
        (is_valid, method, 0),
        (get_id, method, 0),
        (get_path, method, 0),
        (get_parameter_description_count, method, 0),
        (get_parameter_description_by_index, method, 1),
        (get_parameter_description_by_name, method, 1),
        (get_user_property_count, method, 0),
        (get_length, method, 0),
        (get_min_max_distance, method, 0),
        (get_sound_size, method, 0),
        (is_snapshot, method, 0),
        (is_oneshot, method, 0),
        (is_stream, method, 0),
        (is_3d, method, 0),
        (is_doppler_enabled, method, 0),
        (has_sustain_point, method, 0),
        (get_instance_count, method, 0),
        (load_sample_data, method, 0),
        (unload_sample_data, method, 0),
        (get_sample_loading_state, method, 0),
        (release_all_instances, method, 0)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    EventDescription::bind(module)
}
