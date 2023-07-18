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

impl UnwrapFMOD<*const i8> for String {
    fn unwrap_fmod(self) -> *const i8 {
        self.as_str().as_ptr() as _
    }
}

impl WrapFMOD<String> for *const i8 {
    fn wrap_fmod(self) -> String {
        unsafe { std::ffi::CStr::from_ptr(self).to_str().unwrap().to_string() }
    }
}

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

impl WrapFMOD<magnus::Error> for libfmod::FMOD_RESULT {
    fn wrap_fmod(self) -> magnus::Error {
        let str = match self {
            libfmod::FMOD_RESULT::FMOD_OK => "No errors.",
            libfmod::FMOD_RESULT::FMOD_ERR_BADCOMMAND => "Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound::lock on a streaming sound).",
            libfmod::FMOD_RESULT::FMOD_ERR_CHANNEL_ALLOC => "Error trying to allocate a channel.",
            libfmod::FMOD_RESULT::FMOD_ERR_CHANNEL_STOLEN => "The specified channel has been reused to play another sound.",
            libfmod::FMOD_RESULT::FMOD_ERR_DMA => "DMA Failure.  See debug output for more information.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_CONNECTION => "DSP connection error.  Connection possibly caused a cyclic dependency or connected dsps with incompatible buffer counts.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_DONTPROCESS => "DSP return code from a DSP process query callback.  Tells mixer not to call the process callback and therefore not consume CPU.  Use this to optimize the DSP graph.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_FORMAT => "DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format, or a matrix may have been set with the wrong size if the target unit has a specified channel map.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_INUSE => "DSP is already in the mixer's DSP network. It must be removed before being reinserted or released.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_NOTFOUND => "DSP connection error.  Couldn't find the DSP unit specified.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_RESERVED => "DSP operation error.  Cannot perform operation on this DSP as it is reserved by the system.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_SILENCE => "DSP return code from a DSP process query callback.  Tells mixer silence would be produced from read, so go idle and not consume CPU.  Use this to optimize the DSP graph.",
            libfmod::FMOD_RESULT::FMOD_ERR_DSP_TYPE => "DSP operation cannot be performed on a DSP of this type.",
            libfmod::FMOD_RESULT::FMOD_ERR_FILE_BAD => "Error loading file.",
            libfmod::FMOD_RESULT::FMOD_ERR_FILE_COULDNOTSEEK => "Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format.",
            libfmod::FMOD_RESULT::FMOD_ERR_FILE_DISKEJECTED => "Media was ejected while reading.",
            libfmod::FMOD_RESULT::FMOD_ERR_FILE_EOF => "End of file unexpectedly reached while trying to read essential data (truncated?).",
            libfmod::FMOD_RESULT::FMOD_ERR_FILE_ENDOFDATA => "End of current chunk reached while trying to read data.",
            libfmod::FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND => "File not found.",
            libfmod::FMOD_RESULT::FMOD_ERR_FORMAT => "Unsupported file or audio format.",
            libfmod::FMOD_RESULT::FMOD_ERR_HEADER_MISMATCH => "There is a version mismatch between the FMOD header and either the FMOD Studio library or the FMOD Low Level library.",
            libfmod::FMOD_RESULT::FMOD_ERR_HTTP => "A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.",
            libfmod::FMOD_RESULT::FMOD_ERR_HTTP_ACCESS => "The specified resource requires authentication or is forbidden.",
            libfmod::FMOD_RESULT::FMOD_ERR_HTTP_PROXY_AUTH => "Proxy authentication is required to access the specified resource.",
            libfmod::FMOD_RESULT::FMOD_ERR_HTTP_SERVER_ERROR => "A HTTP server error occurred.",
            libfmod::FMOD_RESULT::FMOD_ERR_HTTP_TIMEOUT => "The HTTP request timed out.",
            libfmod::FMOD_RESULT::FMOD_ERR_INITIALIZATION => "FMOD was not initialized correctly to support this function.",
            libfmod::FMOD_RESULT::FMOD_ERR_INITIALIZED => "Cannot call this command after System::init.",
            libfmod::FMOD_RESULT::FMOD_ERR_INTERNAL => "An error occurred that wasn't supposed to.  Contact support.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_FLOAT => "Value passed in was a NaN, Inf or denormalized float.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_HANDLE => "An invalid object handle was used.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_PARAM => "An invalid parameter was passed to this function.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_POSITION => "An invalid seek position was passed to this function.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_SPEAKER => "An invalid speaker was passed to this function based on the current speaker mode.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_SYNCPOINT => "The syncpoint did not come from this sound handle.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_THREAD => "Tried to call a function on a thread that is not supported.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_VECTOR => "The vectors passed in are not unit length, or perpendicular.",
            libfmod::FMOD_RESULT::FMOD_ERR_MAXAUDIBLE => "Reached maximum audible playback count for this sound's soundgroup.",
            libfmod::FMOD_RESULT::FMOD_ERR_MEMORY => "Not enough memory or resources.",
            libfmod::FMOD_RESULT::FMOD_ERR_MEMORY_CANTPOINT => "Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used.",
            libfmod::FMOD_RESULT::FMOD_ERR_NEEDS3D => "Tried to call a command on a 2d sound when the command was meant for 3d sound.",
            libfmod::FMOD_RESULT::FMOD_ERR_NEEDSHARDWARE => "Tried to use a feature that requires hardware support.",
            libfmod::FMOD_RESULT::FMOD_ERR_NET_CONNECT => "Couldn't connect to the specified host.",
            libfmod::FMOD_RESULT::FMOD_ERR_NET_SOCKET_ERROR => "A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere.",
            libfmod::FMOD_RESULT::FMOD_ERR_NET_URL => "The specified URL couldn't be resolved.",
            libfmod::FMOD_RESULT::FMOD_ERR_NET_WOULD_BLOCK => "Operation on a non-blocking socket could not complete immediately.",
            libfmod::FMOD_RESULT::FMOD_ERR_NOTREADY => "Operation could not be performed because specified sound/DSP connection is not ready.",
            libfmod::FMOD_RESULT::FMOD_ERR_OUTPUT_ALLOCATED => "Error initializing output device, but more specifically, the output device is already in use and cannot be reused.",
            libfmod::FMOD_RESULT::FMOD_ERR_OUTPUT_CREATEBUFFER => "Error creating hardware sound buffer.",
            libfmod::FMOD_RESULT::FMOD_ERR_OUTPUT_DRIVERCALL => "A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted.",
            libfmod::FMOD_RESULT::FMOD_ERR_OUTPUT_FORMAT => "Soundcard does not support the specified format.",
            libfmod::FMOD_RESULT::FMOD_ERR_OUTPUT_INIT => "Error initializing output device.",
            libfmod::FMOD_RESULT::FMOD_ERR_OUTPUT_NODRIVERS => "The output device has no drivers installed.  If pre-init, FMOD_OUTPUT_NOSOUND is selected as the output mode.  If post-init, the function just fails.",
            libfmod::FMOD_RESULT::FMOD_ERR_PLUGIN => "An unspecified error has been returned from a plugin.",
            libfmod::FMOD_RESULT::FMOD_ERR_PLUGIN_MISSING => "A requested output, dsp unit type or codec was not available.",
            libfmod::FMOD_RESULT::FMOD_ERR_PLUGIN_RESOURCE => "A resource that the plugin requires cannot be allocated or found. (ie the DLS file for MIDI playback)",
            libfmod::FMOD_RESULT::FMOD_ERR_PLUGIN_VERSION => "A plugin was built with an unsupported SDK version.",
            libfmod::FMOD_RESULT::FMOD_ERR_RECORD => "An error occurred trying to initialize the recording device.",
            libfmod::FMOD_RESULT::FMOD_ERR_REVERB_CHANNELGROUP => "Reverb properties cannot be set on this channel because a parent channelgroup owns the reverb connection.",
            libfmod::FMOD_RESULT::FMOD_ERR_REVERB_INSTANCE => "Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesn't exist.",
            libfmod::FMOD_RESULT::FMOD_ERR_SUBSOUNDS => "The error occurred because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound.",
            libfmod::FMOD_RESULT::FMOD_ERR_SUBSOUND_ALLOCATED => "This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first.",
            libfmod::FMOD_RESULT::FMOD_ERR_SUBSOUND_CANTMOVE => "Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file.",
            libfmod::FMOD_RESULT::FMOD_ERR_TAGNOTFOUND => "The specified tag could not be found or there are no tags.",
            libfmod::FMOD_RESULT::FMOD_ERR_TOOMANYCHANNELS => "The sound created exceeds the allowable input channel count.  This can be increased using the 'maxinputchannels' parameter in System::setSoftwareFormat.",
            libfmod::FMOD_RESULT::FMOD_ERR_TRUNCATED => "The retrieved string is too long to fit in the supplied buffer and has been truncated.",
            libfmod::FMOD_RESULT::FMOD_ERR_UNIMPLEMENTED => "Something in FMOD hasn't been implemented when it should be! contact support!",
            libfmod::FMOD_RESULT::FMOD_ERR_UNINITIALIZED => "This command failed because System::init or System::setDriver was not called.",
            libfmod::FMOD_RESULT::FMOD_ERR_UNSUPPORTED => "A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified.",
            libfmod::FMOD_RESULT::FMOD_ERR_VERSION => "The version number of this file format is not supported.",
            libfmod::FMOD_RESULT::FMOD_ERR_EVENT_ALREADY_LOADED => "The specified bank has already been loaded.",
            libfmod::FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_BUSY => "The live update connection failed due to the game already being connected.",
            libfmod::FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH => "The live update connection failed due to the game data being out of sync with the tool.",
            libfmod::FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT => "The live update connection timed out.",
            libfmod::FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND => "The requested event, parameter, bus or vca could not be found.",
            libfmod::FMOD_RESULT::FMOD_ERR_STUDIO_UNINITIALIZED => "The Studio::System object is not yet initialized.",
            libfmod::FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED => "The specified resource is not loaded, so it can't be unloaded.",
            libfmod::FMOD_RESULT::FMOD_ERR_INVALID_STRING => "An invalid string was passed to this function.",
            libfmod::FMOD_RESULT::FMOD_ERR_ALREADY_LOCKED => "The specified resource is already locked.",
            libfmod::FMOD_RESULT::FMOD_ERR_NOT_LOCKED => "The specified resource is not locked, so it can't be unlocked.",
            libfmod::FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED => "The specified recording driver has been disconnected.",
            libfmod::FMOD_RESULT::FMOD_ERR_TOOMANYSAMPLES => "The length provided exceeds the allowable limit.",
            libfmod::FMOD_RESULT::FMOD_RESULT_FORCEINT => unreachable!()
        };
        magnus::Error::new(magnus::exception::runtime_error(), str)
    }
}
