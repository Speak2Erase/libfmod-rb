// Copyright (C) 2022 lily
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

use crate::{bind_fn, opaque_struct, opaque_struct_function, opaque_struct_method};

opaque_struct!(System, "Core", "System");

impl System {
    opaque_struct_function!(System, create, Self;);

    fn init(&self, maxchannels: i32, flags: std::ffi::c_uint) -> Result<(), magnus::Error> {
        use crate::wrap::WrapFMOD;

        self.0.init(maxchannels, flags, None).wrap_fmod()
    }

    opaque_struct_method!(close, (););

    bind_fn!(
        System, "System";
        (create, singleton_method, 0),
        (init, method, 2),
        (close, method, 0)
    );
}

pub fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
    System::bind(module)?;

    Ok(())
}
