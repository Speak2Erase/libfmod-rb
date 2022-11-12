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
use crate::{bind_fn, opaque_struct, opaque_struct_method, opaque_struct_function};

opaque_struct!(Bank, "Studio", "Bank");

impl Bank {

    
    bind_fn! {
        Bank, "Bank";
    }
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    Bank::bind(module)
}