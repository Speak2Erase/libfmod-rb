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

macro_rules! basic_unwrap_impl {
    ($tname:ty) => {
        impl UnwrapFMOD<$tname> for $tname {
            fn unwrap_fmod(self) -> Self {
                self
            }
        }
    };
}

basic_unwrap_impl!(String);
basic_unwrap_impl!(std::ffi::c_uint);
basic_unwrap_impl!(i32);
basic_unwrap_impl!(u16);
basic_unwrap_impl!(f32);
basic_unwrap_impl!(u64);
basic_unwrap_impl!(bool);

impl<T, TUnwrap> UnwrapFMOD<Option<TUnwrap>> for Option<T>
where
    T: UnwrapFMOD<TUnwrap>,
{
    fn unwrap_fmod(self) -> Option<TUnwrap> {
        self.map(UnwrapFMOD::unwrap_fmod)
    }
}

impl<T, const N: usize> UnwrapFMOD<[T; N]> for Vec<T> {
    fn unwrap_fmod(self) -> [T; N] {
        self.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

pub(crate) trait WrapFMOD<T> {
    fn wrap_fmod(self) -> T;
}

macro_rules! basic_wrap_impl {
    ($tname:ty) => {
        impl WrapFMOD<$tname> for $tname {
            fn wrap_fmod(self) -> Self {
                self
            }
        }
    };
}

macro_rules! tuple_wrap_impl {
    ($( $generic:ident),* ) => {
        paste::paste!{
            impl<$( $generic, [<$generic Wrap>],)*> WrapFMOD<( $( [<$generic Wrap>], )* )> for ( $( $generic, )* )
            where $( $generic: WrapFMOD<[<$generic Wrap>]>, )*
            {
                fn wrap_fmod(self) ->
                    (
                        $(
                            [<$generic Wrap>],
                        )*
                    )
                {
                    #![allow(clippy::unused_unit)]
                    (
                        $(
                            ${ignore(generic)}

                            self.${index()}.wrap_fmod(),
                        )*
                    )
                }
            }
        }
    };
}

basic_wrap_impl!(u64);
basic_wrap_impl!(u32);
basic_wrap_impl!(u16);
basic_wrap_impl!(i32);
basic_wrap_impl!(f32);
basic_wrap_impl!(String);
basic_wrap_impl!(bool);

tuple_wrap_impl!();
tuple_wrap_impl!(T1);
tuple_wrap_impl!(T1, T2);
tuple_wrap_impl!(T1, T2, T3);

impl<T, const N: usize> WrapFMOD<Vec<T>> for [T; N] {
    fn wrap_fmod(self) -> Vec<T> {
        Vec::from(self)
    }
}

impl<T> WrapFMOD<Vec<T>> for Vec<T> {
    fn wrap_fmod(self) -> Vec<T> {
        self
    }
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

/// FIXME: Make this behave differently
impl WrapFMOD<magnus::Error> for libfmod::Error {
    fn wrap_fmod(self) -> magnus::Error {
        magnus::Error::runtime_error(match self {
            Self::Fmod {
                function,
                code,
                message,
            } => format!("{function} error E{code}: {message}"),
            Self::EnumBindgen { enumeration, value } => {
                format!("Invalid variant for enum {enumeration}: {value}")
            }
            Self::String(e) => format!("Error converting string: {e}"),
            Self::StringNul(e) => format!("String nul error: {e}"),
            Self::NotDspFft => "Not DSP fft".to_string(),
        })
    }
}
