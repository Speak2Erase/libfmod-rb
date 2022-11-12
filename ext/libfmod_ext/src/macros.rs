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

#[macro_export]
macro_rules! bindable_enum {
    ($name:ident, $($element:ident),+) => {
        paste::paste! {
            #[magnus::wrap(class = "FMOD::Enum::" $name "", free_immediatly, size)]
            #[derive(Clone, Copy, PartialEq)]
            pub(crate) struct $name(libfmod::$name);
        }

        impl From<$name> for libfmod::$name {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<libfmod::$name> for $name {
            fn from(value: libfmod::$name) -> Self {
                Self(value)
            }
        }

        impl $name {
            // FIXME: Make into Result<Self, libfmod::Error>
            fn new(e: std::ffi::c_int) -> Self{
                Self(libfmod::$name::from(e).unwrap())
            }

            fn to_i(&self) -> std::ffi::c_int {
                From::from(self.0)
            }

            fn to_string(&self) -> String {
                format!("FMOD::Enum::{}::{:#?}", stringify!($name), self.0)
            }

            fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
                use magnus::Object;
                use magnus::Module;

                let class = module.define_class(stringify!($name), Default::default())?;
                $(
                    class.const_set(stringify!($element), $name(libfmod::$name::$element))?;
                )+
                class.define_method("to_s", magnus::method!($name::to_string, 0))?;
                class.define_method("inspect", magnus::method!($name::to_string, 0))?;
                class.define_method("==", magnus::method!($name::eq, 1))?;
                class.define_method("to_i", magnus::method!($name::to_i, 0))?;
                class.define_singleton_method("new", magnus::function!($name::new, 1))?;

                Ok(())
            }
        }

        impl crate::wrap::WrapFMOD for libfmod::$name {
            type Output = $name;

            fn wrap_fmod(self) -> Self::Output {
                $name(self)
            }
        }

        impl crate::wrap::UnwrapFMOD for $name {
            type Output = libfmod::$name;

            fn unwrap_fmod(self) -> Self::Output {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! opaque_struct {
    ($name:ident, $mod:literal, $rb_name:literal) => {
        paste::paste! {
            #[magnus::wrap(class = "FMOD::" $mod "::" $rb_name "", free_immediatly, size)]
            #[derive(Clone, Copy)]
            pub(crate) struct $name(libfmod::$name);
        }


        impl crate::wrap::WrapFMOD for libfmod::$name {
            type Output = $name;

            fn wrap_fmod(self) -> Self::Output {
                $name(self)
            }
        }

        impl crate::wrap::UnwrapFMOD for $name {
            type Output = libfmod::$name;

            fn unwrap_fmod(self) -> Self::Output {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! opaque_struct_method {
    ($struct_name:ident, $fn_name:ident $(, $result:ty)?; $( ( $arg:ty $(: $ref:ident)? ) ),*) => {
        paste::paste!{
        #[allow(unused_imports)]
            fn $fn_name(
                &self, 
                $( [<arg_ ${index()}>]: $arg, )*
            ) $( -> $result )? {
                use crate::wrap::WrapFMOD;
                use crate::wrap::UnwrapFMOD;

                self.0.$fn_name($( $( ${ignore(ref)} &)?[<arg_ ${index()}>].unwrap_fmod(), ${ignore(arg)} )*).wrap_fmod()
            }
        }
    };
}

#[macro_export]
macro_rules! opaque_struct_function {
    ($struct_name:ident, $fn_name:ident $(, $result:ty)?;) => {
        #[allow(unused_imports)]
        fn $fn_name() $( -> $result )? {
            use crate::wrap::WrapFMOD;
            use crate::wrap::UnwrapFMOD;

            libfmod::$struct_name::$fn_name().wrap_fmod()
        }
    };
}

#[macro_export]
macro_rules! bind_fn {
    ($name:ident, $rb_name:literal; $(($fn_name:ident, $fn_type:ident, $fn_args:literal)),*) => {
        #[allow(unused_imports, unused_variables)]
        fn bind(module: impl magnus::Module) -> Result<(), magnus::Error> {
            use magnus::Object;
            use magnus::Module;
            use magnus::function as singleton_method;
            use magnus::method;

            let class = module.define_class($rb_name, Default::default())?;
            $(
                paste::paste! {
                    class.[<define_ $fn_type>](stringify!($fn_name), $fn_type!($name::$fn_name, $fn_args))?;
                }
            )*

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! transparent_struct {
    () => {
        
    };
}