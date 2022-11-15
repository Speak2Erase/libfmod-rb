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
use crate::{
    enums::{EventProperty, LoadingState, PlaybackState, StopMode},
    err_fmod,
};

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
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize])
                        .into_raw();

                    match libfmod::ffi::FMOD_Studio_EventDescription_GetPath(
                        self.0.as_mut_ptr(),
                        cstr,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => {
                            use crate::wrap::WrapFMOD;
                            std::ffi::CString::from_raw(cstr)
                                .into_string()
                                .map_err(|e| libfmod::Error::String(e).wrap_fmod())
                        }
                        err => Err(err_fmod!("FMOD_Studio_EventDescription_GetPath", err)),
                    }
                }
                err => Err(err_fmod!("FMOD_Studio_EventDescription_GetPath", err)),
            }
        }
    }

    opaque_struct_method!(get_parameter_description_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_parameter_description_by_index, Result<RStruct, magnus::Error>; (i32));
    opaque_struct_method!(get_parameter_description_by_id, Result<RStruct, magnus::Error>; (RStruct));
    opaque_struct_method!(get_parameter_description_by_name, Result<RStruct, magnus::Error>; (String: ref));

    fn get_parameter_label_by_index(
        &self,
        index: i32,
        labelindex: i32,
    ) -> Result<String, magnus::Error> {
        unsafe {
            let mut retrieved = 0;

            let result = libfmod::ffi::FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                self.0.as_mut_ptr(),
                index,
                labelindex,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize])
                        .into_raw();

                    match libfmod::ffi::FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                        self.0.as_mut_ptr(),
                        index,
                        labelindex,
                        cstr,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => {
                            use crate::wrap::WrapFMOD;
                            std::ffi::CString::from_raw(cstr)
                                .into_string()
                                .map_err(|e| libfmod::Error::String(e).wrap_fmod())
                        }
                        err => Err(err_fmod!(
                            "FMOD_Studio_EventDescription_GetParameterLabelByIndex",
                            err
                        )),
                    }
                }
                err => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterLabelByIndex",
                    err
                )),
            }
        }
    }

    fn get_parameter_label_by_name(
        &self,
        name: String,
        labelindex: i32,
    ) -> Result<String, magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            let mut retrieved = 0;
            let name = std::ffi::CString::new(name)
                .map_err(|e| libfmod::Error::StringNul(e).wrap_fmod())?;

            let result = libfmod::ffi::FMOD_Studio_EventDescription_GetParameterLabelByName(
                self.0.as_mut_ptr(),
                name.as_ptr(),
                labelindex,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize])
                        .into_raw();

                    match libfmod::ffi::FMOD_Studio_EventDescription_GetParameterLabelByName(
                        self.0.as_mut_ptr(),
                        name.as_ptr(),
                        labelindex,
                        cstr,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => std::ffi::CString::from_raw(cstr)
                            .into_string()
                            .map_err(|e| libfmod::Error::String(e).wrap_fmod()),
                        err => Err(err_fmod!(
                            "FMOD_Studio_EventDescription_GetParameterLabelByName",
                            err
                        )),
                    }
                }
                err => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterLabelByName",
                    err
                )),
            }
        }
    }

    fn get_parameter_label_by_id(
        &self,
        id: RStruct,
        labelindex: i32,
    ) -> Result<String, magnus::Error> {
        unsafe {
            use crate::wrap::UnwrapFMOD;
            use crate::wrap::WrapFMOD;

            let mut retrieved = 0;
            let id: libfmod::ParameterId = id.unwrap_fmod();
            let id = id.into();

            let result = libfmod::ffi::FMOD_Studio_EventDescription_GetParameterLabelByID(
                self.0.as_mut_ptr(),
                id,
                labelindex,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let cstr = std::ffi::CString::from_vec_unchecked(vec![0; retrieved as usize])
                        .into_raw();

                    match libfmod::ffi::FMOD_Studio_EventDescription_GetParameterLabelByID(
                        self.0.as_mut_ptr(),
                        id,
                        labelindex,
                        cstr,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => std::ffi::CString::from_raw(cstr)
                            .into_string()
                            .map_err(|e| libfmod::Error::String(e).wrap_fmod()),
                        err => Err(err_fmod!(
                            "FMOD_Studio_EventDescription_GetParameterLabelByID",
                            err
                        )),
                    }
                }
                err => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterLabelByID",
                    err
                )),
            }
        }
    }

    opaque_struct_method!(get_user_property_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_user_property_by_index, Result<RStruct, magnus::Error>; (i32));
    opaque_struct_method!(get_user_property, Result<RStruct, magnus::Error>; (String: ref));
    opaque_struct_method!(get_length, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_min_max_distance, Result<(f32, f32), magnus::Error>;);
    opaque_struct_method!(get_sound_size, Result<f32, magnus::Error>;);
    opaque_struct_method!(is_snapshot, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_oneshot, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_stream, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_3d, Result<bool, magnus::Error>;);
    opaque_struct_method!(is_doppler_enabled, Result<bool, magnus::Error>;);
    opaque_struct_method!(has_sustain_point, Result<bool, magnus::Error>;);
    opaque_struct_method!(create_instance, Result<EventInstance, magnus::Error>;);
    opaque_struct_method!(get_instance_count, Result<i32, magnus::Error>;);

    fn get_instance_list(&self) -> Result<Vec<EventInstance>, magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            let mut array = vec![std::ptr::null_mut(); self.get_instance_count()? as usize];

            let result = libfmod::ffi::FMOD_Studio_EventDescription_GetInstanceList(
                self.0.as_mut_ptr(),
                array.as_mut_ptr(),
                array.len() as i32,
                std::ptr::null_mut(),
            );

            match result {
                libfmod::ffi::FMOD_OK => Ok(array
                    .into_iter()
                    .map(|e| libfmod::EventInstance::from(e).wrap_fmod())
                    .collect()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetInstanceList",
                    error
                )),
            }
        }
    }

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
        (get_parameter_description_by_id, method, 1),
        (get_parameter_description_by_name, method, 1),
        (get_parameter_label_by_index, method, 2),
        (get_parameter_label_by_name, method, 2),
        (get_parameter_label_by_id, method, 2),
        (get_user_property_count, method, 0),
        (get_user_property_by_index, method, 1),
        (get_user_property, method, 1),
        (get_length, method, 0),
        (get_min_max_distance, method, 0),
        (get_sound_size, method, 0),
        (is_snapshot, method, 0),
        (is_oneshot, method, 0),
        (is_stream, method, 0),
        (is_3d, method, 0),
        (is_doppler_enabled, method, 0),
        (has_sustain_point, method, 0),
        (create_instance, method, 0),
        (get_instance_count, method, 0),
        (get_instance_list, method, 0),
        (load_sample_data, method, 0),
        (unload_sample_data, method, 0),
        (get_sample_loading_state, method, 0),
        (release_all_instances, method, 0)
    }
}

opaque_struct!(EventInstance, "Studio", "EventInstance");

impl EventInstance {
    fn is_valid(&self) -> bool {
        unsafe { libfmod::ffi::FMOD_Studio_EventInstance_IsValid(self.0.as_mut_ptr()) != 0 }
    }

    opaque_struct_method!(get_description, Result<EventDescription, magnus::Error>;);
    opaque_struct_method!(get_volume, Result<(f32, f32), magnus::Error>;);
    opaque_struct_method!(set_volume, Result<(), magnus::Error>; (f32));
    opaque_struct_method!(get_pitch, Result<(f32, f32), magnus::Error>;);
    opaque_struct_method!(set_pitch, Result<(), magnus::Error>; (f32));
    opaque_struct_method!(get_3d_attributes, Result<RStruct, magnus::Error>;);
    opaque_struct_method!(set_3d_attributes, Result<(), magnus::Error>; (RStruct));
    opaque_struct_method!(get_listener_mask, Result<u32, magnus::Error>;);
    opaque_struct_method!(set_listener_mask, Result<(), magnus::Error>; (u32));
    opaque_struct_method!(get_property, Result<f32, magnus::Error>; (&EventProperty));
    opaque_struct_method!(set_property, Result<(), magnus::Error>; (&EventProperty), (f32));
    opaque_struct_method!(get_reverb_level, Result<f32, magnus::Error>; (i32));
    opaque_struct_method!(set_reverb_level, Result<(), magnus::Error>; (i32), (f32));
    opaque_struct_method!(get_paused, Result<bool, magnus::Error>;);
    opaque_struct_method!(set_paused, Result<(), magnus::Error>; (bool));
    opaque_struct_method!(start, Result<(), magnus::Error>;);
    opaque_struct_method!(stop, Result<(), magnus::Error>; (&StopMode));
    opaque_struct_method!(get_timeline_position, Result<i32, magnus::Error>;);
    opaque_struct_method!(set_timeline_position, Result<(), magnus::Error>; (i32));
    opaque_struct_method!(get_playback_state, Result<PlaybackState, magnus::Error>;);
    opaque_struct_method!(get_min_max_distance, Result<(f32, f32), magnus::Error>;);
    opaque_struct_method!(release, Result<(), magnus::Error>;);
    opaque_struct_method!(is_virtual, Result<bool, magnus::Error>;);
    opaque_struct_method!(get_parameter_by_name, Result<(f32, f32), magnus::Error>; (String: ref));
    opaque_struct_method!(set_parameter_by_name, Result<(), magnus::Error>; (String: ref), (f32), (bool));
    opaque_struct_method!(set_parameter_by_name_with_label, Result<(), magnus::Error>; (String: ref), (String: ref), (bool));
    opaque_struct_method!(get_parameter_by_id, Result<(f32, f32), magnus::Error>; (RStruct));
    opaque_struct_method!(set_parameter_by_id, Result<(), magnus::Error>; (RStruct), (f32), (bool));
    opaque_struct_method!(set_parameter_by_id_with_label, Result<(), magnus::Error>; (RStruct), (String: ref), (bool));

    fn set_parameter_by_ids(
        &self,
        ids: magnus::RArray,
        mut values: Vec<f32>,
        ignoreseekspeed: bool,
    ) -> Result<(), magnus::Error> {
        unsafe {
            use crate::wrap::UnwrapFMOD;

            let ids: Vec<_> = ids
                .as_slice()
                .iter()
                .map(|id| {
                    let struct_ = RStruct::from_value(*id).unwrap();
                    let id: libfmod::ParameterId = struct_.unwrap_fmod();
                    id.into()
                })
                .collect();

            assert_eq!(
                ids.len(),
                values.len(),
                "The two arrays should be the same length"
            );

            let result = libfmod::ffi::FMOD_Studio_EventInstance_SetParametersByIDs(
                self.0.as_mut_ptr(),
                ids.as_ptr(),
                values.as_mut_ptr(),
                ids.len() as i32,
                ignoreseekspeed as i32,
            );

            match result {
                libfmod::ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetParametersByIDs",
                    error
                )),
            }
        }
    }

    opaque_struct_method!(key_off, Result<(), magnus::Error>;);
    opaque_struct_method!(get_cpu_usage, Result<(u32, u32), magnus::Error>;);
    opaque_struct_method!(get_memory_usage, Result<RStruct, magnus::Error>;);

    bind_fn! {
        EventInstance, "EventInstance";
        (is_valid, method, 0),
        (get_description, method, 0),
        (get_volume, method, 0),
        (set_volume, method, 1),
        (get_pitch, method, 0),
        (set_pitch, method, 1),
        (get_3d_attributes, method, 0),
        (set_3d_attributes, method, 1),
        (get_listener_mask, method, 0),
        (set_listener_mask, method, 1),
        (get_property, method, 1),
        (set_property, method, 2),
        (get_reverb_level, method, 1),
        (set_reverb_level, method, 2),
        (get_paused, method, 0),
        (set_paused, method, 1),
        (start, method, 0),
        (stop, method, 1),
        (get_timeline_position, method, 0),
        (set_timeline_position, method, 1),
        (get_playback_state, method, 0),
        (get_min_max_distance, method, 0),
        (release, method, 0),
        (is_virtual, method, 0),
        (get_parameter_by_name, method, 1),
        (set_parameter_by_name, method, 3),
        (set_parameter_by_name_with_label, method, 3),
        (get_parameter_by_id, method, 1),
        (set_parameter_by_id, method, 3),
        (set_parameter_by_id_with_label, method, 3),
        (set_parameter_by_ids, method, 3),
        (key_off, method, 0),
        (get_cpu_usage, method, 0),
        (get_memory_usage, method, 0)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    EventDescription::bind(module)?;
    EventInstance::bind(module)
}
