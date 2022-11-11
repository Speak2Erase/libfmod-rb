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

opaque_struct!(Studio, "Studio", "System");

impl Studio {
    fn create() -> Self {
        Self(libfmod::Studio::create().unwrap())
    }

    bind_fn! {
        Studio, "System"
        create, singleton_method, 0
    }
}

opaque_struct!(System, "Core", "System");

impl System {
    bind_fn! {
        System, "System"
    }
}

pub fn bind_system(core: impl magnus::Module, studio: impl magnus::Module) -> Result<(), magnus::Error> {
    System::bind(core)?;
    Studio::bind(studio)?;

    Ok(())
}
