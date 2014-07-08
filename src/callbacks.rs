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

use ffi;
use dsp;
use enums::*;

/*pub type SystemCallback = Option<fn(fmod: &FmodSys, _type: fmod::SystemCallbackType, command_data1: *mut c_void,
    command_data2: *mut c_void) -> fmod::Result>;*/

/* file callbacks */
/*pub type FMOD_FILE_OPENCALLBACK = Option<extern "C" fn(name: *mut c_char, unicode: int, file_size: *mut c_uint, handle: *mut *mut c_void,
    user_data: *mut *mut c_void) -> fmod::Result>;
pub type FMOD_FILE_CLOSECALLBACK = Option<extern "C" fn(handle: *mut c_void, user_data: *mut c_void) -> fmod::Result>;
pub type FMOD_FILE_READCALLBACK = Option<extern "C" fn(handle: *mut c_void, buffer: *mut c_void, size_bytes: c_uint, bytes_read: *mut c_uint,
    user_data: *mut c_void) -> fmod::Result>;
pub type FMOD_FILE_SEEKCALLBACK = Option<extern "C" fn(handle: *mut c_void, pos: c_uint, user_data: *mut c_void) -> fmod::Result>;
pub type FMOD_FILE_ASYNCREADCALLBACK = Option<extern "C" fn(arg1: *mut FMOD_ASYNCREADINFO, arg2: *mut c_void) -> fmod::Result>;
pub type FMOD_FILE_ASYNCCANCELCALLBACK = Option<extern "C" fn(arg1: *mut c_void, arg2: *mut c_void, arg3: c_uint) -> fmod::Result>;*/

/* sound callbacks */
/*pub type FMOD_SOUND_NONBLOCKCALLBACK = Option<extern "C" fn(sound: *mut FMOD_SOUND, result: fmod::Result) -> fmod::Result>;
pub type FMOD_SOUND_PCMREADCALLBACK = Option<extern "C" fn(sound: *mut FMOD_SOUND, data: *mut c_void, data_len: c_uint) -> fmod::Result>;
pub type FMOD_SOUND_PCMSETPOSCALLBACK = Option<extern "C" fn(sound: *mut FMOD_SOUND, sub_sound: c_int, position: c_uint,
        postype: FMOD_TIMEUNIT) -> fmod::Result>;*/

/*  codec callbacks */
/*pub type FMOD_CODEC_OPENCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, user_mode: FMOD_MODE, userexinfo: *mut FMOD_CREATESOUNDEXINFO) -> fmod::Result>;
pub type FMOD_CODEC_CLOSECALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE) -> fmod::Result>;
pub type FMOD_CODEC_READCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, buffer: *mut c_void, size_bytes: c_uint, bytes_read: *mut c_uint) -> fmod::Result>;
pub type FMOD_CODEC_GETLENGTHCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, length: *mut c_uint, length_type: FMOD_TIMEUNIT) -> fmod::Result>;
pub type FMOD_CODEC_SETPOSITIONCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, sub_sound: c_int, position: c_uint, postype: FMOD_TIMEUNIT) -> fmod::Result>;
pub type FMOD_CODEC_GETPOSITIONCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, position: *mut c_uint, postype: FMOD_TIMEUNIT) -> fmod::Result>;
pub type FMOD_CODEC_SOUNDCREATECALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, sub_sound: c_int, sound: *mut FMOD_SOUND) -> fmod::Result>;
pub type FMOD_CODEC_METADATACALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, tag_type: fmod::TagType, name: *mut c_char, data: *mut c_void,
        data_len: c_uint, data_type: fmod::TagDataType, unique: c_int) -> fmod::Result>;
pub type FMOD_CODEC_GETWAVEFORMAT = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, index: c_int, wave_format: *mut FMOD_CODEC_WAVEFORMAT) -> fmod::Result>;
pub type FMOD_3D_ROLLOFFCALLBACK = Option<extern "C" fn(channel: *mut FMOD_CHANNEL, distance: c_float) -> fmod::Result>;*/

/*  DSP callbacks */
pub type DspCreateCallback = Option<fn(dsp_state: &dsp::DspState) -> fmod::Result>;
pub type DspReleaseCallback = Option<fn(dsp_state: &dsp::DspState) -> fmod::Result>;
pub type DspResetCallback = Option<fn(dsp_state: &dsp::DspState) -> fmod::Result>;
pub type DspReadCallback = Option<fn(dsp_state: &dsp::DspState, in_buffer: &mut Vec<f32>, out_buffer: &mut Vec<f32>, length: u32, inchannels: i32, outchannels: i32) -> fmod::Result>;
pub type DspSetPositionCallback = Option<fn(dsp_state: &dsp::DspState, pos: u32) -> fmod::Result>;
pub type DspSetParamCallback = Option<fn(dsp_state: &dsp::DspState, index: i32, value: f32) -> fmod::Result>;
pub type DspGetParamCallback = Option<fn(dsp_state: &dsp::DspState, index: i32, value: &mut f32, value_str: &str) -> fmod::Result>;
pub type DspDialogCallback = Option<fn(dsp_state: dsp::DspState/*, hwnd: *mut c_void*/, show: i32) -> fmod::Result>;
