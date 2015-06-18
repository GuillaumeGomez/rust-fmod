/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FMOD library, is provided by FIRELIGHT TECHNOLOGIES.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

use dsp;
use sound;
use types::TimeUnit;
use fmod_sys;
use file;

/*pub type SystemCallback = Option<fn(fmod: &Sys, _type:SystemCallbackType, command_data1: *mut c_void,
    command_data2: *mut c_void) -> ::Result>;*/

/* file callbacks */
pub type FileOpenCallback = Option<fn(name: &str, unicode: i32) -> Option<(file::FmodFile, Option<fmod_sys::UserData>)>>;
pub type FileCloseCallback = Option<fn(handle: &mut file::FmodFile, user_data: Option<&mut fmod_sys::UserData>)>;
pub type FileReadCallback = Option<fn(handle: &mut file::FmodFile, buffer: &mut [u8], size_to_read: u32, user_data: Option<&mut fmod_sys::UserData>) -> usize>;
pub type FileSeekCallback = Option<fn(handle: &mut file::FmodFile, pos: u32, user_data: Option<&mut fmod_sys::UserData>)>;
/*pub type FMOD_FILE_ASYNCREADCALLBACK = Option<extern "C" fn(arg1: *mut FMOD_ASYNCREADINFO, arg2: *mut c_void) -> ::Result>;
pub type FMOD_FILE_ASYNCCANCELCALLBACK = Option<extern "C" fn(arg1: *mut c_void, arg2: *mut c_void, arg3: c_uint) -> ::Result>;*/

/// sound callback
pub type SoundNonBlockCallback = Option<fn(sound: &sound::Sound, result: ::Result) -> ::Result>;
/// callback which allow to set/change data that will be played
pub type SoundPcmReadCallback = Option<fn(sound: &sound::Sound, data: &mut [i16]) -> ::Result>;
/// notify the user that music position has changed
pub type SoundPcmSetPosCallback = Option<fn(sound: &sound::Sound, sub_sound: i32, position: u32, postype: TimeUnit) -> ::Result>;

/*  codec callbacks */
/*pub type FMOD_CODEC_OPENCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, user_mode: FMOD_MODE, userexinfo: *mut FMOD_CREATESOUNDEXINFO) -> ::Result>;
pub type FMOD_CODEC_CLOSECALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE) -> ::Result>;
pub type FMOD_CODEC_READCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, buffer: *mut c_void, size_bytes: c_uint, bytes_read: *mut c_uint) -> ::Result>;
pub type FMOD_CODEC_GETLENGTHCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, length: *mut c_uint, length_type: FMOD_TIMEUNIT) -> ::Result>;
pub type FMOD_CODEC_SETPOSITIONCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, sub_sound: c_int, position: c_uint, postype: FMOD_TIMEUNIT) -> ::Result>;
pub type FMOD_CODEC_GETPOSITIONCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, position: *mut c_uint, postype: FMOD_TIMEUNIT) -> ::Result>;
pub type FMOD_CODEC_SOUNDCREATECALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, sub_sound: c_int, sound: *mut FMOD_SOUND) -> ::Result>;
pub type FMOD_CODEC_METADATACALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, tag_type:TagType, name: *mut c_char, data: *mut c_void,
        data_len: c_uint, data_type:TagDataType, unique: c_int) -> ::Result>;
pub type FMOD_CODEC_GETWAVEFORMAT = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, index: c_int, wave_format: *mut FMOD_CODEC_WAVEFORMAT) -> ::Result>;
pub type FMOD_3D_ROLLOFFCALLBACK = Option<extern "C" fn(channel: *mut FMOD_CHANNEL, distance: c_float) -> ::Result>;*/

/// notify the user that the DSP has been created
pub type DspCreateCallback = Option<fn(dsp_state: &dsp::DspState) -> ::Result>;
/// notify the user that the DSP has been released
pub type DspReleaseCallback = Option<fn(dsp_state: &dsp::DspState) -> ::Result>;
/// notify the user that the DSP has been reset
pub type DspResetCallback = Option<fn(dsp_state: &dsp::DspState) -> ::Result>;
/// allow the user to modify data that will be read
pub type DspReadCallback = Option<fn(dsp_state: &dsp::DspState, in_buffer: &mut [f32], out_buffer: &mut [f32], length: u32, inchannels: i32, outchannels: i32) -> ::Result>;
/// notify the user that DSP position has changed
pub type DspSetPositionCallback = Option<fn(dsp_state: &dsp::DspState, pos: u32) -> ::Result>;
/// DSP callback
pub type DspSetParamCallback = Option<fn(dsp_state: &dsp::DspState, index: i32, value: f32) -> ::Result>;
/// DSP callback
pub type DspGetParamCallback = Option<fn(dsp_state: &dsp::DspState, index: i32, value: &mut f32, value_str: &str) -> ::Result>;
/// DSP callback, not implemented yet
pub type DspDialogCallback = Option<fn(dsp_state: dsp::DspState/*, hwnd: *mut c_void*/, show: i32) -> ::Result>;
