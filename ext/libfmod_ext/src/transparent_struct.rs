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

use crate::{
    enums::{InstanceType, ParameterType},
    transparent_struct,
};

transparent_struct!(Guid, FMOD_GUID; [
    Data1, data_1: u32,
    Data2, data_2: u16,
    Data3, data_3: u16,
    Data4, data_4: Vec<u8>
]);

transparent_struct!(StudioCpuUsage, FMOD_STUDIO_CPU_USAGE; [update, update: f32]);
transparent_struct!(CpuUsage, FMOD_CPU_USAGE; [
    dsp, dsp: f32,
    stream, stream: f32,
    geometry, geometry: f32,
    update, update: f32, convolution1,
    convolution_1: f32, convolution2,
    convolution_2: f32
]);

transparent_struct!(BufferUsage, FMOD_STUDIO_BUFFER_USAGE; [
    studiocommandqueue, studio_command_queue: RStruct,
    studiohandle, studio_handle: RStruct
]);
transparent_struct!(BufferInfo, FMOD_STUDIO_BUFFER_INFO; [
    currentusage, current_usage: i32,
    peakusage, peak_usage: i32,
    capacity, capacity: i32,
    stallcount, stall_count: i32,
    stalltime, stall_time: f32
]);

transparent_struct!(StudioAdvancedSettings, FMOD_STUDIO_ADVANCEDSETTINGS; [
    cbsize, cb_size: i32,
    commandqueuesize, command_queue_size: u32,
    handleinitialsize, handle_initial_size: u32,
    studioupdateperiod, studio_update_period: i32,
    idlesampledatapoolsize, idle_sample_data_pool_size: i32,
    streamingscheduledelay, streaming_schedule_delay: u32,
    encryptionkey, encryption_key: String
]);

transparent_struct!(ParameterId, FMOD_STUDIO_PARAMETER_ID; [
    data1, data_1: u32,
    data2, data_2: u32
]);

transparent_struct!(ParameterDescription, FMOD_STUDIO_PARAMETER_DESCRIPTION; [
     name, name: String,
     id, id: RStruct,
     minimum, minimum: f32,
     maximum, maximum: f32,
     defaultvalue, default_value: f32,
     type_, type_: &ParameterType,
     flags, flags: std::ffi::c_uint,
     guid, guid: RStruct
]);

fn bind_userproperty(module: impl magnus::Module) -> Result<(), magnus::Error> {
    module.const_set(
        "UserProperty",
        magnus::r_struct::define_struct(Some("UserProperty"), ("name", "type", "data"))?,
    )
}

// FIXME: this will either segfault or leak memory when using string values. THIS IS BAD.
impl crate::wrap::UnwrapFMOD<libfmod::FMOD_STUDIO_USER_PROPERTY> for RStruct {
    fn unwrap_fmod(self) -> libfmod::FMOD_STUDIO_USER_PROPERTY {
        let name: String = self.aref("name").unwrap();
        let name = name.as_ptr() as _;
        let type_ = self
            .aref::<_, &crate::enums::UserPropertyType>("type")
            .unwrap()
            .unwrap_fmod();

        let __bindgen_anon_1 = match type_ {
            libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER => {
                libfmod::FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    intvalue: self.aref("data").unwrap(),
                }
            }
            libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN => {
                libfmod::FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    boolvalue: self.aref("data").unwrap(),
                }
            }
            libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT => {
                libfmod::FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    floatvalue: self.aref("data").unwrap(),
                }
            }
            libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_STRING => {
                libfmod::FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    stringvalue: std::ffi::CString::new(self.aref::<_, String>("data").unwrap())
                        .unwrap()
                        .into_raw(),
                }
            }
            libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_FORCEINT => {
                unreachable!()
            }
        };

        libfmod::FMOD_STUDIO_USER_PROPERTY {
            name,
            type_,
            __bindgen_anon_1,
        }
    }
}

impl crate::wrap::WrapFMOD<RStruct> for libfmod::FMOD_STUDIO_USER_PROPERTY {
    fn wrap_fmod(self) -> RStruct {
        use magnus::{Module, RClass, RModule};

        let rstruct = magnus::class::object()
            .const_get::<_, RModule>("FMOD")
            .unwrap()
            .const_get::<_, RModule>("Struct")
            .unwrap()
            .const_get::<_, RClass>("UserProperty")
            .unwrap();

        let name = unsafe {
            std::ffi::CStr::from_ptr(self.name)
                .to_str()
                .unwrap()
                .to_string()
        };
        let type_ = self.type_;

        RStruct::from_value(
            rstruct
                .new_instance((name, type_.wrap_fmod(), unsafe {
                    match type_ {
                        libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER => {
                            magnus::Value::from(self.__bindgen_anon_1.intvalue)
                        }
                        libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN => {
                            magnus::Value::from(self.__bindgen_anon_1.boolvalue != 0)
                        }
                        libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT => {
                            magnus::Value::from(self.__bindgen_anon_1.floatvalue)
                        }
                        // FIXME: Oh my god this is wildly unsafe
                        libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_STRING => magnus::Value::from(
                            std::ffi::CStr::from_ptr(self.__bindgen_anon_1.stringvalue)
                                .to_str()
                                .unwrap(),
                        ),
                        libfmod::FMOD_STUDIO_USER_PROPERTY_TYPE::FMOD_STUDIO_USER_PROPERTY_TYPE_FORCEINT => unreachable!()
                    }
                }))
                .unwrap(),
        )
        .unwrap()
    }
}

transparent_struct!(Vector, FMOD_VECTOR; [x, x: f32, y, y: f32, z, z: f32]);
transparent_struct!(Attributes3d, FMOD_3D_ATTRIBUTES; [
    position, position: RStruct,
    velocity, velocity: RStruct,
    forward, forward: RStruct,
    up, up: RStruct
]);
transparent_struct!(MemoryUsage, FMOD_STUDIO_MEMORY_USAGE; [
    exclusive, exclusive: i32,
    inclusive, inclusive: i32,
    sampledata, sample_data: i32
]);

transparent_struct!(CommandInfo, FMOD_STUDIO_COMMAND_INFO; [
    commandname, command_name: String,
    parentcommandindex, parent_command_index: i32,
    framenumber, frame_number: i32,
    frametime, frame_time: f32,
    instancetype, instance_type: &InstanceType,
    outputtype, output_type: &InstanceType,
    instancehandle, instance_handle: u32,
    outputhandle, output_handle: u32
]);

transparent_struct!(TimelineMarkerProperties, FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES; [
    name, name: String,
    position, position: i32
]);
transparent_struct!(TimelineBeatProperties, FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES; [
    bar, bar: i32,
    beat, beat: i32,
    position, position: i32,
    tempo, tempo: f32,
    timesignatureupper, time_signature_upper: i32,
    timesignaturelower, time_signature_lower: i32
]);
transparent_struct!(TimelineNestedBeatProperties, FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES; [
    eventid, event_id: RStruct,
    properties, properties: RStruct
]);

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    let module = module.define_module("Struct")?;

    bind_guid(module)?;
    bind_studiocpuusage(module)?;
    bind_cpuusage(module)?;
    bind_bufferusage(module)?;
    bind_bufferinfo(module)?;
    bind_studioadvancedsettings(module)?;
    bind_parameterid(module)?;
    bind_parameterdescription(module)?;
    bind_userproperty(module)?;
    bind_vector(module)?;
    bind_attributes3d(module)?;
    bind_memoryusage(module)?;
    bind_commandinfo(module)?;
    bind_timelinebeatproperties(module)?;
    bind_timelinemarkerproperties(module)?;
    bind_timelinenestedbeatproperties(module)?;

    Ok(())
}
