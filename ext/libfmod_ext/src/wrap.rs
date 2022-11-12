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

pub(crate) trait UnwrapFMOD {
    type Output;

    fn unwrap_fmod(self) -> Self::Output;
}

impl UnwrapFMOD for String {
    type Output = String;

    fn unwrap_fmod(self) -> Self::Output {
        self
    }
}

impl UnwrapFMOD for std::ffi::c_uint {
    type Output = Self;

    fn unwrap_fmod(self) -> Self::Output {
        self
    }
}

impl UnwrapFMOD for i32 {
    type Output = Self;

    fn unwrap_fmod(self) -> Self::Output {
        self
    }
}

pub(crate) trait WrapFMOD {
    type Output;

    fn wrap_fmod(self) -> Self::Output;
}

impl<T, E> WrapFMOD for Result<T, E> 
where T: WrapFMOD, E: WrapFMOD
{    
    type Output = Result<T::Output, E::Output>;

    fn wrap_fmod(self) -> Self::Output {
        self.map(WrapFMOD::wrap_fmod).map_err(WrapFMOD::wrap_fmod)
    }
}

impl<T> WrapFMOD for Option<T> 
where T: WrapFMOD
{
    type Output = Option<T::Output>;

    fn wrap_fmod(self) -> Self::Output {
        self.map(WrapFMOD::wrap_fmod)
    }
}

impl WrapFMOD for () {
    type Output = ();
    
    fn wrap_fmod(self) -> Self::Output {
        ()
    }
}

/// FIXME: Make this behave differently
impl WrapFMOD for libfmod::Error {
    type Output = magnus::Error;

    fn wrap_fmod(self) -> Self::Output {
        magnus::Error::runtime_error(match self {
            Self::Fmod { function, code, message } => format!("{function} error E{code}: {message}"),
            Self::EnumBindgen { enumeration, value } => format!("Invalid variant for enum {enumeration}: {value}"),
            Self::String(e) => format!("Error converting string: {e}"),
            Self::StringNul(e) => format!("String nul error: {e}"),
            Self::NotDspFft => format!("Not DSP fft")
        })
    }
}