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

use magnus::value::BoxValue;

#[allow(unused_imports)]
use crate::{bind_fn, opaque_struct, opaque_struct_function, opaque_struct_method};
use crate::{callback::CommandReplayCallback, err_fmod};
use crate::{enums::PlaybackState, studio::system::Studio};

#[derive(Default)]
pub struct CommandUserData {
    pub userdata: Option<BoxValue<magnus::Value>>,
    pub create_instance: Option<BoxValue<magnus::Value>>,
    pub frame: Option<BoxValue<magnus::Value>>,
    pub bank: Option<BoxValue<magnus::Value>>,
}

pub enum CommandCallbackType {
    Instance {
        replay: libfmod::CommandReplay,
        commandindex: i32,
        description: libfmod::EventDescription,
        instance: *mut *mut libfmod::ffi::FMOD_STUDIO_EVENTINSTANCE,
    },
    Frame {
        replay: libfmod::CommandReplay,
        commandindex: i32,
        time: f32,
    },
    Bank {
        replay: libfmod::CommandReplay,
        commandindex: i32,
        guid: Option<libfmod::Guid>,
        bankfilename: Option<&'static str>,
        flags: u32,
        bank: *mut *mut libfmod::ffi::FMOD_STUDIO_BANK,
    },
}

unsafe extern "C" fn create_instance_callback(
    replay: *mut libfmod::ffi::FMOD_STUDIO_COMMANDREPLAY,
    commandindex: i32,
    eventdescription: *mut libfmod::ffi::FMOD_STUDIO_EVENTDESCRIPTION,
    instance: *mut *mut libfmod::ffi::FMOD_STUDIO_EVENTINSTANCE,
    userdata: *mut std::ffi::c_void,
) -> i32 {
    let reciever = CommandReplayCallback::create(
        CommandCallbackType::Instance {
            replay: libfmod::CommandReplay::from(replay),
            commandindex,
            description: libfmod::EventDescription::from(eventdescription),
            instance,
        },
        &mut *(userdata as *mut _),
    );

    reciever.recv().unwrap_or_else(|e| {
        println!("Warning callback recv error: {e}");
        0
    })
}

unsafe extern "C" fn frame_callback(
    replay: *mut libfmod::ffi::FMOD_STUDIO_COMMANDREPLAY,
    commandindex: i32,
    time: f32,
    userdata: *mut std::ffi::c_void,
) -> i32 {
    let reciever = CommandReplayCallback::create(
        CommandCallbackType::Frame {
            replay: libfmod::CommandReplay::from(replay),
            commandindex,
            time,
        },
        &mut *(userdata as *mut _),
    );

    reciever.recv().unwrap_or_else(|e| {
        println!("Warning callback recv error: {e}");
        0
    })
}

unsafe extern "C" fn load_bank_callback(
    replay: *mut libfmod::ffi::FMOD_STUDIO_COMMANDREPLAY,
    commandindex: i32,
    guid: *const libfmod::ffi::FMOD_GUID,
    bankfilename: *const i8,
    flags: u32,
    bank: *mut *mut libfmod::ffi::FMOD_STUDIO_BANK,
    userdata: *mut std::ffi::c_void,
) -> i32 {
    let guid = guid.as_ref().map(|g| libfmod::Guid::try_from(*g).unwrap());
    let bankfilename = if bankfilename.is_null() {
        None
    } else {
        std::ffi::CStr::from_ptr(bankfilename).to_str().ok()
    };

    let reciever = CommandReplayCallback::create(
        CommandCallbackType::Bank {
            replay: libfmod::CommandReplay::from(replay),
            commandindex,
            guid,
            bankfilename,
            flags,
            bank,
        },
        &mut *(userdata as *mut _),
    );

    reciever.recv().unwrap_or_else(|e| {
        println!("Warning callback recv error: {e}");
        0
    })
}

opaque_struct!(CommandReplay, "Studio", "CommandReplay");

impl CommandReplay {
    fn is_valid(&self) -> bool {
        unsafe { libfmod::ffi::FMOD_Studio_CommandReplay_IsValid(self.0.as_mut_ptr()) != 0 }
    }

    opaque_struct_method!(get_system, Result<Studio, magnus::Error>;);
    opaque_struct_method!(get_length, Result<f32, magnus::Error>;);
    opaque_struct_method!(get_command_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(get_command_info, Result<magnus::RStruct, magnus::Error>; (i32));

    // This function behaves differently than other FMOD functions.
    fn get_command_string(&self, commandindex: i32) -> Result<String, magnus::Error> {
        unsafe {
            // Following the C# bindings here.
            // Start out with a buffer size of 512.
            let mut buffer_size = 512;

            loop {
                // Create a vec with len of buffer_size.
                let mut buffer = vec![0; buffer_size];

                match libfmod::ffi::FMOD_Studio_CommandReplay_GetCommandString(
                    self.0.as_mut_ptr(),
                    commandindex,
                    buffer.as_mut_ptr() as *mut _,
                    buffer_size as _,
                ) {
                    // If the buffer is big enough, convert it to a string and return it.
                    libfmod::ffi::FMOD_OK => {
                        return Ok(String::from_utf8(buffer).unwrap());
                    }
                    // If the buffer needs to be bigger, grow it, and try again.
                    libfmod::ffi::FMOD_ERR_TRUNCATED => {
                        buffer_size += 512;
                        continue;
                    }
                    error => {
                        return Err(err_fmod!(
                            "FMOD_Studio_CommandReplay_GetCommandString",
                            error
                        ))
                    }
                }
            }
        }
    }

    opaque_struct_method!(get_command_at_time, Result<i32, magnus::Error>; (f32));
    opaque_struct_method!(set_bank_path, Result<(), magnus::Error>; (String: ref));
    opaque_struct_method!(start, Result<(), magnus::Error>;);
    opaque_struct_method!(stop, Result<(), magnus::Error>;);
    opaque_struct_method!(seek_to_time, Result<(), magnus::Error>; (f32));
    opaque_struct_method!(seek_to_command, Result<(), magnus::Error>; (i32));
    opaque_struct_method!(get_paused, Result<bool, magnus::Error>;);
    opaque_struct_method!(set_paused, Result<(), magnus::Error>; (bool));
    opaque_struct_method!(get_playback_state, Result<PlaybackState, magnus::Error>;);
    opaque_struct_method!(get_current_command, Result<(i32, f32), magnus::Error>;);
    opaque_struct_method!(release, Result<(), magnus::Error>;);

    fn get_user_data(&self) -> Result<Option<magnus::Value>, magnus::Error> {
        self.get_or_create_user_data()
            .map(|userdata| userdata.userdata.as_ref().map(|b| **b))
    }

    fn set_user_data(&self, val: Option<magnus::Value>) -> Result<(), magnus::Error> {
        self.get_or_create_user_data().map(|userdata| {
            userdata.userdata = val.map(BoxValue::new);
        })
    }

    fn get_or_create_user_data(&self) -> Result<&mut CommandUserData, magnus::Error> {
        use crate::wrap::WrapFMOD;

        let ptr = self.0.get_user_data().map_err(|e| e.wrap_fmod())? as *mut CommandUserData;

        unsafe {
            Ok(ptr.as_mut().unwrap_or_else(|| {
                let raw_ptr: *mut CommandUserData = Box::into_raw(Box::default());
                self.0.set_user_data(raw_ptr as *mut _).unwrap();

                &mut *raw_ptr
            }))
        }
    }

    fn set_create_instance_callback(&self, callback: magnus::Value) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        self.get_or_create_user_data()?.create_instance = Some(BoxValue::new(callback));

        self.0
            .set_create_instance_callback(Some(create_instance_callback))
            .map_err(|e| e.wrap_fmod())
    }

    fn set_frame_callback(&self, callback: magnus::Value) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        self.get_or_create_user_data()?.frame = Some(BoxValue::new(callback));

        self.0
            .set_frame_callback(Some(frame_callback))
            .map_err(|e| e.wrap_fmod())
    }

    fn set_load_bank_callback(&self, callback: magnus::Value) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        self.get_or_create_user_data()?.bank = Some(BoxValue::new(callback));

        self.0
            .set_load_bank_callback(Some(load_bank_callback))
            .map_err(|e| e.wrap_fmod())
    }

    bind_fn! {
        CommandReplay, "CommandReplay";
        (is_valid, method, 0),
        (get_system, method, 0),
        (get_length, method, 0),
        (get_command_count, method, 0),
        (get_command_info, method, 1),
        (get_command_string, method, 1),
        (get_command_at_time, method, 1),
        (set_bank_path, method, 1),
        (start, method, 0),
        (stop, method, 0),
        (seek_to_time, method, 1),
        (seek_to_command, method, 1),
        (get_paused, method, 0),
        (set_paused, method, 1),
        (get_playback_state, method, 0),
        (get_current_command, method, 0),
        (release, method, 0),
        (get_user_data, method, 0),
        (set_user_data, method, 1),
        (set_create_instance_callback, method, 1),
        (set_frame_callback, method, 1),
        (set_load_bank_callback, method, 1)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    CommandReplay::bind(module)
}
