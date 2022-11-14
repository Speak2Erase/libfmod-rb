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

use crate::enums::LoadingState;
#[allow(unused_imports)]
use crate::{bind_fn, opaque_struct, opaque_struct_method, opaque_struct_function};

opaque_struct!(Bank, "Studio", "Bank");

/// FIXME: Add functions with capacity.
/// libfmod-gen does NOT generate them correctly.

impl Bank {
    opaque_struct_method!(Bank, is_valid, Result<(), magnus::Error>;);
    opaque_struct_method!(Bank, get_id, Result<magnus::RStruct, magnus::Error>;);
    opaque_struct_method!(Bank, unload, Result<(), magnus::Error>;);
    opaque_struct_method!(Bank, load_sample_data, Result<(), magnus::Error>;);
    opaque_struct_method!(Bank, unload_sample_data, Result<(), magnus::Error>;);
    opaque_struct_method!(Bank, get_loading_state, Result<LoadingState, magnus::Error>;);
    opaque_struct_method!(Bank, get_sample_loading_state, Result<LoadingState, magnus::Error>;);
    opaque_struct_method!(Bank, get_string_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(Bank, get_event_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(Bank, get_bus_count, Result<i32, magnus::Error>;);
    opaque_struct_method!(Bank, get_vca_count, Result<i32, magnus::Error>;);
    
    bind_fn! {
        Bank, "Bank";
        (is_valid, method, 0),
        (get_id, method, 0),
        (unload, method, 0),
        (load_sample_data, method, 0),
        (unload_sample_data, method, 0),
        (get_loading_state, method, 0),
        (get_sample_loading_state, method, 0),
        (get_string_count, method, 0),
        (get_event_count, method, 0),
        (get_bus_count, method, 0),
        (get_vca_count, method, 0)
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    Bank::bind(module)
}