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

transparent_struct!(Guid; [data_1: u32, data_2: u16, data_3: u16, data_4: Vec<u8>]);

transparent_struct!(StudioCpuUsage; [update: f32]);
transparent_struct!(CpuUsage; [dsp: f32, stream: f32, geometry: f32, update: f32, convolution_1: f32, convolution_2: f32]);

transparent_struct!(BufferUsage; [studiocommandqueue: RStruct, studiohandle: RStruct]);
transparent_struct!(BufferInfo; [currentusage: i32, peakusage: i32, capacity: i32, stallcount: i32, stalltime: f32]);

transparent_struct!(StudioAdvancedSettings; [
     commandqueuesize: u32,
     handleinitialsize: u32,
     studioupdateperiod: i32,
     idlesampledatapoolsize: i32,
     streamingscheduledelay: u32,
     encryptionkey: String
]);

transparent_struct!(ParameterId; [
    data_1: u32,
    data_2: u32
]);

transparent_struct!(ParameterDescription; [
     name: String,
     id: RStruct,
     minimum: f32,
     maximum: f32,
     defaultvalue: f32,
     type_: &ParameterType,
     flags: std::ffi::c_uint,
     guid: RStruct
]);

fn bind_userproperty(module: impl magnus::Module) -> Result<(), magnus::Error> {
    module.const_set(
        "UserProperty",
        magnus::r_struct::define_struct(Some("UserProperty"), ("name", "type", "data"))?,
    )
}

impl crate::wrap::UnwrapFMOD<libfmod::UserProperty> for RStruct {
    fn unwrap_fmod(self) -> libfmod::UserProperty {
        let name = self.aref("name").unwrap();
        let type_ = self
            .aref::<_, &crate::enums::UserPropertyType>("type")
            .unwrap()
            .unwrap_fmod();

        let union = match type_ {
            libfmod::UserPropertyType::Integer => libfmod::ffi::FMOD_STUDIO_USER_PROPERTY_UNION {
                intvalue: self.aref("data").unwrap(),
            },
            libfmod::UserPropertyType::Boolean => libfmod::ffi::FMOD_STUDIO_USER_PROPERTY_UNION {
                boolvalue: self.aref("data").unwrap(),
            },
            libfmod::UserPropertyType::Float => libfmod::ffi::FMOD_STUDIO_USER_PROPERTY_UNION {
                floatvalue: self.aref("data").unwrap(),
            },
            libfmod::UserPropertyType::String => libfmod::ffi::FMOD_STUDIO_USER_PROPERTY_UNION {
                stringvalue: std::ffi::CString::new(self.aref::<_, String>("data").unwrap())
                    .unwrap()
                    .into_raw(),
            },
        };

        libfmod::UserProperty { name, type_, union }
    }
}

impl crate::wrap::WrapFMOD<RStruct> for libfmod::UserProperty {
    fn wrap_fmod(self) -> RStruct {
        use magnus::{Module, RClass, RModule};

        let rstruct = magnus::class::object()
            .const_get::<_, RModule>("FMOD")
            .unwrap()
            .const_get::<_, RModule>("Struct")
            .unwrap()
            .const_get::<_, RClass>("UserProperty")
            .unwrap();

        let name = self.name;
        let type_ = self.type_;

        RStruct::from_value(
            rstruct
                .new_instance((name, type_.wrap_fmod(), unsafe {
                    match type_ {
                        libfmod::UserPropertyType::Integer => {
                            magnus::Value::from(self.union.intvalue)
                        }
                        libfmod::UserPropertyType::Boolean => {
                            magnus::Value::from(self.union.boolvalue != 0)
                        }
                        libfmod::UserPropertyType::Float => {
                            magnus::Value::from(self.union.floatvalue)
                        }
                        // FIXME: Oh my god this is wildly unsafe
                        libfmod::UserPropertyType::String => magnus::Value::from(
                            std::ffi::CStr::from_ptr(self.union.stringvalue)
                                .to_str()
                                .unwrap(),
                        ),
                    }
                }))
                .unwrap(),
        )
        .unwrap()
    }
}

transparent_struct!(Vector; [x: f32, y: f32, z: f32]);
transparent_struct!(Attributes3d; [position: RStruct, velocity: RStruct, forward: RStruct, up: RStruct]);
transparent_struct!(MemoryUsage; [exclusive: i32, inclusive: i32, sampledata: i32]);

transparent_struct!(CommandInfo; [
    commandname: String,
    parentcommandindex: i32,
    framenumber: i32,
    frametime: f32,
    instancetype: &InstanceType,
    outputtype: &InstanceType,
    instancehandle: u32,
    outputhandle: u32
]);

transparent_struct!(TimelineMarkerProperties; [name: String, position: i32]);
transparent_struct!(TimelineBeatProperties; [bar: i32, beat: i32, position: i32, tempo: f32, timesignatureupper: i32, timesignaturelower: i32]);
transparent_struct!(TimelineNestedBeatProperties; [eventid: RStruct, properties: RStruct]);

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
