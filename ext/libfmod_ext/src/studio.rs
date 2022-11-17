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

use magnus::{Module, RStruct};

use crate::command_replay::CommandReplay;
use crate::enums::LoadMemoryMode;
use crate::err_fmod;
use crate::event::EventDescription;
use crate::vca::Vca;
use crate::{bank::Bank, callback::StudioSystemCallback};
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

    // We update the system without the GVL so synchronous updates work.
    // If we did not, this would block all threads (remember ruby doesn't run threads in parallel) including the one responsible for running callbacks.
    fn update(&self) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        unsafe extern "C" fn anon(system: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
            // Allocate a box and pass it back to avoid use after free.
            Box::into_raw(Box::new(libfmod::Studio::from(system as _).update())) as _
        }

        unsafe {
            let box_ptr = rb_sys::rb_thread_call_without_gvl(
                Some(anon),
                self.0.as_mut_ptr() as _,
                None,
                std::ptr::null_mut(),
            );

            if box_ptr.is_null() {
                return Err(magnus::Error::runtime_error(
                    "System::update gvl return value is null. Possible corruption has occured.",
                ));
            }

            // Get back the result.
            let box_: Box<Result<(), libfmod::Error>> = Box::from_raw(box_ptr as _);

            box_.map_err(|e| e.wrap_fmod())
        }
    }

    opaque_struct_method!(release, Result<(), magnus::Error>;);
    opaque_struct_method!(get_event, Result<EventDescription, magnus::Error>; (String: ref));
    opaque_struct_method!(get_vca, Result<Vca, magnus::Error>; (String: ref));
    opaque_struct_method!(get_bank, Result<Bank, magnus::Error>; (String: ref));
    opaque_struct_method!(get_event_by_id, Result<EventDescription, magnus::Error>; (RStruct));
    opaque_struct_method!(get_vca_by_id, Result<Vca, magnus::Error>; (RStruct));
    opaque_struct_method!(get_bank_by_id, Result<Bank, magnus::Error>; (RStruct));
    opaque_struct_method!(get_parameter_description_by_id, Result<RStruct, magnus::Error>; (RStruct));
    opaque_struct_method!(get_parameter_description_by_name, Result<RStruct, magnus::Error>; (String: ref));

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

            let result = libfmod::ffi::FMOD_Studio_System_GetParameterLabelByName(
                self.0.as_mut_ptr(),
                name.as_ptr(),
                labelindex,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let mut buffer = vec![0; retrieved as _];

                    match libfmod::ffi::FMOD_Studio_System_GetParameterLabelByName(
                        self.0.as_mut_ptr(),
                        name.as_ptr(),
                        labelindex,
                        buffer.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => Ok(String::from_utf8(buffer).unwrap()),
                        err => Err(err_fmod!("FMOD_Studio_System_GetParameterLabelByName", err)),
                    }
                }
                err => Err(err_fmod!("FMOD_Studio_System_GetParameterLabelByName", err)),
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

            let mut retrieved = 0;
            let id: libfmod::ParameterId = id.unwrap_fmod();
            let id = id.into();

            let result = libfmod::ffi::FMOD_Studio_System_GetParameterLabelByID(
                self.0.as_mut_ptr(),
                id,
                labelindex,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let mut buffer = vec![0; retrieved as _];

                    match libfmod::ffi::FMOD_Studio_System_GetParameterLabelByID(
                        self.0.as_mut_ptr(),
                        id,
                        labelindex,
                        buffer.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => Ok(String::from_utf8(buffer).unwrap()),
                        err => Err(err_fmod!("FMOD_Studio_System_GetParameterLabelByID", err)),
                    }
                }
                err => Err(err_fmod!("FMOD_Studio_System_GetParameterLabelByID", err)),
            }
        }
    }

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

            let result = libfmod::ffi::FMOD_Studio_System_SetParametersByIDs(
                self.0.as_mut_ptr(),
                ids.as_ptr(),
                values.as_mut_ptr(),
                ids.len() as i32,
                ignoreseekspeed as i32,
            );

            match result {
                libfmod::ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetParametersByIDs", error)),
            }
        }
    }

    opaque_struct_method!(get_parameter_by_name, Result<(f32, f32), magnus::Error>; (String: ref));
    opaque_struct_method!(set_parameter_by_name, Result<(), magnus::Error>; (String: ref), (f32), (bool));
    opaque_struct_method!(set_parameter_by_name_with_label, Result<(), magnus::Error>; (String: ref), (String: ref), (bool));

    opaque_struct_method!(lookup_id, Result<RStruct, magnus::Error>; (String: ref));

    fn lookup_path(&self, id: RStruct) -> Result<String, magnus::Error> {
        unsafe {
            use crate::wrap::UnwrapFMOD;

            let mut retrieved = 0;
            let id: libfmod::Guid = id.unwrap_fmod();
            let id = id.into();

            let result = libfmod::ffi::FMOD_Studio_System_LookupPath(
                self.0.as_mut_ptr(),
                &id,
                std::ptr::null_mut(),
                0,
                &mut retrieved,
            );

            match result {
                libfmod::ffi::FMOD_OK | libfmod::ffi::FMOD_ERR_TRUNCATED => {
                    let mut buffer = vec![0; retrieved as _];

                    match libfmod::ffi::FMOD_Studio_System_LookupPath(
                        self.0.as_mut_ptr(),
                        &id,
                        buffer.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        libfmod::ffi::FMOD_OK => Ok(String::from_utf8(buffer).unwrap()),
                        err => Err(err_fmod!("FMOD_Studio_System_LookupPath", err)),
                    }
                }
                error => Err(err_fmod!("FMOD_Studio_System_LookupPath", error)),
            }
        }
    }

    opaque_struct_method!(unload_all, Result<(), magnus::Error>;);
    opaque_struct_method!(flush_commands, Result<(), magnus::Error>;);
    opaque_struct_method!(flush_sample_loading, Result<(), magnus::Error>;);
    opaque_struct_method!(start_command_capture, Result<(), magnus::Error>; (String: ref), (std::ffi::c_uint));
    opaque_struct_method!(stop_command_capture, Result<(), magnus::Error>;);
    opaque_struct_method!(load_command_replay, Result<CommandReplay, magnus::Error>; (String: ref), (std::ffi::c_uint));

    opaque_struct_method!(get_num_listeners, Result<i32, magnus::Error>;);
    opaque_struct_method!(set_num_listeners, Result<(), magnus::Error>; (i32));
    opaque_struct_method!(get_listener_attributes, Result<(RStruct, RStruct), magnus::Error>; (i32));
    opaque_struct_method!(set_listener_attributes, Result<(), magnus::Error>; (i32), (RStruct), (Option<RStruct>));
    opaque_struct_method!(get_listener_weight, Result<f32, magnus::Error>; (i32));
    opaque_struct_method!(set_listener_weight, Result<(), magnus::Error>; (i32), (f32));

    // Because this function *can* be blocking we HAVE to do this to avoid it deadlocking on callbacks.
    fn load_bank_file(
        &self,
        filename: String,
        flags: std::ffi::c_uint,
    ) -> Result<Bank, magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            // Decided to declare a type to get some type checking.
            // We are casting pointers around and casting them to the wrong value is bad...
            type Args = (libfmod::Studio, String, std::ffi::c_uint);
            unsafe extern "C" fn anon(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
                let (system, filename, flags) = (data as *mut Args).as_mut().unwrap();

                Box::into_raw(Box::new(system.load_bank_file(filename, *flags))) as _
            }

            Box::<Result<Bank, libfmod::Error>>::from_raw(rb_sys::rb_thread_call_without_gvl(
                Some(anon),
                &mut (self.0, filename, flags) as *mut Args as _,
                None,
                std::ptr::null_mut(),
            ) as *mut _)
            .map_err(|e| e.wrap_fmod())
        }
    }

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
                error => Err(err_fmod!("FMOD_Studio_System_LoadBankMemory", error)),
            }
        }
    }

    opaque_struct_method!(get_bank_count, Result<i32, magnus::Error>;);

    fn get_bank_list(&self) -> Result<Vec<Bank>, magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            let mut array = Vec::with_capacity(1.max(self.get_bank_count()? as usize));
            let mut count = 0;

            let result = libfmod::ffi::FMOD_Studio_System_GetBankList(
                self.0.as_mut_ptr(),
                array.as_mut_ptr(),
                array.capacity() as i32,
                &mut count as *mut _,
            );
            //? SAFETY:
            //? FMOD ensures that count <= capacity.
            array.set_len(count as _);

            match result {
                libfmod::ffi::FMOD_OK => Ok(array
                    .into_iter()
                    .map(|e| libfmod::Bank::from(e).wrap_fmod())
                    .collect()),
                error => Err(err_fmod!("FMOD_Studio_System_GetBankList", error)),
            }
        }
    }

    opaque_struct_method!(get_parameter_description_count, Result<i32, magnus::Error>;);

    fn get_parameter_description_list(&self) -> Result<Vec<RStruct>, magnus::Error> {
        unsafe {
            use crate::wrap::WrapFMOD;

            let mut array =
                Vec::with_capacity(1.max(self.get_parameter_description_count()? as usize));
            let mut count = 0;

            let result = libfmod::ffi::FMOD_Studio_System_GetParameterDescriptionList(
                self.0.as_mut_ptr(),
                array.as_mut_ptr(),
                array.capacity() as i32,
                &mut count as *mut _,
            );
            //? SAFETY:
            //? FMOD ensures that count <= capacity.
            array.set_len(count as _);

            match result {
                libfmod::ffi::FMOD_OK => Ok(array
                    .into_iter()
                    .map(|e| {
                        libfmod::ParameterDescription::try_from(e)
                            .unwrap()
                            .wrap_fmod()
                    })
                    .collect()),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_GetParameterDescriptionList",
                    error
                )),
            }
        }
    }

    opaque_struct_method!(get_cpu_usage, Result<(RStruct, RStruct), magnus::Error>;);
    opaque_struct_method!(get_buffer_usage, Result<RStruct, magnus::Error>;);
    opaque_struct_method!(reset_buffer_usage, Result<(), magnus::Error>;);

    // This function is a doozy.
    fn set_callback(
        &self,
        callback: magnus::Value,
        mask: std::ffi::c_uint,
    ) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        // First, we set the callback...
        magnus::class::object()
            .const_get::<_, magnus::RHash>("FMOD_CALLBACKS")?
            .aset("fmod_studio_system_callback", callback)?;

        unsafe extern "C" fn anon(
            system: *mut libfmod::ffi::FMOD_STUDIO_SYSTEM,
            type_: u32,
            data: *mut std::ffi::c_void,
            _userdata: *mut std::ffi::c_void,
        ) -> i32 {
            // Here we create a StudioSystemCallback and wait for it to finish.
            let reciever = StudioSystemCallback::create(
                libfmod::Studio::from(system).wrap_fmod(),
                type_,
                if data.is_null() {
                    None
                } else {
                    Some(libfmod::Bank::from(data as _).wrap_fmod())
                },
            );

            #[cfg(feature = "track-callbacks")]
            println!("Waiting for callback response");

            // Wait for a callback result and finish.
            reciever.recv().unwrap_or_else(|e| {
                println!("Warning callback recv error: {e}");
                0
            })
        }

        self.0
            .set_callback(Some(anon), mask)
            .map_err(|e| e.wrap_fmod())
    }

    opaque_struct_method!(get_memory_usage, Result<RStruct, magnus::Error>;);

    bind_fn! {
        Studio, "System";
        (create, singleton_method, 0),
        (is_valid, method, 0),
        (set_advanced_settings, method, 1),
        (get_advanced_settings, method, 0),
        (init, method, 3),
        (update, method, 0),
        (release, method, 0),
        (get_event, method, 1),
        (get_vca, method, 1),
        (get_bank, method, 1),
        (load_bank_file, method, 2),
        (load_bank_memory, method, 3),
        (get_event_by_id, method, 1),
        (get_vca_by_id, method, 1),
        (get_bank_by_id, method, 1),
        (get_parameter_description_by_id, method, 1),
        (get_parameter_description_by_name, method, 1),
        (get_parameter_label_by_name, method, 2),
        (get_parameter_label_by_id, method, 2),
        (get_parameter_by_id, method, 1),
        (set_parameter_by_id, method, 3),
        (set_parameter_by_id_with_label, method, 3),
        (set_parameter_by_ids, method, 3),
        (get_parameter_by_name, method, 1),
        (set_parameter_by_name, method, 3),
        (set_parameter_by_name_with_label, method, 3),
        (lookup_id, method, 1),
        (lookup_path, method, 1),
        (unload_all, method, 0),
        (flush_commands, method, 0),
        (flush_sample_loading, method, 0),
        (start_command_capture, method, 2),
        (stop_command_capture, method, 0),
        (load_command_replay, method, 2),
        (get_bank_count, method, 0),
        (get_bank_list, method, 0),
        (get_num_listeners, method, 0),
        (set_num_listeners, method, 1),
        (get_listener_attributes, method, 1),
        (set_listener_attributes, method, 3),
        (get_listener_weight, method, 1),
        (set_listener_weight, method, 2),
        (get_parameter_description_count, method, 0),
        (get_parameter_description_list, method, 0),
        (get_cpu_usage, method, 0),
        (get_buffer_usage, method, 0),
        (reset_buffer_usage, method, 0),
        (get_memory_usage, method, 0),
        (set_callback, method, 2)
    }
}

pub fn bind_system(studio: impl magnus::Module) -> Result<(), magnus::Error> {
    Studio::bind(studio)?;

    Ok(())
}
