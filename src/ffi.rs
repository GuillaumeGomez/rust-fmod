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

use enums::*;
use libc::{c_void, c_uint, c_int, c_char, c_float, c_ushort, c_uchar, c_short};

pub type FMOD_FILE_OPENCALLBACK =::std::option::Option<extern "C" fn(name: *c_char, unicode: int, file_size: *c_uint, handle: **c_void,
    user_data: **c_void) -> fmod::Result>;
pub type FMOD_FILE_CLOSECALLBACK =::std::option::Option<extern "C" fn(handle: *c_void, user_data: *c_void) -> fmod::Result>;
pub type FMOD_FILE_READCALLBACK =::std::option::Option<extern "C" fn(handle: *c_void, buffer: *c_void, size_bytes: c_uint, bytes_read: *c_uint,
    user_data: *c_void) -> fmod::Result>;
pub type FMOD_FILE_SEEKCALLBACK =::std::option::Option<extern "C" fn(handle: *c_void, pos: c_uint, user_data: *c_void) -> fmod::Result>;
pub type FMOD_FILE_ASYNCREADCALLBACK =::std::option::Option<extern "C" fn(arg1: *FMOD_ASYNCREADINFO, arg2: *c_void) -> fmod::Result>;
pub type FMOD_FILE_ASYNCCANCELCALLBACK =::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void, arg3: c_uint) -> fmod::Result>;

pub type FMOD_SOUND_NONBLOCKCALLBACK =::std::option::Option<extern "C" fn(sound: FMOD_SOUND, result: fmod::Result) -> fmod::Result>;
pub type FMOD_SOUND_PCMREADCALLBACK =::std::option::Option<extern "C" fn(sound: FMOD_SOUND, data: *c_void, data_len: c_uint) -> fmod::Result>;
pub type FMOD_SOUND_PCMSETPOSCALLBACK =::std::option::Option<extern "C" fn(sound: FMOD_SOUND, sub_sound: c_int, position: c_uint,
    postype: FMOD_TIMEUNIT) -> fmod::Result>;

pub type FMOD_SYSTEM_CALLBACK =::std::option::Option<extern "C" fn(system: FMOD_SYSTEM, _type: fmod::SystemCallbackType, command_data1: *c_void,
    command_data2: *c_void) -> fmod::Result>;

/*  codec callbacks */
pub type FMOD_CODEC_OPENCALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, user_mode: FMOD_MODE, userexinfo: *FMOD_CREATESOUNDEXINFO) -> fmod::Result>;
pub type FMOD_CODEC_CLOSECALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE) -> fmod::Result>;
pub type FMOD_CODEC_READCALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, buffer: *c_void, size_bytes: c_uint, bytes_read: *c_uint) -> fmod::Result>;
pub type FMOD_CODEC_GETLENGTHCALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, length: *c_uint, length_type: FMOD_TIMEUNIT) -> fmod::Result>;
pub type FMOD_CODEC_SETPOSITIONCALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, sub_sound: c_int, position: c_uint, postype: FMOD_TIMEUNIT) -> fmod::Result>;
pub type FMOD_CODEC_GETPOSITIONCALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, position: *c_uint, postype: FMOD_TIMEUNIT) -> fmod::Result>;
pub type FMOD_CODEC_SOUNDCREATECALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, sub_sound: c_int, sound: FMOD_SOUND) -> fmod::Result>;
pub type FMOD_CODEC_METADATACALLBACK =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, tag_type: fmod::TagType, name: *c_char, data: *c_void,
    data_len: c_uint, data_type: fmod::TagDataType, unique: c_int) -> fmod::Result>;
pub type FMOD_CODEC_GETWAVEFORMAT =::std::option::Option<extern "C" fn(codec_state: *FMOD_CODEC_STATE, index: c_int, wave_format: *FMOD_CODEC_WAVEFORMAT) -> fmod::Result>;
pub type FMOD_3D_ROLLOFFCALLBACK =::std::option::Option<extern "C" fn(channel: FMOD_CHANNEL, distance: c_float) -> fmod::Result>;


pub type FMOD_BOOL = c_int;
pub type FMOD_CAPS = c_uint;
pub type FMOD_CHANNEL = *c_void;
pub type FMOD_CHANNELGROUP = *c_void;
pub type FMOD_DSP = *c_void;
pub type FMOD_DSPCONNECTION = *c_void;
pub type FMOD_GEOMETRY = *c_void;
pub type FMOD_INITFLAGS = c_uint;
pub type FMOD_MODE = c_uint;
pub type FMOD_REVERB = *c_void;
pub type FMOD_SOUND = *c_void;
pub type FMOD_SOUNDGROUP = *c_void;
pub type FMOD_SYSTEM = *c_void;
pub type FMOD_SYNCPOINT = *c_void;
pub type FMOD_TIMEUNIT = c_uint;

extern "C" {
    pub fn FMOD_System_Create(system: *FMOD_SYSTEM) -> fmod::Result;
    pub fn FMOD_System_Release(system: FMOD_SYSTEM) -> fmod::Result;
    /* pre-init functions */
    pub fn FMOD_System_SetOutput(system: FMOD_SYSTEM, output_type: fmod::OutputType) -> fmod::Result;
    pub fn FMOD_System_GetOutput(system: FMOD_SYSTEM, output_type: *fmod::OutputType) -> fmod::Result;
    pub fn FMOD_System_GetNumDrivers(system: FMOD_SYSTEM, num_drivers: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetDriverInfo(system: FMOD_SYSTEM, id: c_int, name: *c_char, name_len: c_int, guid: *FMOD_GUID) -> fmod::Result;
    pub fn FMOD_System_GetDriverInfoW(system: FMOD_SYSTEM, id: c_int, name: *c_short, name_len: c_int, guid: *FMOD_GUID) -> fmod::Result;
    pub fn FMOD_System_GetDriverCaps(system: FMOD_SYSTEM, id: c_int, caps: *FMOD_CAPS, control_panel_output_rate: *c_int,
        controlpanelspeakermode: *fmod::SpeakerMode) -> fmod::Result;
    pub fn FMOD_System_SetDriver(system: FMOD_SYSTEM, driver: c_int) -> fmod::Result;
    pub fn FMOD_System_GetDriver(system: FMOD_SYSTEM, driver: *c_int) -> fmod::Result;
    pub fn FMOD_System_SetHardwareChannels(system: FMOD_SYSTEM, num_hardware_channels: c_int) -> fmod::Result;
    pub fn FMOD_System_GetHardwareChannels(system: FMOD_SYSTEM, num_hardware_channels: *c_int) -> fmod::Result;
    pub fn FMOD_System_SetSoftwareChannels(system: FMOD_SYSTEM, num_software_channels: c_int) -> fmod::Result;
    pub fn FMOD_System_GetSoftwareChannels(system: FMOD_SYSTEM, num_software_channels: *c_int) -> fmod::Result;
    pub fn FMOD_System_SetSoftwareFormat(system: FMOD_SYSTEM, sample_rate: c_int, format: fmod::SoundFormat, num_output_channels: c_int,
        max_input_channels: c_int, resample_method: fmod::DSPResampler) -> fmod::Result;
    pub fn FMOD_System_GetSoftwareFormat(system: FMOD_SYSTEM, sample_rate: *c_int, format: *fmod::SoundFormat,
        num_output_channels: *c_int, max_input_channels: *c_int, resample_method: *fmod::DSPResampler, bits: *c_int) -> fmod::Result;
    pub fn FMOD_System_SetDSPBufferSize(system: FMOD_SYSTEM, buffer_length: c_uint, num_buffers: c_int) -> fmod::Result;
    pub fn FMOD_System_GetDSPBufferSize(system: FMOD_SYSTEM, buffer_length: *c_uint, num_buffers: *c_int) -> fmod::Result;
    // I'll bind it a little later
    pub fn FMOD_System_SetFileSystem(system: FMOD_SYSTEM, user_open: FMOD_FILE_OPENCALLBACK, user_close: FMOD_FILE_CLOSECALLBACK,
        user_read: FMOD_FILE_READCALLBACK, user_seek: FMOD_FILE_SEEKCALLBACK, user_async_read: FMOD_FILE_ASYNCREADCALLBACK,
        user_async_cancel: FMOD_FILE_ASYNCCANCELCALLBACK, block_align: c_int) -> fmod::Result;
    // I'll bind it a little later
    pub fn FMOD_System_AttachFileSystem(system: FMOD_SYSTEM, user_open: FMOD_FILE_OPENCALLBACK, user_close: FMOD_FILE_CLOSECALLBACK,
        user_read: FMOD_FILE_READCALLBACK, user_seek: FMOD_FILE_SEEKCALLBACK) -> fmod::Result;
    pub fn FMOD_System_SetAdvancedSettings(system: FMOD_SYSTEM, settings: *FMOD_ADVANCEDSETTINGS) -> fmod::Result;
    pub fn FMOD_System_GetAdvancedSettings(system: FMOD_SYSTEM, settings: *FMOD_ADVANCEDSETTINGS) -> fmod::Result;
    pub fn FMOD_System_SetSpeakerMode(system: FMOD_SYSTEM, speaker_mode: *fmod::SpeakerMode) -> fmod::Result;
    pub fn FMOD_System_GetSpeakerMode(system: FMOD_SYSTEM, speaker_mode: *fmod::SpeakerMode) -> fmod::Result;
    // I'll bind it a little later
    pub fn FMOD_System_SetCallback(system: FMOD_SYSTEM, call_back: FMOD_SYSTEM_CALLBACK) -> fmod::Result;
    /* plug-in part functions */
    pub fn FMOD_System_SetPluginPath(system: FMOD_SYSTEM, path: *c_char) -> fmod::Result;
    pub fn FMOD_System_LoadPlugin(system: FMOD_SYSTEM, filename: *c_char, handle: *c_uint, priority: c_uint) -> fmod::Result;
    pub fn FMOD_System_UnloadPlugin(system: FMOD_SYSTEM, handle: c_uint) -> fmod::Result;
    pub fn FMOD_System_GetNumPlugins(system: FMOD_SYSTEM, plugin_type: fmod::PluginType, num_plugins: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetPluginHandle(system: FMOD_SYSTEM, plugin_type: fmod::PluginType, index: c_int, handle: *c_uint) -> fmod::Result;
    pub fn FMOD_System_GetPluginInfo(system: FMOD_SYSTEM, handle: c_uint, plugin_type: *fmod::PluginType, name: *c_char,
        name_len: c_int, version: *c_uint) -> fmod::Result;
    pub fn FMOD_System_SetOutputByPlugin(system: FMOD_SYSTEM, handle: c_uint) -> fmod::Result;
    pub fn FMOD_System_GetOutputByPlugin(system: FMOD_SYSTEM, handle: *c_uint) -> fmod::Result;
    pub fn FMOD_System_CreateDSPByPlugin(system: FMOD_SYSTEM, handle: c_uint, dsp: *FMOD_DSP) -> fmod::Result;
    /* codec part functions */
    pub fn FMOD_System_RegisterCodec(system: FMOD_SYSTEM, description: *FMOD_CODEC_DESCRIPTION, handle: *c_uint, priority: c_uint) -> fmod::Result;
    /* init/close functions */
    pub fn FMOD_System_Init(system: FMOD_SYSTEM, max_channels: c_int, flags: FMOD_INITFLAGS, extra_driver_data: *c_void) -> fmod::Result;
    pub fn FMOD_System_Close(sound: FMOD_SOUND) -> fmod::Result;
    /* post-init functions */
    pub fn FMOD_System_Update(system: FMOD_SYSTEM) -> fmod::Result;
    pub fn FMOD_System_GetSpectrum(system: FMOD_SYSTEM, spectrum_array: *c_float, num_values: c_int, channel_offset: c_int,
        window_type: fmod::DSP_FFT_Window) -> fmod::Result;
    pub fn FMOD_System_GetWaveData(system: FMOD_SYSTEM, wave_array: *c_float, num_values: c_int, channel_offset: c_int) -> fmod::Result;
    pub fn FMOD_System_SetStreamBufferSize(system: FMOD_SYSTEM, file_buffer_size: c_uint, file_buffer_size_type: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_System_GetStreamBufferSize(system: FMOD_SYSTEM, file_buffer_size: *c_uint, file_buffer_size_type: *FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_System_Set3DNumListeners(system: FMOD_SYSTEM, num_listeners: c_int) -> fmod::Result;
    pub fn FMOD_System_Get3DNumListeners(system: FMOD_SYSTEM, num_listeners: *c_int) -> fmod::Result;
    pub fn FMOD_System_Set3DListenerAttributes(system: FMOD_SYSTEM, listener: c_int, pos: *FMOD_VECTOR, vel: *FMOD_VECTOR, forward: *FMOD_VECTOR,
        up: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_System_Get3DListenerAttributes(system: FMOD_SYSTEM, listener: c_int, pos: *FMOD_VECTOR, vel: *FMOD_VECTOR, forward: *FMOD_VECTOR,
        up: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_System_GetMemoryInfo(system: FMOD_SYSTEM, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memoryused_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;
    /* I'll bind it later */
    pub fn FMOD_System_Set3DRolloffCallback(system: FMOD_SYSTEM, callback: FMOD_3D_ROLLOFFCALLBACK) -> fmod::Result;
    pub fn FMOD_System_Set3DSpeakerPosition(system: FMOD_SYSTEM, speaker: fmod::Speaker, x: c_float, y: c_float, active: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_System_Get3DSpeakerPosition(system: FMOD_SYSTEM, speaker: fmod::Speaker, x: *c_float, y: *c_float, active: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_System_Set3DSettings(system: FMOD_SYSTEM, doppler_scale: c_float, distance_factor: c_float, roll_off_scale: c_float) -> fmod::Result;
    pub fn FMOD_System_Get3DSettings(system: FMOD_SYSTEM, doppler_scale: *c_float, distance_factor: *c_float, roll_off_scale: *c_float) -> fmod::Result;
    /* system information functions */
    pub fn FMOD_System_GetVersion(system: FMOD_SYSTEM, version: *c_uint) -> fmod::Result;
    pub fn FMOD_System_GetOutputHandle(system: FMOD_SYSTEM, handle: **c_void) -> fmod::Result;
    pub fn FMOD_System_GetChannelsPlaying(system: FMOD_SYSTEM, channels: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetCPUUsage(system: FMOD_SYSTEM, dsp: *c_float, stream: *c_float, geometry: *c_float, update: *c_float, total: *c_float) -> fmod::Result;
    pub fn FMOD_System_GetSoundRAM(system: FMOD_SYSTEM, current_alloced: *c_int, max_alloced: *c_int, total: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetNumCDROMDrives(system: FMOD_SYSTEM, num_drives: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetCDROMDriveName(system: FMOD_SYSTEM, drive: c_int, drive_name: *c_char, drive_name_len: c_int, scsi_name: *c_char, scsi_name_len: c_int,
        device_name: *c_char, device_name_len: c_int) -> fmod::Result;
    /* Sound/DSP/Channel/FX creation and retrieval. */
    pub fn FMOD_System_CreateSound(system: FMOD_SYSTEM, name_or_data: *c_char, mode: FMOD_MODE, exinfo: *FMOD_CREATESOUNDEXINFO,
        sound: *FMOD_SOUND) -> fmod::Result;
    pub fn FMOD_System_CreateStream(system: FMOD_SYSTEM, name_or_data: *c_char, mode: FMOD_MODE, exinfo: *FMOD_CREATESOUNDEXINFO,
        sound: *FMOD_SOUND) -> fmod::Result;
    pub fn FMOD_System_CreateChannelGroup(system: FMOD_SYSTEM, name: *c_char, channel_group: *FMOD_CHANNELGROUP) -> fmod::Result;
    pub fn FMOD_System_CreateSoundGroup(system: FMOD_SYSTEM, name: *c_char, sound_group: *FMOD_SOUNDGROUP) -> fmod::Result;
    pub fn FMOD_System_GetChannel(system: FMOD_SYSTEM, channel_id: c_int, channel: *FMOD_CHANNEL) -> fmod::Result;
    pub fn FMOD_System_GetMasterChannelGroup(system: FMOD_SYSTEM, channel_group: *FMOD_CHANNELGROUP) -> fmod::Result;
    pub fn FMOD_System_GetMasterSoundGroup(system: FMOD_SYSTEM, sound_group: *FMOD_SOUNDGROUP) -> fmod::Result;
    /* Reverb API */
    pub fn FMOD_System_SetReverbProperties(system: FMOD_SYSTEM, prop: *FMOD_REVERB_PROPERTIES) -> fmod::Result;
    pub fn FMOD_System_GetReverbProperties(system: FMOD_SYSTEM, prop: *FMOD_REVERB_PROPERTIES) -> fmod::Result;
    pub fn FMOD_System_SetReverbAmbientProperties(system: FMOD_SYSTEM, prop: *FMOD_REVERB_PROPERTIES) -> fmod::Result;
    pub fn FMOD_System_GetReverbAmbientProperties(system: FMOD_SYSTEM, prop: *FMOD_REVERB_PROPERTIES) -> fmod::Result;
    /* System level DSP access.*/
    pub fn FMOD_System_GetDSPHead(system: FMOD_SYSTEM, dsp: *FMOD_DSP) -> fmod::Result;
    pub fn FMOD_System_AddDSP(system: FMOD_SYSTEM, dsp: FMOD_DSP, connection: FMOD_DSPCONNECTION) -> fmod::Result;
    pub fn FMOD_System_LockDSP(system: FMOD_SYSTEM) -> fmod::Result;
    pub fn FMOD_System_UnlockDSP(system: FMOD_SYSTEM) -> fmod::Result;
    pub fn FMOD_System_GetDSPClock(system: FMOD_SYSTEM, hi: *c_uint, lo: *c_uint) -> fmod::Result;
    /* Recording API */
    pub fn FMOD_System_GetRecordNumDrivers(system: FMOD_SYSTEM, num_drivers: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetRecordDriverInfo(system: FMOD_SYSTEM, id: c_int, name: *c_char, name_len: c_int, guid: *FMOD_GUID) -> fmod::Result;
    /* I'll bind it later */
    pub fn FMOD_System_GetRecordDriverInfoW(system: FMOD_SYSTEM, id: c_int, name: *c_short, name_len: c_int, guid: *FMOD_GUID) -> fmod::Result;
    pub fn FMOD_System_GetRecordDriverCaps(system: FMOD_SYSTEM, id: c_int, caps: *FMOD_CAPS, min_frequency: *c_int, max_frequency: *c_int) -> fmod::Result;
    pub fn FMOD_System_GetRecordPosition(system: FMOD_SYSTEM, id: c_int, position: *c_uint) -> fmod::Result;
    pub fn FMOD_System_RecordStart(system: FMOD_SYSTEM, id: c_int, sound: FMOD_SOUND, _loop: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_System_RecordStop(system: FMOD_SYSTEM, id: c_int) -> fmod::Result;
    pub fn FMOD_System_IsRecording(system: FMOD_SYSTEM, id: c_int, recording: *FMOD_BOOL) -> fmod::Result;
    /* Geometry API. */
    pub fn FMOD_System_CreateGeometry(system: FMOD_SYSTEM, max_polygons: c_int, max_vertices: c_int, geometry: *FMOD_GEOMETRY) -> fmod::Result;
    pub fn FMOD_System_SetGeometrySettings(system: FMOD_SYSTEM, max_world_size: c_float) -> fmod::Result;
    pub fn FMOD_System_GetGeometrySettings(system: FMOD_SYSTEM, max_world_size: *c_float) -> fmod::Result;
    /* I'll bind it later */
    pub fn FMOD_System_LoadGeometry(system: FMOD_SYSTEM, data: *c_void, data_size: c_int, geometry: *FMOD_GEOMETRY) -> fmod::Result;
    pub fn FMOD_System_GetGeometryOcclusion(system: FMOD_SYSTEM, listener: *FMOD_VECTOR, source: *FMOD_VECTOR, direct: *c_float, reverb: *c_float) -> fmod::Result;
    /* Network functions.*/
    /* to add */

    /* sound functions */
    pub fn FMOD_System_PlaySound(system: FMOD_SYSTEM, channel_id: fmod::ChannelIndex, sound: FMOD_SOUND, paused: FMOD_BOOL,
        channel: *FMOD_CHANNEL) -> fmod::Result;
    pub fn FMOD_Sound_Release(sound: FMOD_SOUND) -> fmod::Result;
    /* Standard sound manipulation functions. */
    pub fn FMOD_Sound_Lock(sound: FMOD_SOUND, offset: c_uint, length: c_uint, ptr1: **c_void, ptr2: **c_void, len1: *c_uint, len2: *c_uint) -> fmod::Result;
    pub fn FMOD_Sound_Unlock(sound: FMOD_SOUND, ptr1: *c_void, ptr2: *c_void, len1: c_uint, len2: c_uint) -> fmod::Result;
    pub fn FMOD_Sound_GetSystemObject(sound: FMOD_SOUND, system: *FMOD_SYSTEM) -> fmod::Result;
    pub fn FMOD_Sound_SetDefaults(sound: FMOD_SOUND, frequency: c_float, volume: c_float, pan: c_float, priority: c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetDefaults(sound: FMOD_SOUND, frequency: *c_float, volume: *c_float, pan: *c_float, priority: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_SetVariations(sound: FMOD_SOUND, frequency_var: c_float, volume_var: c_float, pan_var: c_float) -> fmod::Result;
    pub fn FMOD_Sound_GetVariations(sound: FMOD_SOUND, frequency_var: *c_float, volume_var: *c_float, pan_var: *c_float) -> fmod::Result;
    pub fn FMOD_Sound_Set3DMinMaxDistance(sound: FMOD_SOUND, min: c_float, max: c_float) -> fmod::Result;
    pub fn FMOD_Sound_Get3DMinMaxDistance(sound: FMOD_SOUND, min: *c_float, max: *c_float) -> fmod::Result;
    pub fn FMOD_Sound_Set3DConeSettings(sound: FMOD_SOUND, inside_cone_angle: c_float, outside_cone_angle: c_float, outside_volume: c_float) -> fmod::Result;
    pub fn FMOD_Sound_Get3DConeSettings(sound: FMOD_SOUND, inside_cone_angle: *c_float, outside_cone_angle: *c_float, outside_volume: *c_float) -> fmod::Result;
    pub fn FMOD_Sound_Set3DCustomRolloff(sound: FMOD_SOUND, points: *FMOD_VECTOR, num_points: c_int) -> fmod::Result;
    pub fn FMOD_Sound_Get3DCustomRolloff(sound: FMOD_SOUND, points: **FMOD_VECTOR, num_points: c_int) -> fmod::Result;
    pub fn FMOD_Sound_SetSubSound(sound: FMOD_SOUND, index: c_int, sub_sound: FMOD_SOUND) -> fmod::Result;
    pub fn FMOD_Sound_GetSubSound(sound: FMOD_SOUND, index: c_int, sub_sound: *FMOD_SOUND) -> fmod::Result;
    pub fn FMOD_Sound_SetSubSoundSentence(sound: FMOD_SOUND, sub_sound_list: *c_int, num_sub_sound: c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetName(sound: FMOD_SOUND, name: *c_char, name_len: c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetLength(sound: FMOD_SOUND, length: *c_uint, length_type: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_Sound_GetFormat(sound: FMOD_SOUND, _type: *fmod::SoundType, format: *fmod::SoundFormat, channels: *c_int, bits: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetNumSubSounds(sound: FMOD_SOUND, num_sub_sound: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetNumTags(sound: FMOD_SOUND, num_tags: *c_int, num_tags_updated: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetTag(sound: FMOD_SOUND, name: *c_char, index: c_int, tag: *FMOD_TAG) -> fmod::Result;
    pub fn FMOD_Sound_GetOpenState(sound: FMOD_SOUND, open_state: *fmod::OpenState, percent_buffered: *c_uint, starving: *FMOD_BOOL,
        disk_busy: *FMOD_BOOL) -> fmod::Result;
    /* I'll bind it later */
    pub fn FMOD_Sound_ReadData(sound: FMOD_SOUND, buffer: *c_void, len_bytes: c_uint, read: *c_uint) -> fmod::Result;
    pub fn FMOD_Sound_SeekData(sound: FMOD_SOUND, pcm: c_uint) -> fmod::Result;
    pub fn FMOD_Sound_SetSoundGroup(sound: FMOD_SOUND, sound_group: FMOD_SOUNDGROUP) -> fmod::Result;
    pub fn FMOD_Sound_GetSoundGroup(sound: FMOD_SOUND, sound_group: *FMOD_SOUNDGROUP) -> fmod::Result;
    /* Synchronization point API.  These points can come from markers embedded in wav files, and can also generate channel callbacks. */
    pub fn FMOD_Sound_GetNumSyncPoints(sound: FMOD_SOUND, num_sync_points: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetSyncPoint(sound: FMOD_SOUND, index: c_int, point: *FMOD_SYNCPOINT) -> fmod::Result;
    pub fn FMOD_Sound_GetSyncPointInfo(sound: FMOD_SOUND, point: FMOD_SYNCPOINT, name: *c_char, name_len: c_int, offset: *c_uint,
        offset_type: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_Sound_AddSyncPoint(sound: FMOD_SOUND, offset: c_uint, offset_type: FMOD_TIMEUNIT, name: *c_char, point: *FMOD_SYNCPOINT) -> fmod::Result;
    pub fn FMOD_Sound_DeleteSyncPoint(sound: FMOD_SOUND, point: FMOD_SYNCPOINT) -> fmod::Result;
    /* Functions also in Channel class but here they are the 'default' to save having to change it in Channel all the time. */
    pub fn FMOD_Sound_SetMode(sound: FMOD_SOUND, mode: FMOD_MODE) -> fmod::Result;
    pub fn FMOD_Sound_GetMode(sound: FMOD_SOUND, mode: *FMOD_MODE) -> fmod::Result;
    pub fn FMOD_Sound_SetLoopCount(sound: FMOD_SOUND, loop_count: c_int) -> fmod::Result;
    pub fn FMOD_Sound_GetLoopCount(sound: FMOD_SOUND, loop_count: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_SetLoopPoints(sound: FMOD_SOUND, loop_start: c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: c_uint, loop_end_type: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_Sound_GetLoopPoints(sound: FMOD_SOUND, loop_start: *c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: *c_uint, loop_end_type: FMOD_TIMEUNIT) -> fmod::Result;
    /* For MOD/S3M/XM/IT/MID sequenced formats only. */
    pub fn FMOD_Sound_GetMusicNumChannels(sound: FMOD_SOUND, num_channels: *c_int) -> fmod::Result;
    pub fn FMOD_Sound_SetMusicChannelVolume(sound: FMOD_SOUND, channel: c_int, volume: c_float) -> fmod::Result;
    pub fn FMOD_Sound_GetMusicChannelVolume(sound: FMOD_SOUND, channel: c_int, volume: *c_float) -> fmod::Result;
    pub fn FMOD_Sound_SetMusicSpeed(sound: FMOD_SOUND, speed: c_float) -> fmod::Result;
    pub fn FMOD_Sound_GetMusicSpeed(sound: FMOD_SOUND, speed: *c_float) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_Sound_SetUserData(sound: FMOD_SOUND, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_Sound_GetUserData(sound: FMOD_SOUND, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_Sound_GetMemoryInfo(sound: FMOD_SOUND, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memory_used_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;


    /* channel functions */
    pub fn FMOD_Channel_GetSystemObject(channel: FMOD_CHANNEL, system: *FMOD_SYSTEM) -> fmod::Result;
    pub fn FMOD_Channel_Stop(channel: FMOD_CHANNEL) -> fmod::Result;
    pub fn FMOD_Channel_SetPaused(channel: FMOD_CHANNEL, pause: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Channel_GetPaused(channel: FMOD_CHANNEL, pause: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Channel_SetVolume(channel: FMOD_CHANNEL, volume: c_float) -> fmod::Result;
    pub fn FMOD_Channel_GetVolume(channel: FMOD_CHANNEL, volume: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_SetFrequency(channel: FMOD_CHANNEL, frequency: c_float) -> fmod::Result;
    pub fn FMOD_Channel_GetFrequency(channel: FMOD_CHANNEL, frequency: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_SetPan(channel: FMOD_CHANNEL, pan: c_float) -> fmod::Result;
    pub fn FMOD_Channel_GetPan(channel: FMOD_CHANNEL, pan: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_SetDelay(channel: FMOD_CHANNEL, delay_type: fmod::DelayType, delayhi: c_uint, delaylo: c_uint) -> fmod::Result;
    pub fn FMOD_Channel_GetDelay(channel: FMOD_CHANNEL, delay_type: fmod::DelayType, delayhi: *c_uint, delaylo: *c_uint) -> fmod::Result;
    pub fn FMOD_Channel_SetSpeakerMix(channel: FMOD_CHANNEL, front_left: c_float, front_right: c_float, center: c_float, lfe: c_float,
        back_left: c_float, back_right: c_float, side_left: c_float, side_right: c_float) -> fmod::Result;
    pub fn FMOD_Channel_GetSpeakerMix(channel: FMOD_CHANNEL, front_left: *c_float, front_right: *c_float, center: *c_float, lfe: *c_float,
        back_left: *c_float, back_right: *c_float, side_left: *c_float, side_right: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_SetSpeakerLevels(channel: FMOD_CHANNEL, speaker: fmod::Speaker, levels: *c_float, num_levels: c_int) -> fmod::Result;
    pub fn FMOD_Channel_GetSpeakerLevels(channel: FMOD_CHANNEL, speaker: fmod::Speaker, levels: *c_float, num_levels: c_int) -> fmod::Result;
    pub fn FMOD_Channel_SetInputChannelMix(channel: FMOD_CHANNEL, levels: *c_float, num_levels: c_int) -> fmod::Result;
    pub fn FMOD_Channel_GetInputChannelMix(channel: FMOD_CHANNEL, levels: *c_float, num_levels: c_int) -> fmod::Result;
    pub fn FMOD_Channel_SetMute(channel: FMOD_CHANNEL, mute: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Channel_GetMute(channel: FMOD_CHANNEL, mute: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Channel_SetPriority(channel: FMOD_CHANNEL, priority: c_int) -> fmod::Result;
    pub fn FMOD_Channel_GetPriority(channel: FMOD_CHANNEL, priority: *c_int) -> fmod::Result;
    pub fn FMOD_Channel_SetPosition(channel: FMOD_CHANNEL, position: c_uint, postype: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_Channel_GetPosition(channel: FMOD_CHANNEL, position: *c_uint, postype: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_Channel_SetReverbProperties(channel: FMOD_CHANNEL, prop: *FMOD_REVERB_CHANNELPROPERTIES) -> fmod::Result;
    pub fn FMOD_Channel_GetReverbProperties(channel: FMOD_CHANNEL, prop: *FMOD_REVERB_CHANNELPROPERTIES) -> fmod::Result;
    pub fn FMOD_Channel_SetLowPassGain(channel: FMOD_CHANNEL, gain: c_float) -> fmod::Result;
    pub fn FMOD_Channel_GetLowPassGain(channel: FMOD_CHANNEL, gain: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_SetChannelGroup(channel: FMOD_CHANNEL, channelgroup: FMOD_CHANNELGROUP) -> fmod::Result;
    pub fn FMOD_Channel_GetChannelGroup(channel: FMOD_CHANNEL, channelgroup: *FMOD_CHANNELGROUP) -> fmod::Result;
    /* I'll bint it later */
    //pub fn FMOD_Channel_SetCallback(channel: FMOD_CHANNEL, callback: FMOD_CHANNEL_CALLBACK) -> fmod::Result;
    /* 3D functionality */
    pub fn FMOD_Channel_Set3DAttributes(channel: FMOD_CHANNEL, position: *FMOD_VECTOR, velociy: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Channel_Get3DAttributes(channel: FMOD_CHANNEL, position: *FMOD_VECTOR, velociy: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Channel_Set3DMinMaxDistance(channel: FMOD_CHANNEL, min_distance: c_float, max_distance: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DMinMaxDistance(channel: FMOD_CHANNEL, min_distance: *c_float, max_distance: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_Set3DConeSettings(channel: FMOD_CHANNEL, inside_cone_angle: c_float, outside_cone_angle: c_float,
        outside_volume: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DConeSettings(channel: FMOD_CHANNEL, inside_cone_angle: *c_float, outside_cone_angle: *c_float,
        outside_volume: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_Set3DConeOrientation(channel: FMOD_CHANNEL, orientation: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Channel_Get3DConeOrientation(channel: FMOD_CHANNEL, orientation: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Channel_Set3DCustomRolloff(channel: FMOD_CHANNEL, points: *FMOD_VECTOR, num_points: c_int) -> fmod::Result;
    pub fn FMOD_Channel_Get3DCustomRolloff(channel: FMOD_CHANNEL, points: **FMOD_VECTOR, num_points: *c_int) -> fmod::Result;
    pub fn FMOD_Channel_Set3DOcclusion(channel: FMOD_CHANNEL, direct_occlusion: c_float, reverb_occlusion: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DOcclusion(channel: FMOD_CHANNEL, direct_occlusion: *c_float, reverb_occlusion: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_Set3DSpread(channel: FMOD_CHANNEL, angle: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DSpread(channel: FMOD_CHANNEL, angle: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_Set3DPanLevel(channel: FMOD_CHANNEL, level: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DPanLevel(channel: FMOD_CHANNEL, level: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_Set3DDopplerLevel(channel: FMOD_CHANNEL, level: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DDopplerLevel(channel: FMOD_CHANNEL, level: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_Set3DDistanceFilter(channel: FMOD_CHANNEL, custom: FMOD_BOOL, custom_level: c_float, center_freq: c_float) -> fmod::Result;
    pub fn FMOD_Channel_Get3DDistanceFilter(channel: FMOD_CHANNEL, custom: *FMOD_BOOL, custom_level: *c_float, center_freq: *c_float) -> fmod::Result;
    /* Information only functions */
    pub fn FMOD_Channel_IsPlaying(channel: FMOD_CHANNEL, is_playing: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Channel_IsVirtual(channel: FMOD_CHANNEL, is_virtual: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Channel_GetAudibility(channel: FMOD_CHANNEL, audibility: *c_float) -> fmod::Result;
    pub fn FMOD_Channel_GetCurrentSound(channel: FMOD_CHANNEL, sound: *FMOD_SOUND) -> fmod::Result;
    pub fn FMOD_Channel_GetSpectrum(channel: FMOD_CHANNEL, spectrum_array: *c_float, num_values: c_int, channel_offset: c_int,
        window_type: fmod::DSP_FFT_Window) -> fmod::Result;
    pub fn FMOD_Channel_GetWaveData(channel: FMOD_CHANNEL, wave_array: *c_float, num_values: c_int, channel_offset: c_int) -> fmod::Result;
    pub fn FMOD_Channel_GetIndex(channel: FMOD_CHANNEL, index: *c_int) -> fmod::Result;
    /* DSP functionality only for channels playing sounds created with FMOD_SOFTWARE */
    pub fn FMOD_Channel_GetDSPHead(channel: FMOD_CHANNEL, dsp: *FMOD_DSP) -> fmod::Result;
    pub fn FMOD_Channel_AddDSP(channel: FMOD_CHANNEL, dsp: FMOD_DSP, connection: *FMOD_DSPCONNECTION) -> fmod::Result;
    //-> /* Functions also found in Sound class but here they can be set per channel */
    pub fn FMOD_Channel_SetMode(channel: FMOD_CHANNEL, mode: FMOD_MODE) -> fmod::Result;
    pub fn FMOD_Channel_GetMode(channel: FMOD_CHANNEL, mode: *FMOD_MODE) -> fmod::Result;
    pub fn FMOD_Channel_SetLoopCount(channel: FMOD_CHANNEL, loop_count: c_int) -> fmod::Result;
    pub fn FMOD_Channel_GetLoopCount(channel: FMOD_CHANNEL, loop_count: *c_int) -> fmod::Result;
    pub fn FMOD_Channel_SetLoopPoints(channel: FMOD_CHANNEL, loop_start: c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: c_uint,
        loop_end_type: FMOD_TIMEUNIT) -> fmod::Result;
    pub fn FMOD_Channel_GetLoopPoints(channel: FMOD_CHANNEL, loop_start: *c_uint, loop_start_type: FMOD_TIMEUNIT, loop_end: *c_uint,
        loop_end_type: FMOD_TIMEUNIT) -> fmod::Result;
    /* Userdata set/get */
    pub fn FMOD_Channel_SetUserData(channel: FMOD_CHANNEL, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_Channel_GetUserData(channel: FMOD_CHANNEL, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_Channel_GetMemoryInfo(channel_group: FMOD_CHANNELGROUP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memoryused_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;

    
    /* channel_group functions */
    pub fn FMOD_ChannelGroup_Release(channel_group: FMOD_CHANNELGROUP) -> fmod::Result;
    /* Channelgroup scale values.  (changes attributes relative to the channels, doesn't overwrite them)*/
    pub fn FMOD_ChannelGroup_SetVolume(channel_group: FMOD_CHANNELGROUP, volume: c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetVolume(channel_group: FMOD_CHANNELGROUP, volume: *c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_SetPitch(channel_group: FMOD_CHANNELGROUP, pitch: c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetPitch(channel_group: FMOD_CHANNELGROUP, pitch: *c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_Set3DOcclusion(channel_group: FMOD_CHANNELGROUP, direct_occlusion: c_float, reverb_occlusion: c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_Get3DOcclusion(channel_group: FMOD_CHANNELGROUP, direct_occlusion: *c_float, reverb_occlusion: *c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_SetPaused(channel_group: FMOD_CHANNELGROUP, paused: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetPaused(channel_group: FMOD_CHANNELGROUP, paused: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_ChannelGroup_SetMute(channel_group: FMOD_CHANNELGROUP, mute: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetMute(channel_group: FMOD_CHANNELGROUP, mute: *FMOD_BOOL) -> fmod::Result;
    /* Channelgroup override values.  (recursively overwrites whatever settings the channels had) */
    pub fn FMOD_ChannelGroup_Stop(channel_group: FMOD_CHANNELGROUP) -> fmod::Result;
    pub fn FMOD_ChannelGroup_OverrideVolume(channel_group: FMOD_CHANNELGROUP, volume: c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_OverrideFrequency(channel_group: FMOD_CHANNELGROUP, frequency: c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_OverridePan(channel_group: FMOD_CHANNELGROUP, pan: c_float) -> fmod::Result;
    pub fn FMOD_ChannelGroup_OverrideReverbProperties(channel_group: FMOD_CHANNELGROUP, prop: *FMOD_REVERB_CHANNELPROPERTIES) -> fmod::Result;
    pub fn FMOD_ChannelGroup_Override3DAttributes(channel_group: FMOD_CHANNELGROUP, pos: *FMOD_VECTOR, vel: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_ChannelGroup_OverrideSpeakerMix(channel_group: FMOD_CHANNELGROUP, front_left: c_float, front_right: c_float, center: c_float, lfe: c_float,
        back_left: c_float, back_right: c_float, side_left: c_float, side_right: c_float) -> fmod::Result;
    /* Nested channel groups.*/
    pub fn FMOD_ChannelGroup_AddGroup(channel_group: FMOD_CHANNELGROUP, group: FMOD_CHANNELGROUP) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetNumGroups(channel_group: FMOD_CHANNELGROUP, num_groups: *c_int) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetGroup(channel_group: FMOD_CHANNELGROUP, index: c_int, group: *FMOD_CHANNELGROUP) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetParentGroup(channel_group: FMOD_CHANNELGROUP, group: *FMOD_CHANNELGROUP) -> fmod::Result;
    /* DSP functionality only for channel groups playing sounds created with FMOD_SOFTWARE. */
    pub fn FMOD_ChannelGroup_GetDSPHead(channel_group: FMOD_CHANNELGROUP, dsp: *FMOD_DSP) -> fmod::Result;
    pub fn FMOD_ChannelGroup_AddDSP(channel_group: FMOD_CHANNELGROUP, dsp: FMOD_DSP, disp_connection: *FMOD_DSPCONNECTION) -> fmod::Result;
    /* Information only functions. */
    pub fn FMOD_ChannelGroup_GetName(channel_group: FMOD_CHANNELGROUP, name: *c_char, name_len: c_int) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetNumChannels(channel_group: FMOD_CHANNELGROUP, num_channels: *c_int) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetChannel(channel_group: FMOD_CHANNELGROUP, index: c_int, channel: *FMOD_CHANNEL) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetSpectrum(channel_group: FMOD_CHANNELGROUP, spectrum_array: *c_float, num_values: c_int, channel_offset: c_int,
        window_type: fmod::DSP_FFT_Window) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetWaveData(channel_group: FMOD_CHANNELGROUP, wave_array: *c_float, num_values: c_int, channel_offset: c_int) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_ChannelGroup_SetUserData(channel_group: FMOD_CHANNELGROUP, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetUserData(channel_group: FMOD_CHANNELGROUP, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_ChannelGroup_GetMemoryInfo(channel_group: FMOD_CHANNELGROUP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memoryused_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;


    /* sound_group functions */
    pub fn FMOD_SoundGroup_Release(sound_group: FMOD_SOUNDGROUP) -> fmod::Result;
    /* SoundGroup control functions.*/
    pub fn FMOD_SoundGroup_SetMaxAudible(sound_group: FMOD_SOUNDGROUP, max_audible: c_int) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetMaxAudible(sound_group: FMOD_SOUNDGROUP, max_audible: *c_int) -> fmod::Result;
    pub fn FMOD_SoundGroup_SetMaxAudibleBehavior(sound_group: FMOD_SOUNDGROUP, behavior: fmod::SoundGroupBehavior) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetMaxAudibleBehavior(sound_group: FMOD_SOUNDGROUP, behavior: *fmod::SoundGroupBehavior) -> fmod::Result;
    pub fn FMOD_SoundGroup_SetMuteFadeSpeed(sound_group: FMOD_SOUNDGROUP, speed: c_float) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetMuteFadeSpeed(sound_group: FMOD_SOUNDGROUP, speed: *c_float) -> fmod::Result;
    pub fn FMOD_SoundGroup_SetVolume(sound_group: FMOD_SOUNDGROUP, volume: c_float) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetVolume(sound_group: FMOD_SOUNDGROUP, volume: *c_float) -> fmod::Result;
    pub fn FMOD_SoundGroup_Stop(sound_group: FMOD_SOUNDGROUP) -> fmod::Result;
    /* Information only functions. */
    pub fn FMOD_SoundGroup_GetName(sound_group: FMOD_SOUNDGROUP, name: *c_char, name_len: c_int) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetNumSounds(sound_group: FMOD_SOUNDGROUP, num_sounds: *c_int) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetSound(sound_group: FMOD_SOUNDGROUP, index: c_int, sound: *FMOD_SOUND) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetNumPlaying(sound_group: FMOD_SOUNDGROUP, num_playing: *c_int) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_SoundGroup_SetUserData(sound_group: FMOD_SOUNDGROUP, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetUserData(sound_group: FMOD_SOUNDGROUP, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_SoundGroup_GetMemoryInfo(sound_group: FMOD_SOUNDGROUP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memoryused_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;


    /* Dsp functions */
    pub fn FMOD_DSP_Release(dsp: FMOD_DSP) -> fmod::Result;
    /* Connection / disconnection / input and output enumeration. */
    pub fn FMOD_DSP_AddInput(dsp: FMOD_DSP, target: FMOD_DSP, connection: *FMOD_DSPCONNECTION) -> fmod::Result;
    pub fn FMOD_DSP_DisconnectFrom(dsp: FMOD_DSP, target: FMOD_DSP) -> fmod::Result;
    pub fn FMOD_DSP_DisconnectAll(dsp: FMOD_DSP, inputs: FMOD_BOOL, outputs: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_Remove(dsp: FMOD_DSP) -> fmod::Result;
    pub fn FMOD_DSP_GetNumInputs(dsp: FMOD_DSP, num_inputs: *c_int) -> fmod::Result;
    pub fn FMOD_DSP_GetNumOutputs(dsp: FMOD_DSP, num_outputs: *c_int) -> fmod::Result;
    pub fn FMOD_DSP_GetInput(dsp: FMOD_DSP, index: c_int, input: *FMOD_DSP, input_connection: *FMOD_DSPCONNECTION) -> fmod::Result;
    pub fn FMOD_DSP_GetOutput(dsp: FMOD_DSP, index: c_int, output: *FMOD_DSP, output_connection: *FMOD_DSPCONNECTION) -> fmod::Result;
    /* DSP unit control. */
    pub fn FMOD_DSP_SetActive(dsp: FMOD_DSP, active: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_GetActive(dsp: FMOD_DSP, active: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_SetBypass(dsp: FMOD_DSP, bypass: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_GetBypass(dsp: FMOD_DSP, bypass: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_SetSpeakerActive(dsp: FMOD_DSP, speaker: fmod::Speaker, active: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_GetSpeakerActive(dsp: FMOD_DSP, speaker: fmod::Speaker, active: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_DSP_Reset(dsp: FMOD_DSP) -> fmod::Result;
    /* DSP parameter control. */
    pub fn FMOD_DSP_SetParameter(dsp: FMOD_DSP, index: c_int, value: c_float) -> fmod::Result;
    pub fn FMOD_DSP_GetParameter(dsp: FMOD_DSP, index: c_int, value: *c_float, value_str: *c_char, value_str_len: c_int) -> fmod::Result;
    pub fn FMOD_DSP_GetNumParameters(dsp: FMOD_DSP, num_params: *c_int) -> fmod::Result;
    pub fn FMOD_DSP_GetParameterInfo(dsp: FMOD_DSP, index: c_int, name: *c_char, label: *c_char, description: *c_char, description_len: c_int, min: *c_float,
        max: *c_float) -> fmod::Result;
    /* I'll bind it later */
    pub fn FMOD_DSP_ShowConfigDialog(dsp: FMOD_DSP, hwnd: *c_void, show: FMOD_BOOL) -> fmod::Result;
    /* DSP attributes. */
    pub fn FMOD_DSP_GetInfo(dsp: FMOD_DSP, name: *c_char, version: *c_uint, channels: *c_int, config_width: *c_int, config_height: *c_int) -> fmod::Result;
    /* I'll bind it later */
    //pub fn FMOD_DSP_GetType(dsp: FMOD_DSP, _type: *FMOD_DSP_TYPE) -> fmod::Result;
    pub fn FMOD_DSP_SetDefaults(dsp: FMOD_DSP, frequency: c_float, volume: c_float, pan: c_float, priority: c_int) -> fmod::Result;
    pub fn FMOD_DSP_GetDefaults(dsp: FMOD_DSP, frequency: *c_float, volume: *c_float, pan: *c_float, priority: *c_int) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_DSP_SetUserData(dsp: FMOD_DSP, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_DSP_GetUserData(dsp: FMOD_DSP, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_DSP_GetMemoryInfo(dsp: FMOD_DSP, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memory_used_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;


    /* DspConnection functions */
    pub fn FMOD_DSPConnection_GetInput(dsp_connection: FMOD_DSPCONNECTION, input: *FMOD_DSP) -> fmod::Result;
    pub fn FMOD_DSPConnection_GetOutput(dsp_connection: FMOD_DSPCONNECTION, output: *FMOD_DSP) -> fmod::Result;
    pub fn FMOD_DSPConnection_SetMix(dsp_connection: FMOD_DSPCONNECTION, volume: c_float) -> fmod::Result;
    pub fn FMOD_DSPConnection_GetMix(dsp_connection: FMOD_DSPCONNECTION, volume: *c_float) -> fmod::Result;
    pub fn FMOD_DSPConnection_SetLevels(dsp_connection: FMOD_DSPCONNECTION, speaker: fmod::Speaker, levels: *c_float, num_levels: c_int) -> fmod::Result;
    pub fn FMOD_DSPConnection_GetLevels(dsp_connection: FMOD_DSPCONNECTION, speaker: fmod::Speaker, levels: *c_float, num_levels: c_int) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_DSPCONNECTION_SetUserData(dsp_connection: FMOD_DSPCONNECTION, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_DSPCONNECTION_GetUserData(dsp_connection: FMOD_DSPCONNECTION, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_DSPCONNECTION_GetMemoryInfo(dsp_connection: FMOD_DSPCONNECTION, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memory_used_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;


    /* geometry functions */
    pub fn FMOD_Geometry_Release(geometry: FMOD_GEOMETRY) -> fmod::Result;
    /* Polygon manipulation. */
    pub fn FMOD_Geometry_AddPolygon(geometry: FMOD_GEOMETRY, direct_occlusion: c_float, reverb_occlusion: c_float, double_sided: FMOD_BOOL, num_vertices: c_int,
        vertices: *FMOD_VECTOR, polygon_index: *c_int) -> fmod::Result;
    pub fn FMOD_Geometry_GetNumPolygons(geometry: FMOD_GEOMETRY, num_polygons: *c_int) -> fmod::Result;
    pub fn FMOD_Geometry_GetMaxPolygons(geometry: FMOD_GEOMETRY, max_polygons: *c_int, max_vertices: *c_int) -> fmod::Result;
    pub fn FMOD_Geometry_GetPolygonNumVertices(geometry: FMOD_GEOMETRY, index: c_int, num_vertices: *c_int) -> fmod::Result;
    pub fn FMOD_Geometry_SetPolygonVertex(geometry: FMOD_GEOMETRY, index: c_int, vertex_index: c_int, vertex: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_GetPolygonVertex(geometry: FMOD_GEOMETRY, index: c_int, vertex_index: c_int, vertex: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_SetPolygonAttributes(geometry: FMOD_GEOMETRY, index: c_int, direct_occlusion: c_float, reverb_occlusion: c_float,
        double_sided: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Geometry_GetPolygonAttributes(geometry: FMOD_GEOMETRY, index: c_int, direct_occlusion: *c_float, reverb_occlusion: *c_float,
        double_sided: *FMOD_BOOL) -> fmod::Result;
    /* Object manipulation. */
    pub fn FMOD_Geometry_SetActive(geometry: FMOD_GEOMETRY, active: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Geometry_GetActive(geometry: FMOD_GEOMETRY, active: *FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Geometry_SetRotation(geometry: FMOD_GEOMETRY, forward: *FMOD_VECTOR, up: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_GetRotation(geometry: FMOD_GEOMETRY, forward: *FMOD_VECTOR, up: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_SetPosition(geometry: FMOD_GEOMETRY, position: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_GetPosition(geometry: FMOD_GEOMETRY, position: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_SetScale(geometry: FMOD_GEOMETRY, scale: *FMOD_VECTOR) -> fmod::Result;
    pub fn FMOD_Geometry_GetScale(geometry: FMOD_GEOMETRY, scale: *FMOD_VECTOR) -> fmod::Result;
    /* I'll bind it later */
    pub fn FMOD_Geometry_Save(geometry: FMOD_GEOMETRY, data: *c_void, data_size: c_int) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_Geometry_SetUserData(geometry: FMOD_GEOMETRY, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_Geometry_GetUserData(geometry: FMOD_GEOMETRY, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_Geometry_GetMemoryInfo(geometry: FMOD_GEOMETRY, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memory_used_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;


    /* reverb function */
    pub fn FMOD_Reverb_Release(reverb: FMOD_REVERB) -> fmod::Result;
    pub fn FMOD_Reverb_Set3DAttributes(reverb: FMOD_REVERB, position: *FMOD_VECTOR, min_distance: c_float, max_distance: c_float) -> fmod::Result;
    pub fn FMOD_Reverb_Get3DAttributes(reverb: FMOD_REVERB, position: *FMOD_VECTOR, min_distance: *c_float, max_distance: *c_float) -> fmod::Result;
    pub fn FMOD_Reverb_SetProperties(reverb: FMOD_REVERB, properties: *FMOD_REVERB_PROPERTIES) -> fmod::Result;
    pub fn FMOD_Reverb_GetProperties(reverb: FMOD_REVERB, properties: *FMOD_REVERB_PROPERTIES) -> fmod::Result;
    pub fn FMOD_Reverb_SetActive(reverb: FMOD_REVERB, active: FMOD_BOOL) -> fmod::Result;
    pub fn FMOD_Reverb_GetActive(reverb: FMOD_REVERB, active: *FMOD_BOOL) -> fmod::Result;
    /* Userdata set/get. */
    pub fn FMOD_Reverb_SetUserData(reverb: FMOD_REVERB, user_data: *c_void) -> fmod::Result;
    pub fn FMOD_Reverb_GetUserData(reverb: FMOD_REVERB, user_data: **c_void) -> fmod::Result;
    pub fn FMOD_Reverb_GetMemoryInfo(reverb: FMOD_REVERB, memory_bits: c_uint, event_memory_bits: c_uint, memory_used: *c_uint,
        memory_used_details: *FMOD_MEMORY_USAGE_DETAILS) -> fmod::Result;
}

pub struct FMOD_ASYNCREADINFO
{
    pub handle     : *c_void,      /* [r] The file handle that was filled out in the open callback. */
    pub offset     : c_uint,       /* [r] Seek position, make sure you read from this file offset. */
    pub sizebytes  : c_uint,       /* [r] how many bytes requested for read. */
    pub priority   : c_int,        /* [r] 0 = low importance.  100 = extremely important (ie 'must read now or stuttering may occur') */

    pub buffer     : *c_void,      /* [w] Buffer to read file data into. */
    pub bytesread  : c_uint,       /* [w] Fill this in before setting result code to tell FMOD how many bytes were read. */
    pub result     : fmod::Result, /* [r/w] Result code, fmod::Ok tells the system it is ready to consume the data.  Set this last!  Default value = FMOD_ERR_NOTREADY. */
    pub userdata   : *c_void       /* [r] User data pointer. */
}

pub struct FMOD_CREATESOUNDEXINFO
{
    pub cbsize             : c_int,                        /* [w] Size of this structure.  This is used so the structure can be expanded in the future and still work on older versions of FMOD Ex. */
    pub length             : c_uint,                       /* [w] Optional. Specify 0 to ignore. Size in bytes of file to load, or sound to create (in this case only if FMOD_OPENUSER is used).  Required if loading from memory.  If 0 is specified, then it will use the size of the file (unless loading from memory then an error will be returned). */
    pub fileoffset         : c_uint,                       /* [w] Optional. Specify 0 to ignore. Offset from start of the file to start loading from.  This is useful for loading files from inside big data files. */
    pub numchannels        : c_int,                        /* [w] Optional. Specify 0 to ignore. Number of channels in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. */
    pub defaultfrequency   : c_int,                        /* [w] Optional. Specify 0 to ignore. Default frequency of sound in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the frequency determined by the file format. */
    pub format             : fmod::SoundFormat,            /* [w] Optional. Specify 0 or fmod::SoundFormatNone to ignore. Format of the sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the format determined by the file format.   */
    pub decodebuffersize   : c_uint,                       /* [w] Optional. Specify 0 to ignore. For streams.  This determines the size of the double buffer (in PCM samples) that a stream uses.  Use this for user created streams if you want to determine the size of the callback buffer passed to you.  Specify 0 to use FMOD's default size which is currently equivalent to 400ms of the sound format created/loaded. */
    pub initialsubsound    : c_int,                        /* [w] Optional. Specify 0 to ignore. In a multi-sample file format such as .FSB/.DLS/.SF2, specify the initial subsound to seek to, only if FMOD_CREATESTREAM is used. */
    pub numsubsounds       : c_int,                        /* [w] Optional. Specify 0 to ignore or have no subsounds.  In a sound created with FMOD_OPENUSER, specify the number of subsounds that are accessable with Sound::getSubSound.  If not created with FMOD_OPENUSER, this will limit the number of subsounds loaded within a multi-subsound file.  If using FSB, then if FMOD_CREATESOUNDEXINFO::inclusionlist is used, this will shuffle subsounds down so that there are not any gaps.  It will mean that the indices of the sounds will be different. */
    pub inclusionlist      : *c_int,                       /* [w] Optional. Specify 0 to ignore. In a multi-sample format such as .FSB/.DLS/.SF2 it may be desirable to specify only a subset of sounds to be loaded out of the whole file.  This is an array of subsound indices to load into memory when created. */
    pub inclusionlistnum   : c_int,                        /* [w] Optional. Specify 0 to ignore. This is the number of integers contained within the inclusionlist array. */
    pub pcmreadcallback    : FMOD_SOUND_PCMREADCALLBACK,   /* [w] Optional. Specify 0 to ignore. Callback to 'piggyback' on FMOD's read functions and accept or even write PCM data while FMOD is opening the sound.  Used for user sounds created with FMOD_OPENUSER or for capturing decoded data as FMOD reads it. */
    pub pcmsetposcallback  : FMOD_SOUND_PCMSETPOSCALLBACK, /* [w] Optional. Specify 0 to ignore. Callback for when the user calls a seeking function such as Channel::setTime or Channel::setPosition within a multi-sample sound, and for when it is opened.*/
    pub nonblockcallback   : FMOD_SOUND_NONBLOCKCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for successful completion, or error while loading a sound that used the FMOD_NONBLOCKING flag.  Also called duing seeking, when setPosition is called or a stream is restarted. */
    pub dlsname            : *c_char,                      /* [w] Optional. Specify 0 to ignore. Filename for a DLS or SF2 sample set when loading a MIDI file. If not specified, on Windows it will attempt to open /windows/system32/drivers/gm.dls or /windows/system32/drivers/etc/gm.dls, on Mac it will attempt to load /System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls, otherwise the MIDI will fail to open. Current DLS support is for level 1 of the specification. */
    pub encryptionkey      : *c_char,                      /* [w] Optional. Specify 0 to ignore. Key for encrypted FSB file.  Without this key an encrypted FSB file will not load. */
    pub maxpolyphony       : c_int,                        /* [w] Optional. Specify 0 to ignore. For sequenced formats with dynamic channel allocation such as .MID and .IT, this specifies the maximum voice count allowed while playing.  .IT defaults to 64.  .MID defaults to 32. */
    pub userdata           : *c_void,                      /* [w] Optional. Specify 0 to ignore. This is user data to be attached to the sound during creation.  Access via Sound::getUserData.  Note: This is not passed to FMOD_FILE_OPENCALLBACK, that is a different userdata that is file specific. */
    pub suggestedsoundtype : fmod::SoundType,              /* [w] Optional. Specify 0 or fmod::SoundTypeUnknown to ignore.  Instead of scanning all codec types, use this to speed up loading by making it jump straight to this codec. */
    pub useropen           : FMOD_FILE_OPENCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for opening this file. */
    pub userclose          : FMOD_FILE_CLOSECALLBACK,      /* [w] Optional. Specify 0 to ignore. Callback for closing this file. */
    pub userread           : FMOD_FILE_READCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for reading from this file. */
    pub userseek           : FMOD_FILE_SEEKCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub userasyncread      : FMOD_FILE_ASYNCREADCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub userasynccancel    : FMOD_FILE_ASYNCCANCELCALLBACK,/* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub speakermap         : fmod::SpeakerMapType,         /* [w] Optional. Specify 0 to ignore. Use this to differ the way fmod maps multichannel sounds to speakers.  See fmod::SpeakerMapType for more. */
    pub initialsoundgroup  : FMOD_SOUNDGROUP,              /* [w] Optional. Specify 0 to ignore. Specify a sound group if required, to put sound in as it is created. */
    pub initialseekposition: c_uint,                       /* [w] Optional. Specify 0 to ignore. For streams. Specify an initial position to seek the stream to. */
    pub initialseekpostype : FMOD_TIMEUNIT,                /* [w] Optional. Specify 0 to ignore. For streams. Specify the time unit for the position set in initialseekposition. */
    pub ignoresetfilesystem: c_int,                        /* [w] Optional. Specify 0 to ignore. Set to 1 to use fmod's built in file system. Ignores setFileSystem callbacks and also FMOD_CREATESOUNEXINFO file callbacks.  Useful for specific cases where you don't want to use your own file system but want to use fmod's file system (ie net streaming). */
    pub cddaforceaspi      : c_int,                        /* [w] Optional. Specify 0 to ignore. For CDDA sounds only - if non-zero use ASPI instead of NTSCSI to access the specified CD/DVD device. */
    pub audioqueuepolicy   : c_uint,                       /* [w] Optional. Specify 0 or FMOD_AUDIOQUEUE_CODECPOLICY_DEFAULT to ignore. Policy used to determine whether hardware or software is used for decoding, see FMOD_AUDIOQUEUE_CODECPOLICY for options (iOS >= 3.0 required, otherwise only hardware is available) */ 
    pub minmidigranularity : c_uint,                       /* [w] Optional. Specify 0 to ignore. Allows you to set a minimum desired MIDI mixer granularity. Values smaller than 512 give greater than default accuracy at the cost of more CPU and vice versa. Specify 0 for default (512 samples). */
    pub nonblockthreadid   : c_int                         /* [w] Optional. Specify 0 to ignore. Specifies a thread index to execute non blocking load on.  Allows for up to 5 threads to be used for loading at once.  This is to avoid one load blocking another.  Maximum value = 4. */
}

pub struct FMOD_REVERB_CHANNELPROPERTIES
{                                           /*       MIN    MAX  DEFAULT  DESCRIPTION */
    pub Direct         : c_int,            /* [r/w] -10000 1000 0        Direct path level                                        (SUPPORTED:SFX) */
    pub Room           : c_int,            /* [r/w] -10000 1000 0        Room effect level                                        (SUPPORTED:SFX) */
    pub Flags          : c_uint,           /* [r/w] FMOD_REVERB_CHANNELFLAGS - modifies the behavior of properties                (SUPPORTED:SFX) */
    pub ConnectionPoint: FMOD_DSP          /* [r/w] See remarks.         DSP network location to connect reverb for this channel. (SUPPORTED:SFX).*/
}

pub struct FMOD_GUID
{
    pub Data1: c_uint,              /* Specifies the first 8 hexadecimal digits of the GUID */
    pub Data2: c_ushort,            /* Specifies the first group of 4 hexadecimal digits.   */
    pub Data3: c_ushort,            /* Specifies the second group of 4 hexadecimal digits.  */
    pub Data4: [c_uchar, ..8]       /* Array of 8 bytes. The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits. */
}

pub struct FMOD_ADVANCEDSETTINGS
{
    pub cbsize                     : c_int,         /* [w]   Size of this structure.  Use sizeof(FMOD_ADVANCEDSETTINGS)  NOTE: This must be set before calling System::getAdvancedSettings! */
    pub maxMPEGcodecs              : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  Mpeg  codecs consume 21,684 bytes per instance and this number will determine how many mpeg channels can be played simultaneously.   Default = 32. */
    pub maxADPCMcodecs             : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  ADPCM codecs consume  2,136 bytes per instance and this number will determine how many ADPCM channels can be played simultaneously.  Default = 32. */
    pub maxXMAcodecs               : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  XMA   codecs consume 14,836 bytes per instance and this number will determine how many XMA channels can be played simultaneously.    Default = 32. */
    pub maxCELTcodecs              : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  CELT  codecs consume 11,500 bytes per instance and this number will determine how many CELT channels can be played simultaneously.   Default = 32. */    
    pub maxVORBIScodecs            : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  Vorbis codecs consume 12,000 bytes per instance and this number will determine how many Vorbis channels can be played simultaneously. Default = 32. */    
    pub maxAT9Codecs               : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  AT9 codecs consume  8,720 bytes per instance and this number will determine how many AT9 channels can be played simultaneously. Default = 32. */    
    pub maxPCMcodecs               : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with PS3 only.                          PCM   codecs consume 12,672 bytes per instance and this number will determine how many streams and PCM voices can be played simultaneously. Default = 16. */
    pub ASIONumChannels            : c_int,         /* [r/w] Optional. Specify 0 to ignore. Number of channels available on the ASIO device. */
    pub ASIOChannelList            : **c_char,      /* [r/w] Optional. Specify 0 to ignore. Pointer to an array of strings (number of entries defined by ASIONumChannels) with ASIO channel names. */
    pub ASIOSpeakerList            : *fmod::Speaker,/* [r/w] Optional. Specify 0 to ignore. Pointer to a list of speakers that the ASIO channels map to.  This can be called after System::init to remap ASIO output. */
    pub max3DReverbDSPs            : c_int,         /* [r/w] Optional. Specify 0 to ignore. The max number of 3d reverb DSP's in the system. (NOTE: CURRENTLY DISABLED / UNUSED) */
    pub HRTFMinAngle               : c_float,       /* [r/w] Optional.                      For use with FMOD_INIT_HRTF_LOWPASS.  The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function begins to have an effect. 0 = in front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 180.0. */
    pub HRTFMaxAngle               : c_float,       /* [r/w] Optional.                      For use with FMOD_INIT_HRTF_LOWPASS.  The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function has maximum effect. 0 = front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 360.0. */
    pub HRTFFreq                   : c_float,       /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_HRTF_LOWPASS.  The cutoff frequency of the HRTF's lowpass filter function when at maximum effect. (i.e. at HRTFMaxAngle).  Default = 4000.0. */
    pub vol0virtualvol             : c_float,       /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_VOL0_BECOMES_VIRTUAL.  If this flag is used, and the volume is 0.0, then the sound will become virtual.  Use this value to raise the threshold to a different point where a sound goes virtual. */
    pub eventqueuesize             : c_int,         /* [r/w] Optional. Specify 0 to ignore. For use with FMOD Event system only.  Specifies the number of slots available for simultaneous non blocking loads, across all threads.  Default = 32. */
    pub defaultDecodeBufferSize    : c_uint,        /* [r/w] Optional. Specify 0 to ignore. For streams. This determines the default size of the double buffer (in milliseconds) that a stream uses.  Default = 400ms */
    pub debugLogFilename           : *c_char,       /* [r/w] Optional. Specify 0 to ignore. Gives fmod's logging system a path/filename.  Normally the log is placed in the same directory as the executable and called fmod.log. When using System::getAdvancedSettings, provide at least 256 bytes of memory to copy into. */
    pub profileport                : c_ushort,      /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_ENABLE_PROFILE.  Specify the port to listen on for connections by the profiler application. */
    pub geometryMaxFadeTime        : c_uint,        /* [r/w] Optional. Specify 0 to ignore. The maximum time in miliseconds it takes for a channel to fade to the new level when its occlusion changes. */
    pub maxSpectrumWaveDataBuffers : c_uint,        /* [r/w] Optional. Specify 0 to ignore. Tells System::init to allocate a pool of wavedata/spectrum buffers to prevent memory fragmentation, any additional buffers will be allocated normally. */
    pub musicSystemCacheDelay      : c_uint,        /* [r/w] Optional. Specify 0 to ignore. The delay the music system should allow for loading a sample from disk (in milliseconds). Default = 400 ms. */
    pub distanceFilterCenterFreq   : c_float,       /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_DISTANCE_FILTERING.  The default center frequency in Hz for the distance filtering effect. Default = 1500.0. */
    pub stackSizeStream            : c_uint,        /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD Stream thread in bytes.  Useful for custom codecs that use excess stack.  Default 49,152 (48kb) */
    pub stackSizeNonBlocking       : c_uint,        /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD_NONBLOCKING loading thread.  Useful for custom codecs that use excess stack.  Default 65,536 (64kb) */
    pub stackSizeMixer             : c_uint         /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD mixer thread.  Useful for custom dsps that use excess stack.  Default 49,152 (48kb) */
}

pub struct FMOD_CODEC_DESCRIPTION
{
    pub name           : *c_char,                       /* [in] Name of the codec. */
    pub version        : c_uint,                        /* [in] Plugin writer's version number. */
    pub defaultasstream: c_int,                         /* [in] Tells FMOD to open the file as a stream when calling System::createSound, and not a static sample.  Should normally be 0 (FALSE), because generally the user wants to decode the file into memory when using System::createSound.   Mainly used for formats that decode for a very long time, or could use large amounts of memory when decoded.  Usually sequenced formats such as mod/s3m/xm/it/midi fall into this category.   It is mainly to stop users that don't know what they're doing from getting FMOD_ERR_MEMORY returned from createSound when they should have in fact called System::createStream or used FMOD_CREATESTREAM in System::createSound. */
    pub timeunits      : FMOD_TIMEUNIT,                 /* [in] When setposition codec is called, only these time formats will be passed to the codec. Use bitwise OR to accumulate different types. */
    pub open           : FMOD_CODEC_OPENCALLBACK,       /* [in] Open callback for the codec for when FMOD tries to open a sound using this codec. */
    pub close          : FMOD_CODEC_CLOSECALLBACK,      /* [in] Close callback for the codec for when FMOD tries to close a sound using this codec.  */
    pub read           : FMOD_CODEC_READCALLBACK,       /* [in] Read callback for the codec for when FMOD tries to read some data from the file to the destination format (specified in the open callback). */
    pub getlength      : FMOD_CODEC_GETLENGTHCALLBACK,  /* [in] Callback to return the length of the song in whatever format required when Sound::getLength is called. */
    pub setposition    : FMOD_CODEC_SETPOSITIONCALLBACK,/* [in] Seek callback for the codec for when FMOD tries to seek within the file with Channel::setPosition. */
    pub getposition    : FMOD_CODEC_GETPOSITIONCALLBACK,/* [in] Tell callback for the codec for when FMOD tries to get the current position within the with Channel::getPosition. */
    pub soundcreate    : FMOD_CODEC_SOUNDCREATECALLBACK,/* [in] Sound creation callback for the codec when FMOD finishes creating the sound.  (So the codec can set more parameters for the related created sound, ie loop points/mode or 3D attributes etc). */
    pub getwaveformat  : FMOD_CODEC_GETWAVEFORMAT       /* [in] Callback to tell FMOD about the waveformat of a particular subsound.  This is to save memory, rather than saving 1000 FMOD_CODEC_WAVEFORMAT structures in the codec, the codec might have a more optimal way of storing this information. */
}

pub struct FMOD_CODEC_WAVEFORMAT
{
    pub name       : [c_char, ..256],  /* [in] Name of sound.*/
    pub format     : fmod::SoundFormat,/* [in] Format for (decompressed) codec output, ie fmod::SoundFormat_PCM8, fmod::SoundFormat_PCM16.*/
    pub channels   : c_int,            /* [in] Number of channels used by codec, ie mono = 1, stereo = 2. */
    pub frequency  : c_int,            /* [in] Default frequency in hz of the codec, ie 44100. */
    pub lengthbytes: c_uint,           /* [in] Length in bytes of the source data. */
    pub lengthpcm  : c_uint,           /* [in] Length in decompressed, PCM samples of the file, ie length in seconds * frequency.  Used for Sound::getLength and for memory allocation of static decompressed sample data. */
    pub blockalign : c_int,            /* [in] Blockalign in decompressed, PCM samples of the optimal decode chunk size for this format.  The codec read callback will be called in multiples of this value. */
    pub loopstart  : c_int,            /* [in] Loopstart in decompressed, PCM samples of file. */
    pub loopend    : c_int,            /* [in] Loopend in decompressed, PCM samples of file. */
    pub mode       : FMOD_MODE,        /* [in] Mode to determine whether the sound should by default load as looping, non looping, 2d or 3d. */
    pub channelmask: c_uint            /* [in] Microsoft speaker channel mask, as defined for WAVEFORMATEXTENSIBLE and is found in ksmedia.h.  Leave at 0 to play in natural speaker order. */
}

pub struct FMOD_CODEC_STATE
{
    pub numsubsounds: c_int,                      /* [in] Number of 'subsounds' in this sound.  Anything other than 0 makes it a 'container' format (ie CDDA/DLS/FSB etc which contain 1 or more su bsounds).  For most normal, single sound codec such as WAV/AIFF/MP3, this should be 0 as they are not a container for subsounds, they are the sound by itself. */
    pub waveformat  : FMOD_CODEC_WAVEFORMAT,      /* [in] Pointer to an array of format structures containing information about each sample.  Can be 0 or NULL if FMOD_CODEC_GETWAVEFORMAT callback is preferred.  The number of entries here must equal the number of subsounds defined in the subsound parameter. If numsubsounds = 0 then there should be 1 instance of this structure. */
    pub plugindata  : *c_void,                    /* [in] Plugin writer created data the codec author wants to attach to this object. */
                                               
    pub filehandle  : *c_void,                    /* [out] This will return an internal FMOD file handle to use with the callbacks provided.  */
    pub filesize    : c_uint,                     /* [out] This will contain the size of the file in bytes. */
    pub fileread    : FMOD_FILE_READCALLBACK,     /* [out] This will return a callable FMOD file function to use from codec. */
    pub fileseek    : FMOD_FILE_SEEKCALLBACK,     /* [out] This will return a callable FMOD file function to use from codec.  */
    pub metadata    : FMOD_CODEC_METADATACALLBACK /* [out] This will return a callable FMOD metadata function to use from codec.  */
}

pub struct FMOD_REVERB_PROPERTIES
{                                       /*       MIN    MAX     DEFAULT DESCRIPTION */
    pub Instance        : c_int,       /* [w]   0      3       0       Environment Instance.                                                 (SUPPORTED:SFX(4 instances) and Wii (3 instances)) */
    pub Environment     : c_int,       /* [r/w] -1     25      -1      Sets all listener properties.  -1 = OFF.                              (SUPPORTED:SFX(-1 only)/PSP) */
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
    pub Flags           : c_uint       /* [r/w] FMOD_REVERB_FLAGS - modifies the behavior of above properties                                (SUPPORTED:WII) */
}

pub struct FMOD_TAG
{
    pub _type   : fmod::TagType,       /* [r] The type of this tag. */
    pub datatype: fmod::TagDataType,   /* [r] The type of data that this tag contains */
    pub name    : *c_char,             /* [r] The name of this tag i.e. "TITLE", "ARTIST" etc. */
    pub data    : *c_void,             /* [r] Pointer to the tag data - its format is determined by the datatype member */
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