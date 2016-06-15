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

extern crate libc;

use callbacks::*;
use libc::{c_void, c_uint, c_int, c_char, c_float, c_ushort, c_uchar, c_short};

pub trait FFI<T> {
    fn wrap(r: *mut T) -> Self;
    fn unwrap(&Self) -> *mut T;
}

pub fn strlen(mut c: *const c_char) -> usize {
    let mut len = 0;

    unsafe {
        if !c.is_null() {
            while *c != 0 {
                len += 1;
                c = c.offset(1);
            }
        }
    }
    len
}

pub type FMOD_BOOL = c_int;
pub type FMOD_CAPS = c_uint;
pub type FMOD_INITFLAGS = c_uint;
pub type FMOD_MODE = c_uint;
pub type FMOD_TIMEUNIT = c_uint;

pub struct FMOD_CHANNEL;
pub struct FMOD_CHANNELGROUP;
pub struct FMOD_DSP;
pub struct FMOD_DSPCONNECTION;
pub struct FMOD_GEOMETRY;
pub struct FMOD_REVERB;
pub struct FMOD_SOUND;
pub struct FMOD_SOUNDGROUP;
pub struct FMOD_SYSTEM;
pub struct FMOD_SYNCPOINT;

pub type FMOD_SYSTEM_CALLBACK = Option<extern "C" fn(system: *mut FMOD_SYSTEM, _type: ::SystemCallbackType, command_data1: *mut c_void,
    command_data2: *mut c_void) -> ::Result>;

/* file callbacks */
pub type FMOD_FILE_OPENCALLBACK = Option<extern "C" fn(name: *mut c_char, unicode: c_int, file_size: *mut c_uint, handle: *mut *mut c_void,
    user_data: *mut *mut c_void) -> ::Result>;
pub type FMOD_FILE_CLOSECALLBACK = Option<extern "C" fn(handle: *mut c_void, user_data: *mut c_void) -> ::Result>;
pub type FMOD_FILE_READCALLBACK = Option<extern "C" fn(handle: *mut c_void, buffer: *mut c_void, size_bytes: c_uint, bytes_read: *mut c_uint,
    user_data: *mut c_void) -> ::Result>;
pub type FMOD_FILE_SEEKCALLBACK = Option<extern "C" fn(handle: *mut c_void, pos: c_uint, user_data: *mut c_void) -> ::Result>;
pub type FMOD_FILE_ASYNCREADCALLBACK = Option<extern "C" fn(arg1: *mut FMOD_ASYNCREADINFO, arg2: *mut c_void) -> ::Result>;
pub type FMOD_FILE_ASYNCCANCELCALLBACK = Option<extern "C" fn(arg1: *mut c_void, arg2: *mut c_void, arg3: c_uint) -> ::Result>;

/* sound callbacks */
pub type FMOD_SOUND_NONBLOCKCALLBACK = Option<extern "C" fn(sound: *mut FMOD_SOUND, result: ::Result) -> ::Result>;
pub type FMOD_SOUND_PCMREADCALLBACK = Option<extern "C" fn(sound: *mut FMOD_SOUND, data: *mut c_void, data_len: c_uint) -> ::Result>;
pub type FMOD_SOUND_PCMSETPOSCALLBACK = Option<extern "C" fn(sound: *mut FMOD_SOUND, sub_sound: c_int, position: c_uint,
    postype: FMOD_TIMEUNIT) -> ::Result>;

/*  codec callbacks */
pub type FMOD_CODEC_OPENCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, user_mode: FMOD_MODE, userexinfo: *mut FMOD_CREATESOUNDEXINFO) -> ::Result>;
pub type FMOD_CODEC_CLOSECALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE) -> ::Result>;
pub type FMOD_CODEC_READCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, buffer: *mut c_void, size_bytes: c_uint, bytes_read: *mut c_uint) -> ::Result>;
pub type FMOD_CODEC_GETLENGTHCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, length: *mut c_uint, length_type: FMOD_TIMEUNIT) -> ::Result>;
pub type FMOD_CODEC_SETPOSITIONCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, sub_sound: c_int, position: c_uint, postype: FMOD_TIMEUNIT) -> ::Result>;
pub type FMOD_CODEC_GETPOSITIONCALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, position: *mut c_uint, postype: FMOD_TIMEUNIT) -> ::Result>;
pub type FMOD_CODEC_SOUNDCREATECALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, sub_sound: c_int, sound: *mut FMOD_SOUND) -> ::Result>;
pub type FMOD_CODEC_METADATACALLBACK = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, tag_type: ::TagType, name: *mut c_char, data: *mut c_void,
    data_len: c_uint, data_type: ::TagDataType, unique: c_int) -> ::Result>;
pub type FMOD_CODEC_GETWAVEFORMAT = Option<extern "C" fn(codec_state: *mut FMOD_CODEC_STATE, index: c_int, wave_format: *mut FMOD_CODEC_WAVEFORMAT) -> ::Result>;
pub type FMOD_3D_ROLLOFFCALLBACK = Option<extern "C" fn(channel: *mut FMOD_CHANNEL, distance: c_float) -> ::Result>;

/*  DSP callbacks */
pub type FMOD_DSP_CREATECALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE) -> ::Result>;
pub type FMOD_DSP_RELEASECALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE) -> ::Result>;
pub type FMOD_DSP_RESETCALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE) -> ::Result>;
pub type FMOD_DSP_READCALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE, in_buffer: *mut c_float, out_buffer: *mut c_float, length: c_uint,
    in_channels: c_int, out_channels: c_int) -> ::Result>;
pub type FMOD_DSP_SETPOSITIONCALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE, pos: c_uint) -> ::Result>;
pub type FMOD_DSP_SETPARAMCALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE, index: c_int, value: c_float) -> ::Result>;
pub type FMOD_DSP_GETPARAMCALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE, index: c_int, value: *mut c_float, value_str: *mut c_char) -> ::Result>;
pub type FMOD_DSP_DIALOGCALLBACK = Option<extern "C" fn(dsp_state: *mut FMOD_DSP_STATE, hwnd: *mut c_void, show: c_int) -> ::Result>;

extern "C" {
    pub fn FMOD_System_Create(system: *mut *mut FMOD_SYSTEM) -> ::Result;
    pub fn FMOD_System_Release(system: *mut FMOD_SYSTEM) -> ::Result;
    /* pre-init functions */
    pub fn FMOD_System_SetOutput(system: *mut FMOD_SYSTEM, output_type: ::OutputType) -> ::Result;
    pub fn FMOD_System_GetOutput(system: *mut FMOD_SYSTEM, output_type: *mut ::OutputType) -> ::Result;
    pub fn FMOD_System_GetNumDrivers(system: *mut FMOD_SYSTEM, num_drivers: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetDriverInfo(system: *mut FMOD_SYSTEM, id: c_int, name: *mut c_char, name_len: c_int, guid: *mut FMOD_GUID) -> ::Result;
    pub fn FMOD_System_GetDriverInfoW(system: *mut FMOD_SYSTEM, id: c_int, name: *mut c_short, name_len: c_int, guid: *mut FMOD_GUID) -> ::Result;
    pub fn FMOD_System_GetDriverCaps(system: *mut FMOD_SYSTEM, id: c_int, caps: *mut FMOD_CAPS, control_panel_output_rate: *mut c_int,
        controlpanelspeakermode: *mut ::SpeakerMode) -> ::Result;
    pub fn FMOD_System_SetDriver(system: *mut FMOD_SYSTEM, driver: c_int) -> ::Result;
    pub fn FMOD_System_GetDriver(system: *mut FMOD_SYSTEM, driver: *mut c_int) -> ::Result;
    pub fn FMOD_System_SetHardwareChannels(system: *mut FMOD_SYSTEM, num_hardware_channels: c_int) -> ::Result;
    pub fn FMOD_System_GetHardwareChannels(system: *mut FMOD_SYSTEM, num_hardware_channels: *mut c_int) -> ::Result;
    pub fn FMOD_System_SetSoftwareChannels(system: *mut FMOD_SYSTEM, num_software_channels: c_int) -> ::Result;
    pub fn FMOD_System_GetSoftwareChannels(system: *mut FMOD_SYSTEM, num_software_channels: *mut c_int) -> ::Result;
    pub fn FMOD_System_SetSoftwareFormat(system: *mut FMOD_SYSTEM, sample_rate: c_int, format: ::SoundFormat, num_output_channels: c_int,
        max_input_channels: c_int, resample_method: ::DspResampler) -> ::Result;
    pub fn FMOD_System_GetSoftwareFormat(system: *mut FMOD_SYSTEM, sample_rate: *mut c_int, format: *mut ::SoundFormat,
        num_output_channels: *mut c_int, max_input_channels: *mut c_int, resample_method: *mut ::DspResampler, bits: *mut c_int) -> ::Result;
    pub fn FMOD_System_SetDSPBufferSize(system: *mut FMOD_SYSTEM, buffer_length: c_uint, num_buffers: c_int) -> ::Result;
    pub fn FMOD_System_GetDSPBufferSize(system: *mut FMOD_SYSTEM, buffer_length: *mut c_uint, num_buffers: *mut c_int) -> ::Result;
    // I'll bind it a little later
    pub fn FMOD_System_SetFileSystem(system: *mut FMOD_SYSTEM, user_open: FMOD_FILE_OPENCALLBACK, user_close: FMOD_FILE_CLOSECALLBACK,
        user_read: FMOD_FILE_READCALLBACK, user_seek: FMOD_FILE_SEEKCALLBACK, user_async_read: FMOD_FILE_ASYNCREADCALLBACK,
        user_async_cancel: FMOD_FILE_ASYNCCANCELCALLBACK, block_align: c_int) -> ::Result;
    // I'll bind it a little later
    pub fn FMOD_System_AttachFileSystem(system: *mut FMOD_SYSTEM, user_open: FMOD_FILE_OPENCALLBACK, user_close: FMOD_FILE_CLOSECALLBACK,
        user_read: FMOD_FILE_READCALLBACK, user_seek: FMOD_FILE_SEEKCALLBACK) -> ::Result;
    pub fn FMOD_System_SetAdvancedSettings(system: *mut FMOD_SYSTEM, settings: *mut FMOD_ADVANCEDSETTINGS) -> ::Result;
    pub fn FMOD_System_GetAdvancedSettings(system: *mut FMOD_SYSTEM, settings: *mut FMOD_ADVANCEDSETTINGS) -> ::Result;
    pub fn FMOD_System_SetSpeakerMode(system: *mut FMOD_SYSTEM, speaker_mode: ::SpeakerMode) -> ::Result;
    pub fn FMOD_System_GetSpeakerMode(system: *mut FMOD_SYSTEM, speaker_mode: *mut ::SpeakerMode) -> ::Result;
    // I'll bind it a little later
    pub fn FMOD_System_SetCallback(system: *mut FMOD_SYSTEM, call_back: FMOD_SYSTEM_CALLBACK) -> ::Result;
    /* plug-in part functions */
    pub fn FMOD_System_SetPluginPath(system: *mut FMOD_SYSTEM, path: *const c_char) -> ::Result;
    pub fn FMOD_System_LoadPlugin(system: *mut FMOD_SYSTEM, filename: *const c_char, handle: *mut c_uint, priority: c_uint) -> ::Result;
    pub fn FMOD_System_UnloadPlugin(system: *mut FMOD_SYSTEM, handle: c_uint) -> ::Result;
    pub fn FMOD_System_GetNumPlugins(system: *mut FMOD_SYSTEM, plugin_type: ::PluginType, num_plugins: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetPluginHandle(system: *mut FMOD_SYSTEM, plugin_type: ::PluginType, index: c_int, handle: *mut c_uint) -> ::Result;
    pub fn FMOD_System_GetPluginInfo(system: *mut FMOD_SYSTEM, handle: c_uint, plugin_type: *mut ::PluginType, name: *mut c_char,
        name_len: c_int, version: *mut c_uint) -> ::Result;
    pub fn FMOD_System_SetOutputByPlugin(system: *mut FMOD_SYSTEM, handle: c_uint) -> ::Result;
    pub fn FMOD_System_GetOutputByPlugin(system: *mut FMOD_SYSTEM, handle: *mut c_uint) -> ::Result;
    pub fn FMOD_System_CreateDSPByPlugin(system: *mut FMOD_SYSTEM, handle: c_uint, dsp: *mut *mut FMOD_DSP) -> ::Result;
    /* codec part functions */
    pub fn FMOD_System_RegisterCodec(system: *mut FMOD_SYSTEM, description: *mut FMOD_CODEC_DESCRIPTION, handle: *mut c_uint, priority: c_uint) -> ::Result;
    /* init/close functions */
    pub fn FMOD_System_Init(system: *mut FMOD_SYSTEM, max_channels: c_int, flags: FMOD_INITFLAGS, extra_driver_data: *mut c_void) -> ::Result;
    pub fn FMOD_System_Close(sound: *mut FMOD_SYSTEM) -> ::Result;
    /* post-init functions */
    pub fn FMOD_System_Update(system: *mut FMOD_SYSTEM) -> ::Result;
    pub fn FMOD_System_GetSpectrum(system: *mut FMOD_SYSTEM, spectrum_array: *mut c_float, num_values: c_int, channel_offset: c_int,
        window_type: ::DspFftWindow) -> ::Result;
    pub fn FMOD_System_GetWaveData(system: *mut FMOD_SYSTEM, wave_array: *mut c_float, num_values: c_int, channel_offset: c_int) -> ::Result;
    pub fn FMOD_System_SetStreamBufferSize(system: *mut FMOD_SYSTEM, file_buffer_size: c_uint, file_buffer_size_type: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_System_GetStreamBufferSize(system: *mut FMOD_SYSTEM, file_buffer_size: *mut c_uint, file_buffer_size_type: *mut FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_System_Set3DNumListeners(system: *mut FMOD_SYSTEM, num_listeners: c_int) -> ::Result;
    pub fn FMOD_System_Get3DNumListeners(system: *mut FMOD_SYSTEM, num_listeners: *mut c_int) -> ::Result;
    pub fn FMOD_System_Set3DListenerAttributes(system: *mut FMOD_SYSTEM, listener: c_int, pos: *const FMOD_VECTOR, vel: *const FMOD_VECTOR, forward: *const FMOD_VECTOR,
        up: *const FMOD_VECTOR) -> ::Result;
    pub fn FMOD_System_Get3DListenerAttributes(system: *mut FMOD_SYSTEM, listener: c_int, pos: *mut FMOD_VECTOR, vel: *mut FMOD_VECTOR, forward: *mut FMOD_VECTOR,
        up: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_System_GetMemoryInfo(system: *mut FMOD_SYSTEM, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memoryused_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;
    /* I'll bind it later */
    pub fn FMOD_System_SetFMOD_3D_ROLLOFFCALLBACK(system: *mut FMOD_SYSTEM, callback: FMOD_3D_ROLLOFFCALLBACK) -> ::Result;
    pub fn FMOD_System_Set3DSpeakerPosition(system: *mut FMOD_SYSTEM, speaker: ::Speaker, x: c_float, y: c_float, active: FMOD_BOOL) -> ::Result;
    pub fn FMOD_System_Get3DSpeakerPosition(system: *mut FMOD_SYSTEM, speaker: ::Speaker, x: *mut c_float, y: *mut c_float, active: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_System_Set3DSettings(system: *mut FMOD_SYSTEM, doppler_scale: c_float, distance_factor: c_float, roll_off_scale: c_float) -> ::Result;
    pub fn FMOD_System_Get3DSettings(system: *mut FMOD_SYSTEM, doppler_scale: *mut c_float, distance_factor: *mut c_float, roll_off_scale: *mut c_float) -> ::Result;
    /* system information functions */
    pub fn FMOD_System_GetVersion(system: *mut FMOD_SYSTEM, version: *mut c_uint) -> ::Result;
    pub fn FMOD_System_GetOutputHandle(system: *mut FMOD_SYSTEM, handle: *mut *mut c_void) -> ::Result;
    pub fn FMOD_System_GetChannelsPlaying(system: *mut FMOD_SYSTEM, channels: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetCPUUsage(system: *mut FMOD_SYSTEM, dsp: *mut c_float, stream: *mut c_float, geometry: *mut c_float, update: *mut c_float, total: *mut c_float) -> ::Result;
    pub fn FMOD_System_GetSoundRAM(system: *mut FMOD_SYSTEM, current_alloced: *mut c_int, max_alloced: *mut c_int, total: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetNumCDROMDrives(system: *mut FMOD_SYSTEM, num_drives: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetCDROMDriveName(system: *mut FMOD_SYSTEM, drive: c_int, drive_name: *mut c_char, drive_name_len: c_int, scsi_name: *mut c_char,
        scsi_name_len: c_int, device_name: *mut c_char, device_name_len: c_int) -> ::Result;
    /* Sound/DSP/Channel/FX creation and retrieval. */
    pub fn FMOD_System_CreateSound(system: *mut FMOD_SYSTEM, name_or_data: *const c_char, mode: FMOD_MODE, exinfo: *mut FMOD_CREATESOUNDEXINFO,
        sound: *mut *mut FMOD_SOUND) -> ::Result;
    pub fn FMOD_System_CreateStream(system: *mut FMOD_SYSTEM, name_or_data: *const c_char, mode: FMOD_MODE, exinfo: *mut FMOD_CREATESOUNDEXINFO,
        sound: *mut *mut FMOD_SOUND) -> ::Result;
    pub fn FMOD_System_CreateReverb(system: *mut FMOD_SYSTEM, reverb: *mut *mut FMOD_REVERB) -> ::Result;
    pub fn FMOD_System_CreateDSP(system: *mut FMOD_SYSTEM, description: *mut FMOD_DSP_DESCRIPTION, dsp: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_System_CreateDSPByType(system: *mut FMOD_SYSTEM, _type: ::DspType, dsp: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_System_CreateChannelGroup(system: *mut FMOD_SYSTEM, name: *const c_char, channel_group: *mut *mut FMOD_CHANNELGROUP) -> ::Result;
    pub fn FMOD_System_CreateSoundGroup(system: *mut FMOD_SYSTEM, name: *const c_char, sound_group: *mut *mut FMOD_SOUNDGROUP) -> ::Result;
    pub fn FMOD_System_GetChannel(system: *mut FMOD_SYSTEM, channel_id: c_int, channel: *mut *mut FMOD_CHANNEL) -> ::Result;
    pub fn FMOD_System_GetMasterChannelGroup(system: *mut FMOD_SYSTEM, channel_group: *mut *mut FMOD_CHANNELGROUP) -> ::Result;
    pub fn FMOD_System_GetMasterSoundGroup(system: *mut FMOD_SYSTEM, sound_group: *mut *mut FMOD_SOUNDGROUP) -> ::Result;
    /* Reverb API */
    pub fn FMOD_System_SetReverbProperties(system: *mut FMOD_SYSTEM, prop: *const FMOD_REVERB_PROPERTIES) -> ::Result;
    pub fn FMOD_System_GetReverbProperties(system: *mut FMOD_SYSTEM, prop: *mut FMOD_REVERB_PROPERTIES) -> ::Result;
    pub fn FMOD_System_SetReverbAmbientProperties(system: *mut FMOD_SYSTEM, prop: *mut FMOD_REVERB_PROPERTIES) -> ::Result;
    pub fn FMOD_System_GetReverbAmbientProperties(system: *mut FMOD_SYSTEM, prop: *mut FMOD_REVERB_PROPERTIES) -> ::Result;
    /* System level DSP access.*/
    pub fn FMOD_System_GetDSPHead(system: *mut FMOD_SYSTEM, dsp: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_System_AddDSP(system: *mut FMOD_SYSTEM, dsp: *mut FMOD_DSP, connection: *mut *mut FMOD_DSPCONNECTION) -> ::Result;
    pub fn FMOD_System_LockDSP(system: *mut FMOD_SYSTEM) -> ::Result;
    pub fn FMOD_System_UnlockDSP(system: *mut FMOD_SYSTEM) -> ::Result;
    pub fn FMOD_System_GetDSPClock(system: *mut FMOD_SYSTEM, hi: *mut c_uint, lo: *mut c_uint) -> ::Result;
    /* Recording API */
    pub fn FMOD_System_GetRecordNumDrivers(system: *mut FMOD_SYSTEM, num_drivers: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetRecordDriverInfo(system: *mut FMOD_SYSTEM, id: c_int, name: *mut c_char, name_len: c_int, guid: *mut FMOD_GUID) -> ::Result;
    /* I'll bind it later */
    pub fn FMOD_System_GetRecordDriverInfoW(system: *mut FMOD_SYSTEM, id: c_int, name: *mut c_short, name_len: c_int, guid: *mut FMOD_GUID) -> ::Result;
    pub fn FMOD_System_GetRecordDriverCaps(system: *mut FMOD_SYSTEM, id: c_int, caps: *mut FMOD_CAPS, min_frequency: *mut c_int, max_frequency: *mut c_int) -> ::Result;
    pub fn FMOD_System_GetRecordPosition(system: *mut FMOD_SYSTEM, id: c_int, position: *mut c_uint) -> ::Result;
    pub fn FMOD_System_RecordStart(system: *mut FMOD_SYSTEM, id: c_int, sound: *mut FMOD_SOUND, _loop: FMOD_BOOL) -> ::Result;
    pub fn FMOD_System_RecordStop(system: *mut FMOD_SYSTEM, id: c_int) -> ::Result;
    pub fn FMOD_System_IsRecording(system: *mut FMOD_SYSTEM, id: c_int, recording: *mut FMOD_BOOL) -> ::Result;
    /* Geometry API. */
    pub fn FMOD_System_CreateGeometry(system: *mut FMOD_SYSTEM, max_polygons: c_int, max_vertices: c_int, geometry: *mut *mut FMOD_GEOMETRY) -> ::Result;
    pub fn FMOD_System_SetGeometrySettings(system: *mut FMOD_SYSTEM, max_world_size: c_float) -> ::Result;
    pub fn FMOD_System_GetGeometrySettings(system: *mut FMOD_SYSTEM, max_world_size: *mut c_float) -> ::Result;
    /* I'll bind it later */
    pub fn FMOD_System_LoadGeometry(system: *mut FMOD_SYSTEM, data: *mut c_void, data_size: c_int, geometry: *mut *mut FMOD_GEOMETRY) -> ::Result;
    pub fn FMOD_System_GetGeometryOcclusion(system: *mut FMOD_SYSTEM, listener: *const FMOD_VECTOR, source: *const FMOD_VECTOR, direct: *mut c_float,
        reverb: *mut c_float) -> ::Result;
    /* Network functions.*/
    /* to add */

    /* sound functions */
    pub fn FMOD_System_PlaySound(system: *mut FMOD_SYSTEM, channel_id: ::ChannelIndex, sound: *mut FMOD_SOUND, paused: FMOD_BOOL,
        channel: *mut *mut FMOD_CHANNEL) -> ::Result;
    pub fn FMOD_Sound_Release(sound: *mut FMOD_SOUND) -> ::Result;
    /* Standard sound manipulation functions. */
    pub fn FMOD_Sound_Lock(sound: *mut FMOD_SOUND, offset: c_uint, length: c_uint, ptr1: *mut *mut c_void, ptr2: *mut *mut c_void, len1: *mut c_uint, len2: *mut c_uint) -> ::Result;
    pub fn FMOD_Sound_Unlock(sound: *mut FMOD_SOUND, ptr1: *mut c_void, ptr2: *mut c_void, len1: c_uint, len2: c_uint) -> ::Result;
    pub fn FMOD_Sound_GetSystemObject(sound: *mut FMOD_SOUND, system: *mut *mut FMOD_SYSTEM) -> ::Result;
    pub fn FMOD_Sound_SetDefaults(sound: *mut FMOD_SOUND, frequency: c_float, volume: c_float, pan: c_float, priority: c_int) -> ::Result;
    pub fn FMOD_Sound_GetDefaults(sound: *mut FMOD_SOUND, frequency: *mut c_float, volume: *mut c_float, pan: *mut c_float, priority: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_SetVariations(sound: *mut FMOD_SOUND, frequency_var: c_float, volume_var: c_float, pan_var: c_float) -> ::Result;
    pub fn FMOD_Sound_GetVariations(sound: *mut FMOD_SOUND, frequency_var: *mut c_float, volume_var: *mut c_float, pan_var: *mut c_float) -> ::Result;
    pub fn FMOD_Sound_Set3DMinMaxDistance(sound: *mut FMOD_SOUND, min: c_float, max: c_float) -> ::Result;
    pub fn FMOD_Sound_Get3DMinMaxDistance(sound: *mut FMOD_SOUND, min: *mut c_float, max: *mut c_float) -> ::Result;
    pub fn FMOD_Sound_Set3DConeSettings(sound: *mut FMOD_SOUND, inside_cone_angle: c_float, outside_cone_angle: c_float, outside_volume: c_float) -> ::Result;
    pub fn FMOD_Sound_Get3DConeSettings(sound: *mut FMOD_SOUND, inside_cone_angle: *mut c_float, outside_cone_angle: *mut c_float,
        outside_volume: *mut c_float) -> ::Result;
    pub fn FMOD_Sound_Set3DCustomRolloff(sound: *mut FMOD_SOUND, points: *mut FMOD_VECTOR, num_points: c_int) -> ::Result;
    pub fn FMOD_Sound_Get3DCustomRolloff(sound: *mut FMOD_SOUND, points: *mut *mut FMOD_VECTOR, num_points: c_int) -> ::Result;
    pub fn FMOD_Sound_SetSubSound(sound: *mut FMOD_SOUND, index: c_int, sub_sound: *mut FMOD_SOUND) -> ::Result;
    pub fn FMOD_Sound_GetSubSound(sound: *mut FMOD_SOUND, index: c_int, sub_sound: *mut *mut FMOD_SOUND) -> ::Result;
    pub fn FMOD_Sound_SetSubSoundSentence(sound: *mut FMOD_SOUND, sub_sound_list: *mut c_int, num_sub_sound: c_int) -> ::Result;
    pub fn FMOD_Sound_GetName(sound: *mut FMOD_SOUND, name: *mut c_char, name_len: c_int) -> ::Result;
    pub fn FMOD_Sound_GetLength(sound: *mut FMOD_SOUND, length: *mut c_uint, length_type: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_Sound_GetFormat(sound: *mut FMOD_SOUND, _type: *mut ::SoundType, format: *mut ::SoundFormat, channels: *mut c_int,
        bits: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_GetNumSubSounds(sound: *mut FMOD_SOUND, num_sub_sound: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_GetNumTags(sound: *mut FMOD_SOUND, num_tags: *mut c_int, num_tags_updated: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_GetTag(sound: *mut FMOD_SOUND, name: *const c_char, index: c_int, tag: *mut FMOD_TAG) -> ::Result;
    pub fn FMOD_Sound_GetOpenState(sound: *mut FMOD_SOUND, open_state: *mut ::OpenState, percent_buffered: *mut c_uint, starving: *mut FMOD_BOOL,
        disk_busy: *mut FMOD_BOOL) -> ::Result;
    /* I'll bind it later */
    pub fn FMOD_Sound_ReadData(sound: *mut FMOD_SOUND, buffer: *mut c_void, len_bytes: c_uint, read: *mut c_uint) -> ::Result;
    pub fn FMOD_Sound_SeekData(sound: *mut FMOD_SOUND, pcm: c_uint) -> ::Result;
    pub fn FMOD_Sound_SetSoundGroup(sound: *mut FMOD_SOUND, sound_group: *mut FMOD_SOUNDGROUP) -> ::Result;
    pub fn FMOD_Sound_GetSoundGroup(sound: *mut FMOD_SOUND, sound_group: *mut *mut FMOD_SOUNDGROUP) -> ::Result;
    /* Synchronization point API. These points can come from markers embedded in wav files, and can also generate channel callbacks. */
    pub fn FMOD_Sound_GetNumSyncPoints(sound: *mut FMOD_SOUND, num_sync_points: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_GetSyncPoint(sound: *mut FMOD_SOUND, index: c_int, point: *mut *mut FMOD_SYNCPOINT) -> ::Result;
    pub fn FMOD_Sound_GetSyncPointInfo(sound: *mut FMOD_SOUND, point: *mut FMOD_SYNCPOINT, name: *mut c_char, name_len: c_int, offset: *mut c_uint,
        offset_type: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_Sound_AddSyncPoint(sound: *mut FMOD_SOUND, offset: c_uint, offset_type: FMOD_TIMEUNIT, name: *const c_char, point: *mut *mut FMOD_SYNCPOINT) -> ::Result;
    pub fn FMOD_Sound_DeleteSyncPoint(sound: *mut FMOD_SOUND, point: *mut FMOD_SYNCPOINT) -> ::Result;
    /* Functions also in Channel class but here they are the 'default' to save having to change it in Channel all the time. */
    pub fn FMOD_Sound_SetMode(sound: *mut FMOD_SOUND, mode: FMOD_MODE) -> ::Result;
    pub fn FMOD_Sound_GetMode(sound: *mut FMOD_SOUND, mode: *mut FMOD_MODE) -> ::Result;
    pub fn FMOD_Sound_SetLoopCount(sound: *mut FMOD_SOUND, loop_count: c_int) -> ::Result;
    pub fn FMOD_Sound_GetLoopCount(sound: *mut FMOD_SOUND, loop_count: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_SetLoopPoints(sound: *mut FMOD_SOUND, loop_start: c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: c_uint,
        loop_end_type: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_Sound_GetLoopPoints(sound: *mut FMOD_SOUND, loop_start: *mut c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: *mut c_uint,
        loop_end_type: FMOD_TIMEUNIT) -> ::Result;
    /* For MOD/S3M/XM/IT/MID sequenced formats only. */
    pub fn FMOD_Sound_GetMusicNumChannels(sound: *mut FMOD_SOUND, num_channels: *mut c_int) -> ::Result;
    pub fn FMOD_Sound_SetMusicChannelVolume(sound: *mut FMOD_SOUND, channel: c_int, volume: c_float) -> ::Result;
    pub fn FMOD_Sound_GetMusicChannelVolume(sound: *mut FMOD_SOUND, channel: c_int, volume: *mut c_float) -> ::Result;
    pub fn FMOD_Sound_SetMusicSpeed(sound: *mut FMOD_SOUND, speed: c_float) -> ::Result;
    pub fn FMOD_Sound_GetMusicSpeed(sound: *mut FMOD_SOUND, speed: *mut c_float) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_Sound_SetUserData(sound: *mut FMOD_SOUND, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_Sound_GetUserData(sound: *mut FMOD_SOUND, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_Sound_GetMemoryInfo(sound: *mut FMOD_SOUND, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memory_used_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;


    /* channel functions */
    pub fn FMOD_Channel_GetSystemObject(channel: *mut FMOD_CHANNEL, system: *mut *mut FMOD_SYSTEM) -> ::Result;
    pub fn FMOD_Channel_Stop(channel: *mut FMOD_CHANNEL) -> ::Result;
    pub fn FMOD_Channel_SetPaused(channel: *mut FMOD_CHANNEL, pause: FMOD_BOOL) -> ::Result;
    pub fn FMOD_Channel_GetPaused(channel: *mut FMOD_CHANNEL, pause: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_Channel_SetVolume(channel: *mut FMOD_CHANNEL, volume: c_float) -> ::Result;
    pub fn FMOD_Channel_GetVolume(channel: *mut FMOD_CHANNEL, volume: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_SetFrequency(channel: *mut FMOD_CHANNEL, frequency: c_float) -> ::Result;
    pub fn FMOD_Channel_GetFrequency(channel: *mut FMOD_CHANNEL, frequency: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_SetPan(channel: *mut FMOD_CHANNEL, pan: c_float) -> ::Result;
    pub fn FMOD_Channel_GetPan(channel: *mut FMOD_CHANNEL, pan: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_SetDelay(channel: *mut FMOD_CHANNEL, delay_type: ::DelayType, delayhi: c_uint, delaylo: c_uint) -> ::Result;
    pub fn FMOD_Channel_GetDelay(channel: *mut FMOD_CHANNEL, delay_type: ::DelayType, delayhi: *mut c_uint, delaylo: *mut c_uint) -> ::Result;
    pub fn FMOD_Channel_SetSpeakerMix(channel: *mut FMOD_CHANNEL, front_left: c_float, front_right: c_float, center: c_float, lfe: c_float,
        back_left: c_float, back_right: c_float, side_left: c_float, side_right: c_float) -> ::Result;
    pub fn FMOD_Channel_GetSpeakerMix(channel: *mut FMOD_CHANNEL, front_left: *mut c_float, front_right: *mut c_float, center: *mut c_float, lfe: *mut c_float,
        back_left: *mut c_float, back_right: *mut c_float, side_left: *mut c_float, side_right: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_SetSpeakerLevels(channel: *mut FMOD_CHANNEL, speaker: ::Speaker, levels: *mut c_float, num_levels: c_int) -> ::Result;
    pub fn FMOD_Channel_GetSpeakerLevels(channel: *mut FMOD_CHANNEL, speaker: ::Speaker, levels: *mut c_float, num_levels: c_int) -> ::Result;
    pub fn FMOD_Channel_SetInputChannelMix(channel: *mut FMOD_CHANNEL, levels: *mut c_float, num_levels: c_int) -> ::Result;
    pub fn FMOD_Channel_GetInputChannelMix(channel: *mut FMOD_CHANNEL, levels: *mut c_float, num_levels: c_int) -> ::Result;
    pub fn FMOD_Channel_SetMute(channel: *mut FMOD_CHANNEL, mute: FMOD_BOOL) -> ::Result;
    pub fn FMOD_Channel_GetMute(channel: *mut FMOD_CHANNEL, mute: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_Channel_SetPriority(channel: *mut FMOD_CHANNEL, priority: c_int) -> ::Result;
    pub fn FMOD_Channel_GetPriority(channel: *mut FMOD_CHANNEL, priority: *mut c_int) -> ::Result;
    pub fn FMOD_Channel_SetPosition(channel: *mut FMOD_CHANNEL, position: c_uint, postype: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_Channel_GetPosition(channel: *mut FMOD_CHANNEL, position: *mut c_uint, postype: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_Channel_SetReverbProperties(channel: *mut FMOD_CHANNEL, prop: *const FMOD_REVERB_CHANNELPROPERTIES) -> ::Result;
    pub fn FMOD_Channel_GetReverbProperties(channel: *mut FMOD_CHANNEL, prop: *mut FMOD_REVERB_CHANNELPROPERTIES) -> ::Result;
    pub fn FMOD_Channel_SetLowPassGain(channel: *mut FMOD_CHANNEL, gain: c_float) -> ::Result;
    pub fn FMOD_Channel_GetLowPassGain(channel: *mut FMOD_CHANNEL, gain: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_SetChannelGroup(channel: *mut FMOD_CHANNEL, channelgroup: *mut FMOD_CHANNELGROUP) -> ::Result;
    pub fn FMOD_Channel_GetChannelGroup(channel: *mut FMOD_CHANNEL, channelgroup: *mut *mut FMOD_CHANNELGROUP) -> ::Result;
    /* I'll bind it later */
    //pub fn FMOD_Channel_SetCallback(channel: *mut FMOD_CHANNEL, callback: *mut FMOD_Channel_CALLBACK) -> ::Result;
    /* 3D functionality */
    pub fn FMOD_Channel_Set3DAttributes(channel: *mut FMOD_CHANNEL, position: *mut FMOD_VECTOR, velociy: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Channel_Get3DAttributes(channel: *mut FMOD_CHANNEL, position: *mut FMOD_VECTOR, velociy: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Channel_Set3DMinMaxDistance(channel: *mut FMOD_CHANNEL, min_distance: c_float, max_distance: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DMinMaxDistance(channel: *mut FMOD_CHANNEL, min_distance: *mut c_float, max_distance: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_Set3DConeSettings(channel: *mut FMOD_CHANNEL, inside_cone_angle: c_float, outside_cone_angle: c_float,
        outside_volume: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DConeSettings(channel: *mut FMOD_CHANNEL, inside_cone_angle: *mut c_float, outside_cone_angle: *mut c_float,
        outside_volume: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_Set3DConeOrientation(channel: *mut FMOD_CHANNEL, orientation: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Channel_Get3DConeOrientation(channel: *mut FMOD_CHANNEL, orientation: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Channel_Set3DCustomRolloff(channel: *mut FMOD_CHANNEL, points: *mut FMOD_VECTOR, num_points: c_int) -> ::Result;
    pub fn FMOD_Channel_Get3DCustomRolloff(channel: *mut FMOD_CHANNEL, points: *mut *mut FMOD_VECTOR, num_points: *mut c_int) -> ::Result;
    pub fn FMOD_Channel_Set3DOcclusion(channel: *mut FMOD_CHANNEL, direct_occlusion: c_float, reverb_occlusion: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DOcclusion(channel: *mut FMOD_CHANNEL, direct_occlusion: *mut c_float, reverb_occlusion: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_Set3DSpread(channel: *mut FMOD_CHANNEL, angle: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DSpread(channel: *mut FMOD_CHANNEL, angle: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_Set3DPanLevel(channel: *mut FMOD_CHANNEL, level: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DPanLevel(channel: *mut FMOD_CHANNEL, level: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_Set3DDopplerLevel(channel: *mut FMOD_CHANNEL, level: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DDopplerLevel(channel: *mut FMOD_CHANNEL, level: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_Set3DDistanceFilter(channel: *mut FMOD_CHANNEL, custom: FMOD_BOOL, custom_level: c_float, center_freq: c_float) -> ::Result;
    pub fn FMOD_Channel_Get3DDistanceFilter(channel: *mut FMOD_CHANNEL, custom: *mut FMOD_BOOL, custom_level: *mut c_float, center_freq: *mut c_float) -> ::Result;
    /* Information only functions */
    pub fn FMOD_Channel_IsPlaying(channel: *mut FMOD_CHANNEL, is_playing: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_Channel_IsVirtual(channel: *mut FMOD_CHANNEL, is_virtual: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_Channel_GetAudibility(channel: *mut FMOD_CHANNEL, audibility: *mut c_float) -> ::Result;
    pub fn FMOD_Channel_GetCurrentSound(channel: *mut FMOD_CHANNEL, sound: *mut *mut FMOD_SOUND) -> ::Result;
    pub fn FMOD_Channel_GetSpectrum(channel: *mut FMOD_CHANNEL, spectrum_array: *mut c_float, num_values: c_int, channel_offset: c_int,
        window_type: ::DspFftWindow) -> ::Result;
    pub fn FMOD_Channel_GetWaveData(channel: *mut FMOD_CHANNEL, wave_array: *mut c_float, num_values: c_int, channel_offset: c_int) -> ::Result;
    pub fn FMOD_Channel_GetIndex(channel: *mut FMOD_CHANNEL, index: *mut c_int) -> ::Result;
    /* DSP functionality only for channels playing sounds created with FMOD_SOFTWARE */
    pub fn FMOD_Channel_GetDSPHead(channel: *mut FMOD_CHANNEL, dsp: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_Channel_AddDSP(channel: *mut FMOD_CHANNEL, dsp: *mut FMOD_DSP, connection: *mut *mut FMOD_DSPCONNECTION) -> ::Result;
    //-> /* Functions also found in Sound class but here they can be set per channel */
    pub fn FMOD_Channel_SetMode(channel: *mut FMOD_CHANNEL, mode: FMOD_MODE) -> ::Result;
    pub fn FMOD_Channel_GetMode(channel: *mut FMOD_CHANNEL, mode: *mut FMOD_MODE) -> ::Result;
    pub fn FMOD_Channel_SetLoopCount(channel: *mut FMOD_CHANNEL, loop_count: c_int) -> ::Result;
    pub fn FMOD_Channel_GetLoopCount(channel: *mut FMOD_CHANNEL, loop_count: *mut c_int) -> ::Result;
    pub fn FMOD_Channel_SetLoopPoints(channel: *mut FMOD_CHANNEL, loop_start: c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: c_uint,
        loop_end_type: FMOD_TIMEUNIT) -> ::Result;
    pub fn FMOD_Channel_GetLoopPoints(channel: *mut FMOD_CHANNEL, loop_start: *mut c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: *mut c_uint,
        loop_end_type: FMOD_TIMEUNIT) -> ::Result;
    /* Userdata set/get */
    pub fn FMOD_Channel_SetUserData(channel: *mut FMOD_CHANNEL, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_Channel_GetUserData(channel: *mut FMOD_CHANNEL, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_Channel_GetMemoryInfo(channel: *mut FMOD_CHANNEL, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memoryused_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;

    
    /* channel_group functions */
    pub fn FMOD_ChannelGroup_Release(channel_group: *mut FMOD_CHANNELGROUP) -> ::Result;
    /* Channelgroup scale values. (changes attributes relative to the channels, doesn't overwrite them)*/
    pub fn FMOD_ChannelGroup_SetVolume(channel_group: *mut FMOD_CHANNELGROUP, volume: c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_GetVolume(channel_group: *mut FMOD_CHANNELGROUP, volume: *mut c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_SetPitch(channel_group: *mut FMOD_CHANNELGROUP, pitch: c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_GetPitch(channel_group: *mut FMOD_CHANNELGROUP, pitch: *mut c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_Set3DOcclusion(channel_group: *mut FMOD_CHANNELGROUP, direct_occlusion: c_float, reverb_occlusion: c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_Get3DOcclusion(channel_group: *mut FMOD_CHANNELGROUP, direct_occlusion: *mut c_float, reverb_occlusion: *mut c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_SetPaused(channel_group: *mut FMOD_CHANNELGROUP, paused: FMOD_BOOL) -> ::Result;
    pub fn FMOD_ChannelGroup_GetPaused(channel_group: *mut FMOD_CHANNELGROUP, paused: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_ChannelGroup_SetMute(channel_group: *mut FMOD_CHANNELGROUP, mute: FMOD_BOOL) -> ::Result;
    pub fn FMOD_ChannelGroup_GetMute(channel_group: *mut FMOD_CHANNELGROUP, mute: *mut FMOD_BOOL) -> ::Result;
    /* Channelgroup override values. (recursively overwrites whatever settings the channels had) */
    pub fn FMOD_ChannelGroup_Stop(channel_group: *mut FMOD_CHANNELGROUP) -> ::Result;
    pub fn FMOD_ChannelGroup_OverrideVolume(channel_group: *mut FMOD_CHANNELGROUP, volume: c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_OverrideFrequency(channel_group: *mut FMOD_CHANNELGROUP, frequency: c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_OverridePan(channel_group: *mut FMOD_CHANNELGROUP, pan: c_float) -> ::Result;
    pub fn FMOD_ChannelGroup_OverrideReverbProperties(channel_group: *mut FMOD_CHANNELGROUP, prop: *const FMOD_REVERB_CHANNELPROPERTIES) -> ::Result;
    pub fn FMOD_ChannelGroup_Override3DAttributes(channel_group: *mut FMOD_CHANNELGROUP, pos: *mut FMOD_VECTOR, vel: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_ChannelGroup_OverrideSpeakerMix(channel_group: *mut FMOD_CHANNELGROUP, front_left: c_float, front_right: c_float, center: c_float, lfe: c_float,
        back_left: c_float, back_right: c_float, side_left: c_float, side_right: c_float) -> ::Result;
    /* Nested channel groups.*/
    pub fn FMOD_ChannelGroup_AddGroup(channel_group: *mut FMOD_CHANNELGROUP, group: *mut FMOD_CHANNELGROUP) -> ::Result;
    pub fn FMOD_ChannelGroup_GetNumGroups(channel_group: *mut FMOD_CHANNELGROUP, num_groups: *mut c_int) -> ::Result;
    pub fn FMOD_ChannelGroup_GetGroup(channel_group: *mut FMOD_CHANNELGROUP, index: c_int, group: *mut *mut FMOD_CHANNELGROUP) -> ::Result;
    pub fn FMOD_ChannelGroup_GetParentGroup(channel_group: *mut FMOD_CHANNELGROUP, group: *mut *mut FMOD_CHANNELGROUP) -> ::Result;
    /* DSP functionality only for channel groups playing sounds created with FMOD_SOFTWARE. */
    pub fn FMOD_ChannelGroup_GetDSPHead(channel_group: *mut FMOD_CHANNELGROUP, dsp: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_ChannelGroup_AddDSP(channel_group: *mut FMOD_CHANNELGROUP, dsp: *mut FMOD_DSP, disp_connection: *mut *mut FMOD_DSPCONNECTION) -> ::Result;
    /* Information only functions. */
    pub fn FMOD_ChannelGroup_GetName(channel_group: *mut FMOD_CHANNELGROUP, name: *mut c_char, name_len: c_int) -> ::Result;
    pub fn FMOD_ChannelGroup_GetNumChannels(channel_group: *mut FMOD_CHANNELGROUP, num_channels: *mut c_int) -> ::Result;
    pub fn FMOD_ChannelGroup_GetChannel(channel_group: *mut FMOD_CHANNELGROUP, index: c_int, channel: *mut *mut FMOD_CHANNEL) -> ::Result;
    pub fn FMOD_ChannelGroup_GetSpectrum(channel_group: *mut FMOD_CHANNELGROUP, spectrum_array: *mut c_float, num_values: c_int, channel_offset: c_int,
        window_type: ::DspFftWindow) -> ::Result;
    pub fn FMOD_ChannelGroup_GetWaveData(channel_group: *mut FMOD_CHANNELGROUP, wave_array: *mut c_float, num_values: c_int, channel_offset: c_int) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_ChannelGroup_SetUserData(channel_group: *mut FMOD_CHANNELGROUP, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_ChannelGroup_GetUserData(channel_group: *mut FMOD_CHANNELGROUP, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_ChannelGroup_GetMemoryInfo(channel_group: *mut FMOD_CHANNELGROUP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memoryused_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;


    /* sound_group functions */
    pub fn FMOD_SoundGroup_Release(sound_group: *mut FMOD_SOUNDGROUP) -> ::Result;
    /* SoundGroup control functions.*/
    pub fn FMOD_SoundGroup_SetMaxAudible(sound_group: *mut FMOD_SOUNDGROUP, max_audible: c_int) -> ::Result;
    pub fn FMOD_SoundGroup_GetMaxAudible(sound_group: *mut FMOD_SOUNDGROUP, max_audible: *mut c_int) -> ::Result;
    pub fn FMOD_SoundGroup_SetMaxAudibleBehavior(sound_group: *mut FMOD_SOUNDGROUP, behavior: ::SoundGroupBehavior) -> ::Result;
    pub fn FMOD_SoundGroup_GetMaxAudibleBehavior(sound_group: *mut FMOD_SOUNDGROUP, behavior: *mut ::SoundGroupBehavior) -> ::Result;
    pub fn FMOD_SoundGroup_SetMuteFadeSpeed(sound_group: *mut FMOD_SOUNDGROUP, speed: c_float) -> ::Result;
    pub fn FMOD_SoundGroup_GetMuteFadeSpeed(sound_group: *mut FMOD_SOUNDGROUP, speed: *mut c_float) -> ::Result;
    pub fn FMOD_SoundGroup_SetVolume(sound_group: *mut FMOD_SOUNDGROUP, volume: c_float) -> ::Result;
    pub fn FMOD_SoundGroup_GetVolume(sound_group: *mut FMOD_SOUNDGROUP, volume: *mut c_float) -> ::Result;
    pub fn FMOD_SoundGroup_Stop(sound_group: *mut FMOD_SOUNDGROUP) -> ::Result;
    /* Information only functions. */
    pub fn FMOD_SoundGroup_GetName(sound_group: *mut FMOD_SOUNDGROUP, name: *mut c_char, name_len: c_int) -> ::Result;
    pub fn FMOD_SoundGroup_GetNumSounds(sound_group: *mut FMOD_SOUNDGROUP, num_sounds: *mut c_int) -> ::Result;
    pub fn FMOD_SoundGroup_GetSound(sound_group: *mut FMOD_SOUNDGROUP, index: c_int, sound: *mut *mut FMOD_SOUND) -> ::Result;
    pub fn FMOD_SoundGroup_GetNumPlaying(sound_group: *mut FMOD_SOUNDGROUP, num_playing: *mut c_int) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_SoundGroup_SetUserData(sound_group: *mut FMOD_SOUNDGROUP, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_SoundGroup_GetUserData(sound_group: *mut FMOD_SOUNDGROUP, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_SoundGroup_GetMemoryInfo(sound_group: *mut FMOD_SOUNDGROUP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memoryused_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;


    /* Dsp functions */
    pub fn FMOD_DSP_Release(dsp: *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_System_PlayDSP(system: *mut FMOD_SYSTEM, channel_id: ::ChannelIndex, dsp: *mut FMOD_DSP, paused: FMOD_BOOL,
        channel: *mut *mut FMOD_CHANNEL) -> ::Result;
    pub fn FMOD_DSP_GetSystemObject(dsp: *mut FMOD_DSP, system: *mut *mut FMOD_SYSTEM) -> ::Result;
    /* Connection / disconnection / input and output enumeration. */
    pub fn FMOD_DSP_AddInput(dsp: *mut FMOD_DSP, target: *mut FMOD_DSP, connection: *mut *mut FMOD_DSPCONNECTION) -> ::Result;
    pub fn FMOD_DSP_DisconnectFrom(dsp: *mut FMOD_DSP, target: *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_DSP_DisconnectAll(dsp: *mut FMOD_DSP, inputs: FMOD_BOOL, outputs: FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_Remove(dsp: *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_DSP_GetNumInputs(dsp: *mut FMOD_DSP, num_inputs: *mut c_int) -> ::Result;
    pub fn FMOD_DSP_GetNumOutputs(dsp: *mut FMOD_DSP, num_outputs: *mut c_int) -> ::Result;
    pub fn FMOD_DSP_GetInput(dsp: *mut FMOD_DSP, index: c_int, input: *mut *mut FMOD_DSP, input_connection: *mut *mut FMOD_DSPCONNECTION) -> ::Result;
    pub fn FMOD_DSP_GetOutput(dsp: *mut FMOD_DSP, index: c_int, output: *mut *mut FMOD_DSP, output_connection: *mut *mut FMOD_DSPCONNECTION) -> ::Result;
    /* DSP unit control. */
    pub fn FMOD_DSP_SetActive(dsp: *mut FMOD_DSP, active: FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_GetActive(dsp: *mut FMOD_DSP, active: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_SetBypass(dsp: *mut FMOD_DSP, bypass: FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_GetBypass(dsp: *mut FMOD_DSP, bypass: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_SetSpeakerActive(dsp: *mut FMOD_DSP, speaker: ::Speaker, active: FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_GetSpeakerActive(dsp: *mut FMOD_DSP, speaker: ::Speaker, active: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_DSP_Reset(dsp: *mut FMOD_DSP) -> ::Result;
    /* DSP parameter control. */
    pub fn FMOD_DSP_SetParameter(dsp: *mut FMOD_DSP, index: c_int, value: c_float) -> ::Result;
    pub fn FMOD_DSP_GetParameter(dsp: *mut FMOD_DSP, index: c_int, value: *mut c_float, value_str: *mut c_char, value_str_len: c_int) -> ::Result;
    pub fn FMOD_DSP_GetNumParameters(dsp: *mut FMOD_DSP, num_params: *mut c_int) -> ::Result;
    pub fn FMOD_DSP_GetParameterInfo(dsp: *mut FMOD_DSP, index: c_int, name: *mut c_char, label: *mut c_char, description: *mut c_char, description_len: c_int,
        min: *mut c_float, max: *mut c_float) -> ::Result;
    /* I'll bind it later */
    pub fn FMOD_DSP_ShowConfigDialog(dsp: *mut FMOD_DSP, hwnd: *mut c_void, show: FMOD_BOOL) -> ::Result;
    /* DSP attributes. */
    pub fn FMOD_DSP_GetInfo(dsp: *mut FMOD_DSP, name: *mut c_char, version: *mut c_uint, channels: *mut c_int, config_width: *mut c_int, config_height: *mut c_int) -> ::Result;
    pub fn FMOD_DSP_GetType(dsp: *mut FMOD_DSP, _type: *mut ::DspType) -> ::Result;
    pub fn FMOD_DSP_SetDefaults(dsp: *mut FMOD_DSP, frequency: c_float, volume: c_float, pan: c_float, priority: c_int) -> ::Result;
    pub fn FMOD_DSP_GetDefaults(dsp: *mut FMOD_DSP, frequency: *mut c_float, volume: *mut c_float, pan: *mut c_float, priority: *mut c_int) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_DSP_SetUserData(dsp: *mut FMOD_DSP, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_DSP_GetUserData(dsp: *mut FMOD_DSP, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_DSP_GetMemoryInfo(dsp: *mut FMOD_DSP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memory_used_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;


    /* DspConnection functions */
    pub fn FMOD_DSPConnection_GetInput(dsp_connection: *mut FMOD_DSPCONNECTION, input: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_DSPConnection_GetOutput(dsp_connection: *mut FMOD_DSPCONNECTION, output: *mut *mut FMOD_DSP) -> ::Result;
    pub fn FMOD_DSPConnection_SetMix(dsp_connection: *mut FMOD_DSPCONNECTION, volume: c_float) -> ::Result;
    pub fn FMOD_DSPConnection_GetMix(dsp_connection: *mut FMOD_DSPCONNECTION, volume: *mut c_float) -> ::Result;
    pub fn FMOD_DSPConnection_SetLevels(dsp_connection: *mut FMOD_DSPCONNECTION, speaker: ::Speaker, levels: *mut c_float, num_levels: c_int) -> ::Result;
    pub fn FMOD_DSPConnection_GetLevels(dsp_connection: *mut FMOD_DSPCONNECTION, speaker: ::Speaker, levels: *mut c_float, num_levels: c_int) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_DSPConnection_SetUserData(dsp_connection: *mut FMOD_DSPCONNECTION, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_DSPConnection_GetUserData(dsp_connection: *mut FMOD_DSPCONNECTION, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_DSPConnection_GetMemoryInfo(dsp_connection: *mut FMOD_DSPCONNECTION, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memory_used_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;


    /* geometry functions */
    pub fn FMOD_Geometry_Release(geometry: *mut FMOD_GEOMETRY) -> ::Result;
    /* Polygon manipulation. */
    pub fn FMOD_Geometry_AddPolygon(geometry: *mut FMOD_GEOMETRY, direct_occlusion: c_float, reverb_occlusion: c_float, double_sided: FMOD_BOOL, num_vertices: c_int,
        vertices: *const FMOD_VECTOR, polygon_index: *mut c_int) -> ::Result;
    pub fn FMOD_Geometry_GetNumPolygons(geometry: *mut FMOD_GEOMETRY, num_polygons: *mut c_int) -> ::Result;
    pub fn FMOD_Geometry_GetMaxPolygons(geometry: *mut FMOD_GEOMETRY, max_polygons: *mut c_int, max_vertices: *mut c_int) -> ::Result;
    pub fn FMOD_Geometry_GetPolygonNumVertices(geometry: *mut FMOD_GEOMETRY, index: c_int, num_vertices: *mut c_int) -> ::Result;
    pub fn FMOD_Geometry_SetPolygonVertex(geometry: *mut FMOD_GEOMETRY, index: c_int, vertex_index: c_int, vertex: *const FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_GetPolygonVertex(geometry: *mut FMOD_GEOMETRY, index: c_int, vertex_index: c_int, vertex: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_SetPolygonAttributes(geometry: *mut FMOD_GEOMETRY, index: c_int, direct_occlusion: c_float, reverb_occlusion: c_float,
        double_sided: FMOD_BOOL) -> ::Result;
    pub fn FMOD_Geometry_GetPolygonAttributes(geometry: *mut FMOD_GEOMETRY, index: c_int, direct_occlusion: *mut c_float, reverb_occlusion: *mut c_float,
        double_sided: *mut FMOD_BOOL) -> ::Result;
    /* Object manipulation. */
    pub fn FMOD_Geometry_SetActive(geometry: *mut FMOD_GEOMETRY, active: FMOD_BOOL) -> ::Result;
    pub fn FMOD_Geometry_GetActive(geometry: *mut FMOD_GEOMETRY, active: *mut FMOD_BOOL) -> ::Result;
    pub fn FMOD_Geometry_SetRotation(geometry: *mut FMOD_GEOMETRY, forward: *const FMOD_VECTOR, up: *const FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_GetRotation(geometry: *mut FMOD_GEOMETRY, forward: *mut FMOD_VECTOR, up: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_SetPosition(geometry: *mut FMOD_GEOMETRY, position: *const FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_GetPosition(geometry: *mut FMOD_GEOMETRY, position: *mut FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_SetScale(geometry: *mut FMOD_GEOMETRY, scale: *const FMOD_VECTOR) -> ::Result;
    pub fn FMOD_Geometry_GetScale(geometry: *mut FMOD_GEOMETRY, scale: *mut FMOD_VECTOR) -> ::Result;
    /* I'll bind it later */
    pub fn FMOD_Geometry_Save(geometry: *mut FMOD_GEOMETRY, data: *mut c_void, data_size: *mut c_int) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_Geometry_SetUserData(geometry: *mut FMOD_GEOMETRY, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_Geometry_GetUserData(geometry: *mut FMOD_GEOMETRY, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_Geometry_GetMemoryInfo(geometry: *mut FMOD_GEOMETRY, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memory_used_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;


    /* reverb function */
    pub fn FMOD_Reverb_Release(reverb: *mut FMOD_REVERB) -> ::Result;
    pub fn FMOD_Reverb_Set3DAttributes(reverb: *mut FMOD_REVERB, position: *const FMOD_VECTOR, min_distance: c_float, max_distance: c_float) -> ::Result;
    pub fn FMOD_Reverb_Get3DAttributes(reverb: *mut FMOD_REVERB, position: *mut FMOD_VECTOR, min_distance: *mut c_float, max_distance: *mut c_float) -> ::Result;
    pub fn FMOD_Reverb_SetProperties(reverb: *mut FMOD_REVERB, properties: *const FMOD_REVERB_PROPERTIES) -> ::Result;
    pub fn FMOD_Reverb_GetProperties(reverb: *mut FMOD_REVERB, properties: *mut FMOD_REVERB_PROPERTIES) -> ::Result;
    pub fn FMOD_Reverb_SetActive(reverb: *mut FMOD_REVERB, active: FMOD_BOOL) -> ::Result;
    pub fn FMOD_Reverb_GetActive(reverb: *mut FMOD_REVERB, active: *mut FMOD_BOOL) -> ::Result;
    /* Userdata set/get. */
    pub fn FMOD_Reverb_SetUserData(reverb: *mut FMOD_REVERB, user_data: *mut c_void) -> ::Result;
    pub fn FMOD_Reverb_GetUserData(reverb: *mut FMOD_REVERB, user_data: *mut *mut c_void) -> ::Result;
    pub fn FMOD_Reverb_GetMemoryInfo(reverb: *mut FMOD_REVERB, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *mut c_uint,
        memory_used_details: *mut FMOD_MEMORY_USAGE_DETAILS) -> ::Result;
}

pub struct FMOD_ASYNCREADINFO
{
    pub handle     : *mut c_void,   /* [r] The file handle that was filled out in the open callback. */
    pub offset     : c_uint,        /* [r] Seek position, make sure you read from this file offset. */
    pub sizebytes  : c_uint,        /* [r] how many bytes requested for read. */
    pub priority   : c_int,         /* [r] 0 = low importance. 100 = extremely important (ie 'must read now or stuttering may occur') */

    pub buffer     : *mut c_void,   /* [w] Buffer to read file data into. */
    pub bytesread  : c_uint,        /* [w] Fill this in before setting result code to tell FMOD how many bytes were read. */
    pub result     : ::Result, /* [r/w] Result code, Ok tells the system it is ready to consume the data. Set this last!  Default value = FMOD_ERR_NOTREADY. */
    pub userdata   : *mut c_void    /* [r] User data pointer. */
}

pub struct FMOD_CREATESOUNDEXINFO
{
    pub cbsize             : c_int,                        /* [w] Size of this structure. This is used so the structure can be expanded in the future and still work on older versions of FMOD Ex. */
    pub length             : c_uint,                       /* [w] Optional. Specify 0 to ignore. Size in bytes of file to load, or sound to create (in this case only if FMOD_OPENUSER is used). Required if loading from memory. If 0 is specified, then it will use the size of the file (unless loading from memory then an error will be returned). */
    pub fileoffset         : c_uint,                       /* [w] Optional. Specify 0 to ignore. Offset from start of the file to start loading from. This is useful for loading files from inside big data files. */
    pub numchannels        : c_int,                        /* [w] Optional. Specify 0 to ignore. Number of channels in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. */
    pub defaultfrequency   : c_int,                        /* [w] Optional. Specify 0 to ignore. Default frequency of sound in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. Other formats use the frequency determined by the file format. */
    pub format             : ::SoundFormat,           /* [w] Optional. Specify 0 or ::SoundFormatNone to ignore. Format of the sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. Other formats use the format determined by the file format.  */
    pub decodebuffersize   : c_uint,                       /* [w] Optional. Specify 0 to ignore. For streams. This determines the size of the double buffer (in PCM samples) that a stream uses. Use this for user created streams if you want to determine the size of the callback buffer passed to you. Specify 0 to use FMOD's default size which is currently equivalent to 400ms of the sound format created/loaded. */
    pub initialsubsound    : c_int,                        /* [w] Optional. Specify 0 to ignore. In a multi-sample file format such as .FSB/.DLS/.SF2, specify the initial subsound to seek to, only if FMOD_CREATESTREAM is used. */
    pub numsubsounds       : c_int,                        /* [w] Optional. Specify 0 to ignore or have no subsounds. In a sound created with FMOD_OPENUSER, specify the number of subsounds that are accessable with Sound::getSubSound. If not created with FMOD_OPENUSER, this will limit the number of subsounds loaded within a multi-subsound file. If using FSB, then if FMOD_CREATESOUNDEXINFO::inclusionlist is used, this will shuffle subsounds down so that there are not any gaps. It will mean that the indices of the sounds will be different. */
    pub inclusionlist      : *mut c_int,                   /* [w] Optional. Specify 0 to ignore. In a multi-sample format such as .FSB/.DLS/.SF2 it may be desirable to specify only a subset of sounds to be loaded out of the whole file. This is an array of subsound indices to load into memory when created. */
    pub inclusionlistnum   : c_int,                        /* [w] Optional. Specify 0 to ignore. This is the number of integers contained within the inclusionlist array. */
    pub pcmreadcallback    : FMOD_SOUND_PCMREADCALLBACK,   /* [w] Optional. Specify 0 to ignore. Callback to 'piggyback' on FMOD's read functions and accept or even write PCM data while FMOD is opening the sound. Used for user sounds created with FMOD_OPENUSER or for capturing decoded data as FMOD reads it. */
    pub pcmsetposcallback  : FMOD_SOUND_PCMSETPOSCALLBACK, /* [w] Optional. Specify 0 to ignore. Callback for when the user calls a seeking function such as Channel::setTime or Channel::setPosition within a multi-sample sound, and for when it is opened.*/
    pub nonblockcallback   : FMOD_SOUND_NONBLOCKCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for successful completion, or error while loading a sound that used the FMOD_NONBLOCKING flag. Also called duing seeking, when setPosition is called or a stream is restarted. */
    pub dlsname            : *mut c_char,                  /* [w] Optional. Specify 0 to ignore. Filename for a DLS or SF2 sample set when loading a MIDI file. If not specified, on Windows it will attempt to open /windows/system32/drivers/gm.dls or /windows/system32/drivers/etc/gm.dls, on Mac it will attempt to load /System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls, otherwise the MIDI will fail to open. Current DLS support is for level 1 of the specification. */
    pub encryptionkey      : *mut c_char,                  /* [w] Optional. Specify 0 to ignore. Key for encrypted FSB file. Without this key an encrypted FSB file will not load. */
    pub maxpolyphony       : c_int,                        /* [w] Optional. Specify 0 to ignore. For sequenced formats with dynamic channel allocation such as .MID and .IT, this specifies the maximum voice count allowed while playing. .IT defaults to 64. .MID defaults to 32. */
    pub userdata           : *mut c_void,                  /* [w] Optional. Specify 0 to ignore. This is user data to be attached to the sound during creation. Access via Sound::getUserData. Note: This is not passed to FMOD_FILE_OPENCALLBACK, that is a different userdata that is file specific. */
    pub suggestedsoundtype : ::SoundType,             /* [w] Optional. Specify 0 or SoundTypeUnknown to ignore. Instead of scanning all codec types, use this to speed up loading by making it jump straight to this codec. */
    pub useropen           : FMOD_FILE_OPENCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for opening this file. */
    pub userclose          : FMOD_FILE_CLOSECALLBACK,      /* [w] Optional. Specify 0 to ignore. Callback for closing this file. */
    pub userread           : FMOD_FILE_READCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for reading from this file. */
    pub userseek           : FMOD_FILE_SEEKCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub userasyncread      : FMOD_FILE_ASYNCREADCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub userasynccancel    : FMOD_FILE_ASYNCCANCELCALLBACK,/* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub speakermap         : ::SpeakerMapType,        /* [w] Optional. Specify 0 to ignore. Use this to differ the way fmod maps multichannel sounds to speakers. See SpeakerMapType for more. */
    pub initialsoundgroup  : *mut FMOD_SOUNDGROUP,         /* [w] Optional. Specify 0 to ignore. Specify a sound group if required, to put sound in as it is created. */
    pub initialseekposition: c_uint,                       /* [w] Optional. Specify 0 to ignore. For streams. Specify an initial position to seek the stream to. */
    pub initialseekpostype : FMOD_TIMEUNIT,                /* [w] Optional. Specify 0 to ignore. For streams. Specify the time unit for the position set in initialseekposition. */
    pub ignoresetfilesystem: c_int,                        /* [w] Optional. Specify 0 to ignore. Set to 1 to use fmod's built in file system. Ignores setFileSystem callbacks and also FMOD_CREATESOUNEXINFO file callbacks. Useful for specific cases where you don't want to use your own file system but want to use fmod's file system (ie net streaming). */
    pub cddaforceaspi      : c_int,                        /* [w] Optional. Specify 0 to ignore. For CDDA sounds only - if non-zero use ASPI instead of NTSCSI to access the specified CD/DVD device. */
    pub audioqueuepolicy   : c_uint,                       /* [w] Optional. Specify 0 or FMOD_AUDIOQUEUE_CODECPOLICY_DEFAULT to ignore. Policy used to determine whether hardware or software is used for decoding, see FMOD_AUDIOQUEUE_CODECPOLICY for options (iOS >= 3.0 required, otherwise only hardware is available) */ 
    pub minmidigranularity : c_uint,                       /* [w] Optional. Specify 0 to ignore. Allows you to set a minimum desired MIDI mixer granularity. Values smaller than 512 give greater than default accuracy at the cost of more CPU and vice versa. Specify 0 for default (512 samples). */
    pub nonblockthreadid   : c_int                         /* [w] Optional. Specify 0 to ignore. Specifies a thread index to execute non blocking load on. Allows for up to 5 threads to be used for loading at once. This is to avoid one load blocking another. Maximum value = 4. */
}

pub struct FMOD_REVERB_CHANNELPROPERTIES
{                                          /*       MIN    MAX  DEFAULT  DESCRIPTION */
    pub Direct         : c_int,            /* [r/w] -10000 1000 0        Direct path level                                        (SUPPORTED:SFX) */
    pub Room           : c_int,            /* [r/w] -10000 1000 0        Room effect level                                        (SUPPORTED:SFX) */
    pub Flags          : c_uint,           /* [r/w] *mut FMOD_REVERB_CHANNELFLAGS - modifies the behavior of properties           (SUPPORTED:SFX) */
    pub ConnectionPoint: *mut FMOD_DSP     /* [r/w] See remarks.        DSP network location to connect reverb for this channel. (SUPPORTED:SFX).*/
}

pub struct FMOD_GUID
{
    pub Data1: c_uint,              /* Specifies the first 8 hexadecimal digits of the GUID */
    pub Data2: c_ushort,            /* Specifies the first group of 4 hexadecimal digits.  */
    pub Data3: c_ushort,            /* Specifies the second group of 4 hexadecimal digits. */
    pub Data4: [c_uchar; 8]         /* Array of 8 bytes. The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits. */
}

pub struct FMOD_ADVANCEDSETTINGS
{
    pub cbsize                     : c_int,              /* [w]   Size of this structure. Use sizeof(FMOD_ADVANCEDSETTINGS)  NOTE: This must be set before calling System::getAdvancedSettings! */
    pub maxMPEGcodecs              : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. Mpeg  codecs consume 21,684 bytes per instance and this number will determine how many mpeg channels can be played simultaneously.  Default = 32. */
    pub maxADPCMcodecs             : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. ADPCM codecs consume  2,136 bytes per instance and this number will determine how many ADPCM channels can be played simultaneously. Default = 32. */
    pub maxXMAcodecs               : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. XMA   codecs consume 14,836 bytes per instance and this number will determine how many XMA channels can be played simultaneously.   Default = 32. */
    pub maxCELTcodecs              : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. CELT  codecs consume 11,500 bytes per instance and this number will determine how many CELT channels can be played simultaneously.  Default = 32. */    
    pub maxVORBIScodecs            : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. Vorbis codecs consume 12,000 bytes per instance and this number will determine how many Vorbis channels can be played simultaneously. Default = 32. */    
    pub maxAT9Codecs               : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only. AT9 codecs consume  8,720 bytes per instance and this number will determine how many AT9 channels can be played simultaneously. Default = 32. */    
    pub maxPCMcodecs               : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with PS3 only.                         PCM   codecs consume 12,672 bytes per instance and this number will determine how many streams and PCM voices can be played simultaneously. Default = 16. */
    pub ASIONumChannels            : c_int,              /* [r/w] Optional. Specify 0 to ignore. Number of channels available on the ASIO device. */
    pub ASIOChannelList            : *mut *mut c_char,   /* [r/w] Optional. Specify 0 to ignore. Pointer to an array of strings (number of entries defined by ASIONumChannels) with ASIO channel names. */
    pub ASIOSpeakerList            : *mut ::Speaker, /* [r/w] Optional. Specify 0 to ignore. Pointer to a list of speakers that the ASIO channels map to. This can be called after System::init to remap ASIO output. */
    pub max3DReverbDSPs            : c_int,              /* [r/w] Optional. Specify 0 to ignore. The max number of 3d reverb DSP's in the system. (NOTE: CURRENTLY DISABLED / UNUSED) */
    pub HRTFMinAngle               : c_float,            /* [r/w] Optional.                     For use with FMOD_INIT_HRTF_LOWPASS. The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function begins to have an effect. 0 = in front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 180.0. */
    pub HRTFMaxAngle               : c_float,            /* [r/w] Optional.                     For use with FMOD_INIT_HRTF_LOWPASS. The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function has maximum effect. 0 = front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 360.0. */
    pub HRTFFreq                   : c_float,            /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_HRTF_LOWPASS. The cutoff frequency of the HRTF's lowpass filter function when at maximum effect. (i.e. at HRTFMaxAngle). Default = 4000.0. */
    pub vol0virtualvol             : c_float,            /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_VOL0_BECOMES_VIRTUAL. If this flag is used, and the volume is 0.0, then the sound will become virtual. Use this value to raise the threshold to a different point where a sound goes virtual. */
    pub eventqueuesize             : c_int,              /* [r/w] Optional. Specify 0 to ignore. For use with FMOD Event system only. Specifies the number of slots available for simultaneous non blocking loads, across all threads. Default = 32. */
    pub defaultDecodeBufferSize    : c_uint,             /* [r/w] Optional. Specify 0 to ignore. For streams. This determines the default size of the double buffer (in milliseconds) that a stream uses. Default = 400ms */
    pub debugLogFilename           : *mut c_char,        /* [r/w] Optional. Specify 0 to ignore. Gives fmod's logging system a path/filename. Normally the log is placed in the same directory as the executable and called fmod.log. When using System::getAdvancedSettings, provide at least 256 bytes of memory to copy into. */
    pub profileport                : c_ushort,           /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_ENABLE_PROFILE. Specify the port to listen on for connections by the profiler application. */
    pub geometryMaxFadeTime        : c_uint,             /* [r/w] Optional. Specify 0 to ignore. The maximum time in miliseconds it takes for a channel to fade to the new level when its occlusion changes. */
    pub maxSpectrumWaveDataBuffers : c_uint,             /* [r/w] Optional. Specify 0 to ignore. Tells System::init to allocate a pool of wavedata/spectrum buffers to prevent memory fragmentation, any additional buffers will be allocated normally. */
    pub musicSystemCacheDelay      : c_uint,             /* [r/w] Optional. Specify 0 to ignore. The delay the music system should allow for loading a sample from disk (in milliseconds). Default = 400 ms. */
    pub distanceFilterCenterFreq   : c_float,            /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_DISTANCE_FILTERING. The default center frequency in Hz for the distance filtering effect. Default = 1500.0. */
    pub stackSizeStream            : c_uint,             /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD Stream thread in bytes. Useful for custom codecs that use excess stack. Default 49,152 (48kb) */
    pub stackSizeNonBlocking       : c_uint,             /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD_NONBLOCKING loading thread. Useful for custom codecs that use excess stack. Default 65,536 (64kb) */
    pub stackSizeMixer             : c_uint              /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD mixer thread. Useful for custom dsps that use excess stack. Default 49,152 (48kb) */
}

pub struct FMOD_CODEC_DESCRIPTION
{
    pub name           : *mut c_char,                   /* [in] Name of the codec. */
    pub version        : c_uint,                        /* [in] Plugin writer's version number. */
    pub defaultasstream: c_int,                         /* [in] Tells FMOD to open the file as a stream when calling System::createSound, and not a static sample. Should normally be 0 (FALSE), because generally the user wants to decode the file into memory when using System::createSound.  Mainly used for formats that decode for a very long time, or could use large amounts of memory when decoded. Usually sequenced formats such as mod/s3m/xm/it/midi fall into this category.  It is mainly to stop users that don't know what they're doing from getting FMOD_ERR_MEMORY returned from createSound when they should have in fact called System::createStream or used FMOD_CREATESTREAM in System::createSound. */
    pub timeunits      : FMOD_TIMEUNIT,                 /* [in] When setposition codec is called, only these time formats will be passed to the codec. Use bitwise OR to accumulate different types. */
    pub open           : FMOD_CODEC_OPENCALLBACK,       /* [in] Open callback for the codec for when FMOD tries to open a sound using this codec. */
    pub close          : FMOD_CODEC_CLOSECALLBACK,      /* [in] Close callback for the codec for when FMOD tries to close a sound using this codec. */
    pub read           : FMOD_CODEC_READCALLBACK,       /* [in] Read callback for the codec for when FMOD tries to read some data from the file to the destination format (specified in the open callback). */
    pub getlength      : FMOD_CODEC_GETLENGTHCALLBACK,  /* [in] Callback to return the length of the song in whatever format required when Sound::getLength is called. */
    pub setposition    : FMOD_CODEC_SETPOSITIONCALLBACK,/* [in] Seek callback for the codec for when FMOD tries to seek within the file with Channel::setPosition. */
    pub getposition    : FMOD_CODEC_GETPOSITIONCALLBACK,/* [in] Tell callback for the codec for when FMOD tries to get the current position within the with Channel::getPosition. */
    pub soundcreate    : FMOD_CODEC_SOUNDCREATECALLBACK,/* [in] Sound creation callback for the codec when FMOD finishes creating the sound. (So the codec can set more parameters for the related created sound, ie loop points/mode or 3D attributes etc). */
    pub getwaveformat  : FMOD_CODEC_GETWAVEFORMAT       /* [in] Callback to tell FMOD about the waveformat of a particular subsound. This is to save memory, rather than saving 1000 FMOD_CODEC_WAVEFORMAT structures in the codec, the codec might have a more optimal way of storing this information. */
}

pub struct FMOD_CODEC_WAVEFORMAT
{
    pub name       : [c_char; 256],    /* [in] Name of sound.*/
    pub format     : ::SoundFormat,    /* [in] Format for (decompressed) codec output, ie ::SoundFormat_PCM8, ::SoundFormat_PCM16.*/
    pub channels   : c_int,            /* [in] Number of channels used by codec, ie mono = 1, stereo = 2. */
    pub frequency  : c_int,            /* [in] Default frequency in hz of the codec, ie 44100. */
    pub lengthbytes: c_uint,           /* [in] Length in bytes of the source data. */
    pub lengthpcm  : c_uint,           /* [in] Length in decompressed, PCM samples of the file, ie length in seconds * frequency. Used for Sound::getLength and for memory allocation of static decompressed sample data. */
    pub blockalign : c_int,            /* [in] Blockalign in decompressed, PCM samples of the optimal decode chunk size for this format. The codec read callback will be called in multiples of this value. */
    pub loopstart  : c_int,            /* [in] Loopstart in decompressed, PCM samples of file. */
    pub loopend    : c_int,            /* [in] Loopend in decompressed, PCM samples of file. */
    pub mode       : FMOD_MODE,        /* [in] Mode to determine whether the sound should by default load as looping, non looping, 2d or 3d. */
    pub channelmask: c_uint            /* [in] Microsoft speaker channel mask, as defined for WAVEFORMATEXTENSIBLE and is found in ksmedia.h. Leave at 0 to play in natural speaker order. */
}

pub struct FMOD_CODEC_STATE
{
    pub numsubsounds: c_int,                      /* [in] Number of 'subsounds' in this sound. Anything other than 0 makes it a 'container' format (ie CDDA/DLS/FSB etc which contain 1 or more su bsounds). For most normal, single sound codec such as WAV/AIFF/MP3, this should be 0 as they are not a container for subsounds, they are the sound by itself. */
    pub waveformat  : FMOD_CODEC_WAVEFORMAT,      /* [in] Pointer to an array of format structures containing information about each sample. Can be 0 or NULL if FMOD_CODEC_GETWAVEFORMAT callback is preferred. The number of entries here must equal the number of subsounds defined in the subsound parameter. If numsubsounds = 0 then there should be 1 instance of this structure. */
    pub plugindata  : *mut c_void,                /* [in] Plugin writer created data the codec author wants to attach to this object. */
                                               
    pub filehandle  : *mut c_void,                /* [out] This will return an internal FMOD file handle to use with the callbacks provided. */
    pub filesize    : c_uint,                     /* [out] This will contain the size of the file in bytes. */
    pub fileread    : FMOD_FILE_READCALLBACK,     /* [out] This will return a callable FMOD file function to use from codec. */
    pub fileseek    : FMOD_FILE_SEEKCALLBACK,     /* [out] This will return a callable FMOD file function to use from codec. */
    pub metadata    : FMOD_CODEC_METADATACALLBACK /* [out] This will return a callable FMOD metadata function to use from codec. */
}

pub struct FMOD_REVERB_PROPERTIES
{                                      /*       MIN    MAX     DEFAULT DESCRIPTION */
    pub Instance        : c_int,       /* [w]   0      3       0       Environment Instance.                                                (SUPPORTED:SFX(4 instances) and Wii (3 instances)) */
    pub Environment     : c_int,       /* [r/w] -1     25      -1      Sets all listener properties. -1 = OFF.                             (SUPPORTED:SFX(-1 only)/PSP) */
    pub EnvDiffusion    : c_float,     /* [r/w] 0.0    1.0     1.0     Environment diffusion                                                 (SUPPORTED:WII) */
    pub Room            : c_int,       /* [r/w] -10000 0       -1000   Room effect level (at mid frequencies)                                (SUPPORTED:SFX/WII/PSP) */
    pub RoomHF          : c_int,       /* [r/w] -10000 0       -100    Relative room effect level at high frequencies                        (SUPPORTED:SFX) */
    pub RoomLF          : c_int,       /* [r/w] -10000 0       0       Relative room effect level at low frequencies                         (SUPPORTED:SFX) */
    pub DecayTime       : c_float,     /* [r/w] 0.1    20.0    1.49    Reverberation decay time at mid frequencies                           (SUPPORTED:SFX/WII) */
    pub DecayHFRatio    : c_float,     /* [r/w] 0.1    2.0     0.83    High-frequency to mid-frequency decay time ratio                      (SUPPORTED:SFX) */
    pub DecayLFRatio    : c_float,     /* [r/w] 0.1    2.0     1.0     Low-frequency to mid-frequency decay time ratio                       (SUPPORTED:---) */
    pub Reflections     : c_int,       /* [r/w] -10000 1000    -2602   Early reflections level relative to room effect                       (SUPPORTED:SFX/WII) */
    pub ReflectionsDelay: c_float,     /* [r/w] 0.0    0.3     0.007   Initial reflection delay time                                         (SUPPORTED:SFX) */
    pub Reverb          : c_int,       /* [r/w] -10000 2000    200     Late reverberation level relative to room effect                      (SUPPORTED:SFX) */
    pub ReverbDelay     : c_float,     /* [r/w] 0.0    0.1     0.011   Late reverberation delay time relative to initial reflection          (SUPPORTED:SFX/WII) */
    pub ModulationTime  : c_float,     /* [r/w] 0.04   4.0     0.25    Modulation time                                                       (SUPPORTED:---) */
    pub ModulationDepth : c_float,     /* [r/w] 0.0    1.0     0.0     Modulation depth                                                      (SUPPORTED:WII) */
    pub HFReference     : c_float,     /* [r/w] 20.0   20000.0 5000.0  Reference high frequency (hz)                                         (SUPPORTED:SFX) */
    pub LFReference     : c_float,     /* [r/w] 20.0   1000.0  250.0   Reference low frequency (hz)                                          (SUPPORTED:SFX) */
    pub Diffusion       : c_float,     /* [r/w] 0.0    100.0   100.0   Value that controls the echo density in the late reverberation decay. (SUPPORTED:SFX) */
    pub Density         : c_float,     /* [r/w] 0.0    100.0   100.0   Value that controls the modal density in the late reverberation decay (SUPPORTED:SFX) */
    pub Flags           : c_uint       /* [r/w] *mut FMOD_REVERB_FLAGS - modifies the behavior of above properties                                (SUPPORTED:WII) */
}

pub struct FMOD_TAG
{
    pub _type   : ::TagType,      /* [r] The type of this tag. */
    pub datatype: ::TagDataType,  /* [r] The type of data that this tag contains */
    pub name    : *mut c_char,         /* [r] The name of this tag i.e. "TITLE", "ARTIST" etc. */
    pub data    : *mut c_void,         /* [r] Pointer to the tag data - its format is determined by the datatype member */
    pub datalen : c_uint,              /* [r] Length of the data contained in this tag */
    pub updated : FMOD_BOOL            /* [r] True if this tag has been updated since last being accessed with Sound::getTag */
}

pub struct FMOD_VECTOR
{
    pub x: c_float, /* X co-ordinate in 3D space. */
    pub y: c_float, /* Y co-ordinate in 3D space. */
    pub z: c_float  /* Z co-ordinate in 3D space. */
}

pub struct FMOD_MEMORY_USAGE_DETAILS
{
    pub other                  : c_uint, /* [out] Memory not accounted for by other types */
    pub string                 : c_uint, /* [out] String data */
    pub system                 : c_uint, /* [out] System object and various internals */
    pub plugins                : c_uint, /* [out] Plugin objects and internals */
    pub output                 : c_uint, /* [out] Output module object and internals */
    pub channel                : c_uint, /* [out] Channel related memory */
    pub channel_group          : c_uint, /* [out] ChannelGroup objects and internals */
    pub codec                  : c_uint, /* [out] Codecs allocated for streaming */
    pub file                   : c_uint, /* [out] File buffers and structures */
    pub sound                  : c_uint, /* [out] Sound objects and internals */
    pub secondary_ram          : c_uint, /* [out] Sound data stored in secondary RAM */
    pub sound_group            : c_uint, /* [out] SoundGroup objects and internals */
    pub stream_buffer          : c_uint, /* [out] Stream buffer memory */
    pub dsp_connection         : c_uint, /* [out] DSPConnection objects and internals */
    pub dsp                    : c_uint, /* [out] DSP implementation objects */
    pub dsp_codec              : c_uint, /* [out] Realtime file format decoding DSP objects */
    pub profile                : c_uint, /* [out] Profiler memory footprint. */
    pub record_buffer          : c_uint, /* [out] Buffer used to store recorded data from microphone */
    pub reverb                 : c_uint, /* [out] Reverb implementation objects */
    pub reverb_channel_props   : c_uint, /* [out] Reverb channel properties structs */
    pub geometry               : c_uint, /* [out] Geometry objects and internals */
    pub sync_point             : c_uint, /* [out] Sync point memory. */
    pub event_system           : c_uint, /* [out] EventSystem and various internals */
    pub music_system           : c_uint, /* [out] MusicSystem and various internals */
    pub fev                    : c_uint, /* [out] Definition of objects contained in all loaded projects e.g. events, groups, categories */
    pub memory_fsb             : c_uint, /* [out] Data loaded with preloadFSB */
    pub event_project          : c_uint, /* [out] EventProject objects and internals */
    pub event_group_i          : c_uint, /* [out] EventGroup objects and internals */
    pub sound_bank_class       : c_uint, /* [out] Objects used to manage wave banks */
    pub sound_bank_list        : c_uint, /* [out] Data used to manage lists of wave bank usage */
    pub stream_instance        : c_uint, /* [out] Stream objects and internals */
    pub sound_def_class        : c_uint, /* [out] Sound definition objects */
    pub sound_def_def_class    : c_uint, /* [out] Sound definition static data objects */
    pub sound_def_pool         : c_uint, /* [out] Sound definition pool data */
    pub reverb_def             : c_uint, /* [out] Reverb definition objects */
    pub event_reverb           : c_uint, /* [out] Reverb objects */
    pub user_property          : c_uint, /* [out] User property objects */
    pub event_instance         : c_uint, /* [out] Event instance base objects */
    pub event_instance_complex : c_uint, /* [out] Complex event instance objects */
    pub event_instance_simple  : c_uint, /* [out] Simple event instance objects */
    pub event_instance_layer   : c_uint, /* [out] Event layer instance objects */
    pub event_instance_sound   : c_uint, /* [out] Event sound instance objects */
    pub event_envelope         : c_uint, /* [out] Event envelope objects */
    pub event_envelope_def     : c_uint, /* [out] Event envelope definition objects */
    pub event_parameter        : c_uint, /* [out] Event parameter objects */
    pub event_category         : c_uint, /* [out] Event category objects */
    pub event_envelope_point   : c_uint, /* [out] Event envelope point objects */
    pub event_instance_pool    : c_uint  /* [out] Event instance pool memory */
}

pub struct FMOD_DSP_PARAMETERDESC
{
    pub min         : c_float,      /* [w] Minimum value of the parameter (ie 100.0). */
    pub max         : c_float,      /* [w] Maximum value of the parameter (ie 22050.0). */
    pub default_val : c_float,      /* [w] Default value of parameter. */
    pub name        : [c_char; 16], /* [w] Name of the parameter to be displayed (ie "Cutoff frequency"). */
    pub label       : [c_char; 16], /* [w] Short string to be put next to value to denote the unit type (ie "hz"). */
    pub description : *const c_char /* [w] Description of the parameter to be displayed as a help item / tooltip for this parameter. */
}

pub struct FMOD_DSP_DESCRIPTION
{
    pub name                    : [c_char; 32],                 /* [w] Name of the unit to be displayed in the network. */
    pub version                 : c_uint,                       /* [w] Plugin writer's version number. */
    pub channels                : c_int,                        /* [w] Number of channels. Use 0 to process whatever number of channels is currently in the network. >0 would be mostly used if the unit is a unit that only generates sound. */
    pub create                  : FMOD_DSP_CREATECALLBACK,      /* [w] Create callback. This is called when DSP unit is created. Can be null. */
    pub release                 : FMOD_DSP_RELEASECALLBACK,     /* [w] Release callback. This is called just before the unit is freed so the user can do any cleanup needed for the unit. Can be null. */
    pub reset                   : FMOD_DSP_RESETCALLBACK,       /* [w] Reset callback. This is called by the user to reset any history buffers that may need resetting for a filter, when it is to be used or re-used for the first time to its initial clean state. Use to avoid clicks or artifacts. */
    pub read                    : FMOD_DSP_READCALLBACK,        /* [w] Read callback. Processing is done here. Can be null. */
    pub set_position            : FMOD_DSP_SETPOSITIONCALLBACK, /* [w] Set position callback. This is called if the unit wants to update its position info but not process data, or reset a cursor position internally if it is reading data from a certain source. Can be null. */
    pub num_parameters          : c_int,                        /* [w] Number of parameters used in this filter. The user finds this with DSP::getNumParameters */
    pub param_desc              : *mut FMOD_DSP_PARAMETERDESC,  /* [w] Variable number of parameter structures. */
    pub set_parameter           : FMOD_DSP_SETPARAMCALLBACK,    /* [w] This is called when the user calls DSP::setParameter. Can be null. */
    pub get_parameter           : FMOD_DSP_GETPARAMCALLBACK,    /* [w] This is called when the user calls DSP::getParameter. Can be null. */
    pub config                  : FMOD_DSP_DIALOGCALLBACK,      /* [w] This is called when the user calls DSP::showConfigDialog. Can be used to display a dialog to configure the filter. Can be null. */
    pub config_width            : c_int,                        /* [w] Width of config dialog graphic if there is one. 0 otherwise.*/
    pub config_height           : c_int,                        /* [w] Height of config dialog graphic if there is one. 0 otherwise.*/
    pub user_data               : *mut c_void                   /* [w] Optional. Specify 0 to ignore. This is user data to be attached to the DSP unit during creation. Access via DSP::getUserData. */
}

pub struct FMOD_DSP_STATE
{
    pub instance: *mut FMOD_DSP,    /* [r] Handle to the DSP hand the user created. Not to be modified. C++ users cast to DSP to use. */
    pub plugin_data: *mut c_void,   /* [w] Plugin writer created data the output author wants to attach to this object. */
    pub speaker_mask: c_ushort      /* [w] Specifies which speakers the DSP effect is active on */
}

pub struct SoundData {
    pub non_block: SoundNonBlockCallback,
    pub pcm_read: SoundPcmReadCallback,
    pub pcm_set_pos: SoundPcmSetPosCallback,
    pub user_data: *mut c_void
}

impl SoundData {
    pub fn new() -> SoundData {
        SoundData {
            non_block: None,
            pcm_read: None,
            pcm_set_pos: None,
            user_data: ::std::ptr::null_mut()
        }
    }
}
