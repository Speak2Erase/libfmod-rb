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

use std::ffi::c_void;

// Type safe wrapper around rb_thread_call_without_gvl.
// This function is still very unsafe and should be used sparingly.
pub unsafe fn without_gvl_no_ubf<Func, FuncReturn>(func: Func) -> FuncReturn
where
    Func: FnMut() -> FuncReturn,
{
    unsafe extern "C" fn anon_func<Func, FuncReturn>(data: *mut c_void) -> *mut c_void
    where
        Func: FnMut() -> FuncReturn,
    {
        let mut func: Func = *Box::from_raw(data as *mut Func);

        Box::into_raw(Box::new(func())) as *mut _
    }

    //? SAFETY: We box the function and args to pass them over the FFI boundary.
    let boxed_func = Box::new(func);

    let result = rb_sys::rb_thread_call_without_gvl(
        Some(anon_func::<Func, FuncReturn>),
        Box::into_raw(boxed_func) as *mut _,
        None,
        std::ptr::null_mut(),
    );

    *Box::from_raw(result as _)
}

// Type safe wrapper around rb_thread_call_without_gvl. Takes in a unblocking function.
// This function is still very unsafe and should be used sparingly.
pub unsafe fn without_gvl<Func, FuncReturn, Unblock>(func: Func, unblock: Unblock) -> FuncReturn
where
    Func: FnMut() -> FuncReturn,
    Unblock: FnMut(),
{
    unsafe extern "C" fn anon_func<Func, FuncReturn>(data: *mut c_void) -> *mut c_void
    where
        Func: FnMut() -> FuncReturn,
    {
        let mut func: Func = *Box::from_raw(data as _);

        Box::into_raw(Box::new(func())) as _
    }

    unsafe extern "C" fn anon_unblock<Unblock>(data: *mut c_void)
    where
        Unblock: FnMut(),
    {
        let mut func: Unblock = *Box::from_raw(data as _);

        func();
    }

    //? SAFETY: We box the function and args to pass them over the FFI boundary.
    let boxed_func = Box::new(func);
    let boxed_unblock_func = Box::new(unblock);

    let result = rb_sys::rb_thread_call_without_gvl(
        Some(anon_func::<Func, FuncReturn>),
        Box::into_raw(boxed_func) as *mut _,
        Some(anon_unblock::<Unblock>),
        Box::into_raw(boxed_unblock_func) as *mut _,
    );

    *Box::from_raw(result as _)
}

// Type safe wrapper around rb_thread_create.
// This function is still very unsafe and should be used sparingly.
pub unsafe fn spawn_rb_thread<Func>(func: Func) -> u64
where
    Func: FnMut() -> u64 + 'static,
{
    unsafe extern "C" fn anon<Func>(data: *mut c_void) -> u64
    where
        Func: FnMut() -> u64 + 'static,
    {
        let mut func: Func = *Box::from_raw(data as _);

        func()
    }

    let boxed_func = Box::new(func);

    rb_sys::rb_thread_create(Some(anon::<Func>), Box::into_raw(boxed_func) as _)
}
