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
pub unsafe fn without_gvl_no_ubf<Func, FuncArgs, FuncReturn>(
    func: Func,
    func_args: FuncArgs,
) -> FuncReturn
where
    Func: FnMut(FuncArgs) -> FuncReturn,
{
    unsafe extern "C" fn anon_func<Func, FuncArgs, FuncReturn>(data: *mut c_void) -> *mut c_void
    where
        Func: FnMut(FuncArgs) -> FuncReturn,
    {
        let (mut func, func_args): (Func, FuncArgs) = *Box::from_raw(data as *mut (Func, FuncArgs));

        Box::into_raw(Box::new(func(func_args))) as *mut _
    }

    //? SAFETY: We box the function and args to pass them over the FFI boundary.
    let boxed_args = Box::new((func, func_args));

    let result = rb_sys::rb_thread_call_without_gvl(
        Some(anon_func::<Func, FuncArgs, FuncReturn>),
        Box::into_raw(boxed_args) as *mut _,
        None,
        std::ptr::null_mut(),
    );

    *Box::from_raw(result as _)
}

// Type safe wrapper around rb_thread_call_without_gvl. Takes in a unblocking function.
// This function is still very unsafe and should be used sparingly.
pub unsafe fn without_gvl<Func, FuncArgs, FuncReturn, Unblock, UnblockArgs>(
    func: Func,
    func_args: FuncArgs,
    unblock: Unblock,
    unblock_args: UnblockArgs,
) -> FuncReturn
where
    Func: FnMut(FuncArgs) -> FuncReturn,
    Unblock: FnMut(UnblockArgs),
{
    unsafe extern "C" fn anon_func<Func, FuncArgs, FuncReturn>(data: *mut c_void) -> *mut c_void
    where
        Func: FnMut(FuncArgs) -> FuncReturn,
    {
        let (mut func, func_args): (Func, FuncArgs) = *Box::from_raw(data as _);

        Box::into_raw(Box::new(func(func_args))) as _
    }

    unsafe extern "C" fn anon_unblock<Unblock, UnblockArgs>(data: *mut c_void)
    where
        Unblock: FnMut(UnblockArgs),
    {
        let (mut func, func_args): (Unblock, UnblockArgs) = *Box::from_raw(data as _);

        func(func_args);
    }

    //? SAFETY: We box the function and args to pass them over the FFI boundary.
    let boxed_args = Box::new((func, func_args));
    let boxed_unblock_args = Box::new((unblock, unblock_args));

    let result = rb_sys::rb_thread_call_without_gvl(
        Some(anon_func::<Func, FuncArgs, FuncReturn>),
        Box::into_raw(boxed_args) as *mut _,
        Some(anon_unblock::<Unblock, UnblockArgs>),
        Box::into_raw(boxed_unblock_args) as *mut _,
    );

    *Box::from_raw(result as _)
}

// Type safe wrapper around rb_thread_create.
// This function is still very unsafe and should be used sparingly.
pub unsafe fn spawn_rb_thread<Func, FuncArgs>(func: Func, func_args: FuncArgs) -> u64
where
    Func: FnMut(FuncArgs) -> u64,
{
    unsafe extern "C" fn anon<Func, FuncArgs>(data: *mut c_void) -> u64
    where
        Func: FnMut(FuncArgs) -> u64,
    {
        let (mut func, func_args): (Func, FuncArgs) = *Box::from_raw(data as _);

        func(func_args)
    }

    let boxed_args = Box::new((func, func_args));

    rb_sys::rb_thread_create(Some(anon::<Func, FuncArgs>), Box::into_raw(boxed_args) as _)
}
