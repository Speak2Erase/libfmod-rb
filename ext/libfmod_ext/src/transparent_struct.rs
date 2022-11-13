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

use crate::transparent_struct;

transparent_struct!(Guid; [data_1, data_2, data_3, data_4]);

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    let module = module.define_module("Struct")?;

    bind_guid(module)
}