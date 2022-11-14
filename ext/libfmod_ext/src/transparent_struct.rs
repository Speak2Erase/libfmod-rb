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

use crate::transparent_struct;

transparent_struct!(Guid; [data_1: u32, data_2: u16, data_3: u16, data_4: Vec<u8>]);

transparent_struct!(StudioCpuUsage; [update: f32]);
transparent_struct!(CpuUsage; [dsp: f32, stream: f32, geometry: f32, update: f32, convolution_1: f32, convolution_2: f32]);

transparent_struct!(BufferUsage; [studiocommandqueue: RStruct, studiohandle: RStruct]);
transparent_struct!(BufferInfo; [currentusage: i32, peakusage: i32, capacity: i32, stallcount: i32, stalltime: f32]);

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    let module = module.define_module("Struct")?;

    bind_guid(module)?;
    bind_studiocpuusage(module)?;
    bind_cpuusage(module)?;
    bind_bufferusage(module)?;
    bind_bufferinfo(module)
}