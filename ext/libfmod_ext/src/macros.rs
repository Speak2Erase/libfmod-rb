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
            struct $name(libfmod::$name);
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
    };
}

#[macro_export]
macro_rules! opaque_struct {
    ($name:ident, $mod:literal, $rb_name:literal) => {
        paste::paste! {
            #[magnus::wrap(class = "FMOD::" $mod "::" $rb_name "", free_immediatly, size)]
            #[derive(Clone, Copy)]
            struct $name(libfmod::$name);
        }
    };
}

#[macro_export]
macro_rules! opaque_struct_method {
    () => {
        
    };
}

#[macro_export]
macro_rules! opaque_struct_function {
    () => {
        
    };
}

#[macro_export]
macro_rules! bind_fn {
    ($name:ident, $rb_name:literal $($fn_name:ident, $fn_type:ident, $fn_args:literal),*) => {
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