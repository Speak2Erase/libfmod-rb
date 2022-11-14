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
use crate::{enums::LoadingState, err_fmod, event::EventDescription};

opaque_struct!(Bank, "Studio", "Bank");

/// FIXME: Add functions with capacity.
/// libfmod-gen does NOT generate them correctly.

impl Bank {
    fn is_valid(&self) -> bool {
        unsafe { libfmod::ffi::FMOD_Studio_Bank_IsValid(self.0.as_mut_ptr()) != 0 }
    }

    opaque_struct_method!(get_id, Result<magnus::RStruct, magnus::Error>;);

    fn get_path(&self) -> Result<String, magnus::Error> {
        // TODO: Make macro
        unsafe {
            let mut retrieved = 0;

            let result = libfmod::ffi::FMOD_Studio_Bank_GetPath(
                self.0.as_mut_ptr(),
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize]);

                    match libfmod::ffi::FMOD_Studio_Bank_GetPath(
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

    opaque_struct_method!(unload, Result<(), magnus::Error>;);
    opaque_struct_method!(load_sample_data, Result<(), magnus::Error>;);
    opaque_struct_method!(unload_sample_data, Result<(), magnus::Error>;);
    opaque_struct_method!(get_loading_state, Result<LoadingState, magnus::Error>;);
    opaque_struct_method!(get_sample_loading_state, Result<LoadingState, magnus::Error>;);
    opaque_struct_method!(get_string_count, Result<i32, magnus::Error>;);

    fn get_string_info(&self, index: i32) -> Result<(magnus::RStruct, String), magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            let mut retrieved = 0;
            let mut guid = libfmod::ffi::FMOD_GUID::default();

            let result = libfmod::ffi::FMOD_Studio_Bank_GetStringInfo(
                self.0.as_mut_ptr(),
                index,
                &mut guid,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize]);

                    match libfmod::ffi::FMOD_Studio_Bank_GetStringInfo(
                        self.0.as_mut_ptr(),
                        index,
                        &mut guid,
                        cstr.as_ptr() as _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => cstr
                            .into_string()
                            .map_err(|e| libfmod::Error::String(e).wrap_fmod())
                            .map(|s| (libfmod::Guid::try_from(guid).unwrap().wrap_fmod(), s)),
                        err => Err(err_fmod!("FMOD_Studio_Bank_GetPath", err)),
                    }
                }
                err => Err(err_fmod!("FMOD_Studio_Bank_GetPath", err)),
            }
        }
    }

    opaque_struct_method!(get_event_count, Result<i32, magnus::Error>;);

    fn get_event_list(&self) -> Result<Vec<EventDescription>, magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            let mut array = vec![std::ptr::null_mut(); self.get_event_count()? as usize];

            let result = libfmod::ffi::FMOD_Studio_Bank_GetEventList(
                self.0.as_mut_ptr(),
                array.as_mut_ptr(),
                array.len() as i32,
                std::ptr::null_mut(),
            );

            match result {
                libfmod::ffi::FMOD_OK => Ok(array
                    .into_iter()
                    .map(|e| libfmod::EventDescription::from(e).wrap_fmod())
                    .collect()),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetEventList", error)),
            }
        }
    }

    opaque_struct_method!(get_bus_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_vca_count, Result<i32, magnus::Error>;);

    bind_fn! {
        Bank, "Bank";
        (is_valid, method, 0),
        (get_id, method, 0),
        (get_path, method, 0),
        (unload, method, 0),
        (load_sample_data, method, 0),
        (unload_sample_data, method, 0),
        (get_loading_state, method, 0),
        (get_sample_loading_state, method, 0),
        (get_string_count, method, 0),
        (get_string_info, method, 1),
        (get_event_count, method, 0),
        (get_event_list, method, 0),
        (get_bus_count, method, 0),
        (get_vca_count, method, 0)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    Bank::bind(module)
}
