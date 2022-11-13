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

pub(crate) trait UnwrapFMOD<T> {
    fn unwrap_fmod(self) -> T;
}

impl UnwrapFMOD<String> for String {
    fn unwrap_fmod(self) -> String {
        self
    }
}

impl UnwrapFMOD<std::ffi::c_uint> for std::ffi::c_uint {
    fn unwrap_fmod(self) -> std::ffi::c_uint {
        self
    }
}

impl UnwrapFMOD<i32> for i32 {
    fn unwrap_fmod(self) -> i32 {
        self
    }
}

pub(crate) trait WrapFMOD<T> {
    fn wrap_fmod(self) -> T;
}

// Thank YOU so much Bruh#1794!!!
//
// https://discord.com/channels/273534239310479360/1041444308018016306/1041448745994293300
impl<T, TWrap> WrapFMOD<Option<TWrap>> for Option<T>
where
    T: WrapFMOD<TWrap>,
{
    fn wrap_fmod(self) -> Option<TWrap> {
        self.map(WrapFMOD::wrap_fmod)
    }
}

impl<T, TWrap, E, EWrap> WrapFMOD<Result<TWrap, EWrap>> for Result<T, E>
where
    T: WrapFMOD<TWrap>,
    E: WrapFMOD<EWrap>,
{
    fn wrap_fmod(self) -> Result<TWrap, EWrap> {
        self.map(WrapFMOD::wrap_fmod).map_err(WrapFMOD::wrap_fmod)
    }
}

// TODO: Auto impl for TryConvert

impl WrapFMOD<()> for () {
    fn wrap_fmod(self) -> () {
        ()
    }
}

impl WrapFMOD<u32> for u32 {
    fn wrap_fmod(self) -> u32 {
        self
    }
}

impl WrapFMOD<u16> for u16 {
    fn wrap_fmod(self) -> u16 {
        self
    }
}

impl<T, const N: usize> WrapFMOD<Vec<T>> for [T; N] {
    fn wrap_fmod(self) -> Vec<T> {
        Vec::from(self)
    }
}

/// FIXME: Make this behave differently
impl WrapFMOD<magnus::Error> for libfmod::Error {
    fn wrap_fmod(self) -> magnus::Error {
        magnus::Error::runtime_error(match self {
            Self::Fmod { function, code, message } => format!("{function} error E{code}: {message}"),
            Self::EnumBindgen { enumeration, value } => format!("Invalid variant for enum {enumeration}: {value}"),
            Self::String(e) => format!("Error converting string: {e}"),
            Self::StringNul(e) => format!("String nul error: {e}"),
            Self::NotDspFft => format!("Not DSP fft")
        })
    }
}