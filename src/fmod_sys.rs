/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FmodEx library, is provided by FIRELIGHT TECHNOLOGIES.
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

use libc::{c_void, c_uint, c_int, c_char, c_float};
use ffi;
use types::*;
use enums::*;
use sound;
use sound::Sound;
use std::mem;

pub struct FmodGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8, ..8] 
}

pub struct FmodSoftwareFormat {
    pub sample_rate         : i32,
    pub format              : FMOD_SOUND_FORMAT,
    pub num_output_channels : i32,
    pub max_input_channels  : i32,
    pub resample_method     : FMOD_DSP_RESAMPLER,
    pub bits                : i32
}

pub struct FmodAdvancedSettings
{
    pub max_MPEG_codecs               : i32,
    pub max_ADPCM_codecs              : i32,
    pub max_XMA_codecs                : i32,
    pub max_CELT_codecs               : i32,
    pub max_VORBIS_codecs             : i32,
    pub max_AT9_codecs                : i32,
    pub max_PCM_codecs                : i32,
    pub ASIO_num_channels             : i32,
    pub ASIO_channel_list             : Vec<StrBuf>,
    pub ASIO_speaker_list             : Vec<FMOD_SPEAKER>,
    pub max_3D_reverb_DSPs            : i32,
    pub HRTF_min_angle                : f32,
    pub HRTF_max_angle                : f32,
    pub HRTF_freq                     : f32,
    pub vol0_virtual_vol              : f32,
    pub event_queue_size              : i32,
    pub default_decode_buffer_size    : u32,
    pub debug_log_filename            : StrBuf,
    pub profile_port                  : u16,
    pub geometry_max_fade_time        : u32,
    pub max_spectrum_wave_data_buffers: u32,
    pub music_system_cache_delay      : u32,
    pub distance_filter_center_freq   : f32,
    pub stack_size_stream             : u32,
    pub stack_size_non_blocking       : u32,
    pub stack_size_mixer              : u32
}

pub struct FmodCodecDescription {
    pub name            : StrBuf,                             /* [in] Name of the codec. */
    pub version         : u32,                                /* [in] Plugin writer's version number. */
    pub defaultasstream : i32,                                /* [in] Tells FMOD to open the file as a stream when calling System::createSound, and not a static sample.  Should normally be 0 (FALSE), because generally the user wants to decode the file into memory when using System::createSound.   Mainly used for formats that decode for a very long time, or could use large amounts of memory when decoded.  Usually sequenced formats such as mod/s3m/xm/it/midi fall into this category.   It is mainly to stop users that don't know what they're doing from getting FMOD_ERR_MEMORY returned from createSound when they should have in fact called System::createStream or used FMOD_CREATESTREAM in System::createSound. */
    pub timeunits       : FmodTimeUnit,                       /* [in] When setposition codec is called, only these time formats will be passed to the codec. Use bitwise OR to accumulate different types. */
    pub open            : ffi::FMOD_CODEC_OPENCALLBACK,       /* [in] Open callback for the codec for when FMOD tries to open a sound using this codec. */
    pub close           : ffi::FMOD_CODEC_CLOSECALLBACK,      /* [in] Close callback for the codec for when FMOD tries to close a sound using this codec.  */
    pub read            : ffi::FMOD_CODEC_READCALLBACK,       /* [in] Read callback for the codec for when FMOD tries to read some data from the file to the destination format (specified in the open callback). */
    pub getlength       : ffi::FMOD_CODEC_GETLENGTHCALLBACK,  /* [in] Callback to return the length of the song in whatever format required when Sound::getLength is called. */
    pub setposition     : ffi::FMOD_CODEC_SETPOSITIONCALLBACK,/* [in] Seek callback for the codec for when FMOD tries to seek within the file with Channel::setPosition. */
    pub getposition     : ffi::FMOD_CODEC_GETPOSITIONCALLBACK,/* [in] Tell callback for the codec for when FMOD tries to get the current position within the with Channel::getPosition. */
    pub soundcreate     : ffi::FMOD_CODEC_SOUNDCREATECALLBACK,/* [in] Sound creation callback for the codec when FMOD finishes creating the sound.  (So the codec can set more parameters for the related created sound, ie loop points/mode or 3D attributes etc). */
    pub getwaveformat   : ffi::FMOD_CODEC_GETWAVEFORMAT       /* [in] Callback to tell FMOD about the waveformat of a particular subsound.  This is to save memory, rather than saving 1000 FMOD_CODEC_WAVEFORMAT structures in the codec, the codec might have a more optimal way of storing this information. */
}

pub struct FmodDSP {
    dsp : ffi::FMOD_DSP
}

impl FmodDSP {
    pub fn new() -> FmodDSP {
        FmodDSP{dsp: ::std::ptr::null()}
    }
}

pub struct FmodOutputHandle {
    handle: *c_void
}

pub struct FmodVector
{
    pub x: f32, /* X co-ordinate in 3D space. */
    pub y: f32, /* Y co-ordinate in 3D space. */
    pub z: f32  /* Z co-ordinate in 3D space. */
}

impl FmodVector {
    pub fn new() -> FmodVector {
        FmodVector{x: 0f32, y: 0f32, z: 0f32}
    }

    fn from_c(vec : ffi::FMOD_VECTOR) -> FmodVector {
        FmodVector{x: vec.x, y: vec.y, z: vec.z}
    }

    fn convert_to_c(&self) -> ffi::FMOD_VECTOR {
        ffi::FMOD_VECTOR{x: self.x, y: self.y, z: self.z}
    }
}

pub struct FmodSys {
    system : ffi::FMOD_SYSTEM,
}

impl FmodSys {
    pub fn new() -> Result<FmodSys, FMOD_RESULT> {
        let tmp = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_Create(&tmp) } {
            FMOD_OK => Ok(FmodSys{system : tmp}),
            err => Err(err)
        }
    }

    pub fn init(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Init(self.system, 1, FMOD_INIT_NORMAL, ::std::ptr::null()) }
    }

    pub fn init_with_parameters(&self, max_channels: i32, FmodInitFlag(flag) : FmodInitFlag) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Init(self.system, max_channels, flag, ::std::ptr::null()) }
    }

    pub fn update(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Update(self.system) }
    }

    pub fn release(&self) -> FMOD_RESULT {
        unsafe {
            ffi::FMOD_System_Close(self.system);
            ffi::FMOD_System_Release(self.system)
        }
    }

    pub fn create_sound(&self, music : StrBuf, options: Option<FmodMode>) -> Result<Sound, FMOD_RESULT> {
        let tmp_v = music.clone().into_owned();
        let sound = ::std::ptr::null();
        let op = match options {
            Some(FmodMode(t)) => t,
            None => FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM
        };

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateSound(self.system, c_str, op, ::std::ptr::null(), &sound) } {
                FMOD_OK => {Ok(sound::new(self.system, music.clone(), sound))},
                err => Err(err)
            }
        })
    }

    pub fn set_output(&self, output_type: FMOD_OUTPUTTYPE) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetOutput(self.system, output_type) }
    }

    pub fn get_output(&self) -> Result<FMOD_OUTPUTTYPE, FMOD_RESULT> {
        let output_type = FMOD_OUTPUTTYPE_AUTODETECT;
        
        match unsafe { ffi::FMOD_System_GetOutput(self.system, &output_type) } {
            FMOD_OK => Ok(output_type),
            e => Err(e)
        }
    }

    pub fn get_num_drivers(&self) -> Result<i32, FMOD_RESULT> {
        let num_drivers = 0i32;

        match unsafe { ffi::FMOD_System_GetNumDrivers(self.system, &num_drivers) } {
            FMOD_OK => Ok(num_drivers),
            e => Err(e)
        }
    }

    pub fn get_driver_info(&self, id: i32, name: StrBuf) -> Result<FmodGuid, FMOD_RESULT> {
        let tmp_v = name.clone().into_owned();
        let guid = ffi::FMOD_GUID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetDriverInfo(self.system, id, c_str, tmp_v.len() as i32, &guid) } {
                FMOD_OK => Ok(FmodGuid{data1: guid.Data1, data2: guid.Data2, data3: guid.Data3, data4: guid.Data4}),
                e => Err(e)
            }
        })
    }

    pub fn get_driver_caps(&self, id: i32) -> Result<(FmodCaps, i32, FMOD_SPEAKERMODE), FMOD_RESULT> {
        let fmod_caps = 0u32;
        let speaker_mode = FMOD_SPEAKERMODE_RAW;
        let control_panel_output_rate = 0i32;

        match unsafe { ffi::FMOD_System_GetDriverCaps(self.system, id, &fmod_caps, &control_panel_output_rate, &speaker_mode) } {
            FMOD_OK => Ok((FmodCaps(fmod_caps), control_panel_output_rate, speaker_mode)),
            e => Err(e)
        }
    }

    pub fn set_driver(&self, driver: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetDriver(self.system, driver) }
    }

    pub fn get_driver(&self) -> Result<i32, FMOD_RESULT> {
        let driver = 0i32;

        match unsafe { ffi::FMOD_System_GetDriver(self.system, &driver) } {
            FMOD_OK => Ok(driver),
            e => Err(e)
        }
    }

    pub fn set_hardware_channels(&self, num_hardware_channels: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetHardwareChannels(self.system, num_hardware_channels) }
    }

    pub fn get_hardware_channels(&self) -> Result<i32, FMOD_RESULT> {
        let num_hardware_channels = 0i32;

        match unsafe { ffi::FMOD_System_GetHardwareChannels(self.system, &num_hardware_channels) } {
            FMOD_OK => Ok(num_hardware_channels),
            e => Err(e)
        }
    }

    pub fn set_software_channels(&self, num_software_channels: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSoftwareChannels(self.system, num_software_channels) }
    }

    pub fn get_software_channels(&self) -> Result<i32, FMOD_RESULT> {
        let num_software_channels = 0i32;

        match unsafe { ffi::FMOD_System_GetSoftwareChannels(self.system, &num_software_channels) } {
            FMOD_OK => Ok(num_software_channels),
            e => Err(e)
        }
    }

    pub fn set_software_format(&self, sample_rate: i32, format: FMOD_SOUND_FORMAT, num_output_channels: i32,
        max_input_channels: i32, resample_method: FMOD_DSP_RESAMPLER) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSoftwareFormat(self.system, sample_rate, format, num_output_channels,
            max_input_channels, resample_method) }
    }

    pub fn get_software_format(&self) -> Result<FmodSoftwareFormat, FMOD_RESULT> {
        let t = FmodSoftwareFormat{sample_rate: 0, format: FMOD_SOUND_FORMAT_NONE, num_output_channels: 0,
            max_input_channels: 0, resample_method: FMOD_DSP_RESAMPLER_NOINTERP, bits: 0};

        match unsafe { ffi::FMOD_System_GetSoftwareFormat(self.system, &t.sample_rate, &t.format,
            &t.num_output_channels, &t.max_input_channels, &t.resample_method, &t.bits) } {
            FMOD_OK => Ok(t),
            e => Err(e)
        }
    }

    pub fn set_DSP_buffer_size(&self, buffer_length: u32, num_buffers: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetDSPBufferSize(self.system, buffer_length, num_buffers) }
    }

    pub fn get_DSP_buffer_size(&self) -> Result<(u32, i32), FMOD_RESULT> {
        let buffer_length = 0u32;
        let num_buffers = 0i32;

        match unsafe { ffi::FMOD_System_GetDSPBufferSize(self.system, &buffer_length, &num_buffers) } {
            FMOD_OK => Ok((buffer_length, num_buffers)),
            e => Err(e)
        }
    }

    pub fn set_advanced_settings(&self, settings: FmodAdvancedSettings) -> FMOD_RESULT {
        let converted_c_char = Vec::from_fn(settings.ASIO_channel_list.len(), |pos| {
            settings.ASIO_channel_list.get(pos).clone().into_owned().with_c_str(|c_str| c_str)
        });
        let deb_log_filename = settings.debug_log_filename.clone().into_owned();
        let advanced_settings = ffi::FMOD_ADVANCEDSETTINGS{cbsize: mem::size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGcodecs: settings.max_MPEG_codecs, maxADPCMcodecs: settings.max_ADPCM_codecs, maxXMAcodecs: settings.max_XMA_codecs,
            maxCELTcodecs: settings.max_CELT_codecs, maxVORBIScodecs: settings.max_VORBIS_codecs, maxAT9Codecs: settings.max_AT9_codecs,
            maxPCMcodecs: settings.max_PCM_codecs, ASIONumChannels: settings.ASIO_num_channels, ASIOChannelList: converted_c_char.as_ptr(),
            ASIOSpeakerList: settings.ASIO_speaker_list.as_ptr(), max3DReverbDSPs: settings.max_3D_reverb_DSPs, HRTFMinAngle: settings.HRTF_min_angle,
            HRTFMaxAngle: settings.HRTF_max_angle, HRTFFreq: settings.HRTF_freq, vol0virtualvol: settings.vol0_virtual_vol,
            eventqueuesize: settings.event_queue_size, defaultDecodeBufferSize: settings.default_decode_buffer_size,
            debugLogFilename: deb_log_filename.with_c_str(|c_str| c_str), profileport: settings.profile_port,
            geometryMaxFadeTime: settings.geometry_max_fade_time, maxSpectrumWaveDataBuffers: settings.max_spectrum_wave_data_buffers,
            musicSystemCacheDelay: settings.music_system_cache_delay, distanceFilterCenterFreq: settings.distance_filter_center_freq,
            stackSizeStream: settings.stack_size_stream, stackSizeNonBlocking: settings.stack_size_non_blocking, stackSizeMixer: settings.stack_size_mixer};

        unsafe { ffi::FMOD_System_SetAdvancedSettings(self.system, &advanced_settings) }
    }

    pub fn get_advanced_settings(&self) -> Result<FmodAdvancedSettings, FMOD_RESULT> {
        let advanced_settings = ffi::FMOD_ADVANCEDSETTINGS{cbsize: mem::size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGcodecs: 0, maxADPCMcodecs: 0, maxXMAcodecs: 0, maxCELTcodecs: 0, maxVORBIScodecs: 0, maxAT9Codecs: 0,
            maxPCMcodecs: 0, ASIONumChannels: 0, ASIOChannelList: ::std::ptr::null(), ASIOSpeakerList: ::std::ptr::null(),
            max3DReverbDSPs: 0, HRTFMinAngle: 0f32, HRTFMaxAngle: 0f32, HRTFFreq: 0f32, vol0virtualvol: 0f32, eventqueuesize: 0,
            defaultDecodeBufferSize: 0, debugLogFilename: ::std::ptr::null(), profileport: 0, geometryMaxFadeTime: 0,
            maxSpectrumWaveDataBuffers: 0, musicSystemCacheDelay: 0, distanceFilterCenterFreq: 0f32, stackSizeStream: 0, stackSizeNonBlocking: 0,
            stackSizeMixer: 0};

        match unsafe { ffi::FMOD_System_GetAdvancedSettings(self.system, &advanced_settings) } {
            FMOD_OK => {
                let mut converted_ASIO_channel_vec = Vec::new();
                let mut converted_ASIO_speaker_vec = Vec::new();

                if advanced_settings.ASIOChannelList != ::std::ptr::null() {
                    unsafe {::std::ptr::array_each(advanced_settings.ASIOChannelList, |c_str| {
                        converted_ASIO_channel_vec.push(StrBuf::from_owned_str(::std::str::raw::from_c_str(c_str)))
                    })};
                }
                if advanced_settings.ASIOSpeakerList != ::std::ptr::null() {
                    // TODO : I need to check if that closure can segfault !
                    unsafe {::std::ptr::position(advanced_settings.ASIOSpeakerList, |elem| {
                        if elem != &FMOD_SPEAKER_NULL {
                            converted_ASIO_speaker_vec.push(*elem);
                        }
                        elem != &FMOD_SPEAKER_NULL
                    })};
                }
                Ok(FmodAdvancedSettings{max_MPEG_codecs: advanced_settings.maxMPEGcodecs, max_ADPCM_codecs: advanced_settings.maxADPCMcodecs,
                    max_XMA_codecs: advanced_settings.maxXMAcodecs, max_CELT_codecs: advanced_settings.maxCELTcodecs, max_VORBIS_codecs: advanced_settings.maxVORBIScodecs,
                    max_AT9_codecs: advanced_settings.maxAT9Codecs, max_PCM_codecs: advanced_settings.maxPCMcodecs, ASIO_num_channels: advanced_settings.ASIONumChannels,
                    ASIO_channel_list: converted_ASIO_channel_vec.clone(), ASIO_speaker_list: converted_ASIO_speaker_vec, max_3D_reverb_DSPs: advanced_settings.max3DReverbDSPs,
                    HRTF_min_angle: advanced_settings.HRTFMinAngle, HRTF_max_angle: advanced_settings.HRTFMaxAngle, HRTF_freq: advanced_settings.HRTFFreq,
                    vol0_virtual_vol: advanced_settings.vol0virtualvol, event_queue_size: advanced_settings.eventqueuesize, default_decode_buffer_size: advanced_settings.defaultDecodeBufferSize,
                    debug_log_filename: {
                        if advanced_settings.debugLogFilename != ::std::ptr::null() {
                            StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(advanced_settings.debugLogFilename) }).clone()
                        } else {
                            StrBuf::new()
                        }
                    },
                    profile_port: advanced_settings.profileport, geometry_max_fade_time: advanced_settings.geometryMaxFadeTime, max_spectrum_wave_data_buffers: advanced_settings.maxSpectrumWaveDataBuffers,
                    music_system_cache_delay: advanced_settings.musicSystemCacheDelay, distance_filter_center_freq: advanced_settings.distanceFilterCenterFreq,
                    stack_size_stream: advanced_settings.stackSizeStream, stack_size_non_blocking: advanced_settings.stackSizeNonBlocking, stack_size_mixer: advanced_settings.stackSizeMixer})
            }
            e => Err(e)
        }
    }

    pub fn set_speaker_mode(&self, speaker_mode: FMOD_SPEAKERMODE) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSpeakerMode(self.system, &speaker_mode) }
    }

    pub fn get_speaker_mode(&self) -> Result<FMOD_SPEAKERMODE, FMOD_RESULT> {
        let speaker_mode = FMOD_SPEAKERMODE_RAW;

        match unsafe { ffi::FMOD_System_GetSpeakerMode(self.system, &speaker_mode) } {
            FMOD_OK => Ok(speaker_mode),
            e => Err(e)
        }
    }

    pub fn set_plugin_path(&self, path: StrBuf) -> FMOD_RESULT {
        let tmp_v = path.clone().into_owned();

        tmp_v.with_c_str(|c_str|{
            unsafe { ffi::FMOD_System_SetPluginPath(self.system, c_str) }
        })
    }

    pub fn load_plugin(&self, filename: StrBuf, priority: u32) -> Result<FmodPluginHandle, FMOD_RESULT> {
        let tmp_v = filename.clone().into_owned();
        let handle = 0u32;

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_LoadPlugin(self.system, c_str, &handle, priority) } {
                FMOD_OK => Ok(FmodPluginHandle(handle)),
                e => Err(e)
            }
        })
    }

    pub fn unload_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_UnloadPlugin(self.system, handle) }
    }

    pub fn get_num_plugins(&self, plugin_type: FMOD_PLUGINTYPE) -> Result<i32, FMOD_RESULT> {
        let num_plugins = 0i32;

        match unsafe { ffi::FMOD_System_GetNumPlugins(self.system, plugin_type, &num_plugins) } {
            FMOD_OK => Ok(num_plugins),
            e => Err(e)
        }
    }

    pub fn get_plugin_handle(&self, plugin_type: FMOD_PLUGINTYPE, index: i32) ->Result<FmodPluginHandle, FMOD_RESULT> {
        let handle = 0u32;

        match unsafe { ffi::FMOD_System_GetPluginHandle(self.system, plugin_type, index, &handle) } {
            FMOD_OK => Ok(FmodPluginHandle(handle)),
            e => Err(e)
        }
    }

    pub fn get_plugin_info(&self, FmodPluginHandle(handle): FmodPluginHandle, name_len: u32) -> Result<(StrBuf, FMOD_PLUGINTYPE, u32), FMOD_RESULT> {
        let name = StrBuf::with_capacity(name_len as uint).into_owned();
        let plugin_type = FMOD_PLUGINTYPE_OUTPUT;
        let version = 0u32;

        name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetPluginInfo(self.system, handle, &plugin_type, c_str, name_len as i32, &version) } {
                FMOD_OK => Ok((StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_str) }).clone(), plugin_type, version)),
                e => Err(e)
            }
        })
    }

    pub fn set_output_by_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetOutputByPlugin(self.system, handle) }
    }

    pub fn get_output_by_plugin(&self) -> Result<FmodPluginHandle, FMOD_RESULT> {
        let handle = 0u32;

        match unsafe { ffi::FMOD_System_GetOutputByPlugin(self.system, &handle) } {
            FMOD_OK => Ok(FmodPluginHandle(handle)),
            e => Err(e)
        }
    }

    pub fn create_DSP_by_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> Result<FmodDSP, FMOD_RESULT> {
        let dsp = FmodDSP::new();

        match unsafe { ffi::FMOD_System_CreateDSPByPlugin(self.system, handle, &dsp.dsp) } {
            FMOD_OK => Ok(dsp),
            e => Err(e)
        }
    }

    pub fn set_3D_settings(&self, doppler_scale: f32, distance_factor: f32, roll_off_scale: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Set3DSettings(self.system, doppler_scale, distance_factor, roll_off_scale) }
    }

    pub fn get_3D_settings(&self) -> Result<(f32, f32, f32), FMOD_RESULT> {
        let doppler_scale = 0f32;
        let distance_factor = 0f32;
        let roll_off_scale = 0f32;

        match unsafe { ffi::FMOD_System_Get3DSettings(self.system, &doppler_scale, &distance_factor, &roll_off_scale) } {
            FMOD_OK => Ok((doppler_scale, distance_factor, roll_off_scale)),
            e => Err(e)
        }
    }

    pub fn set_3D_num_listeners(&self, num_listeners : i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Set3DNumListeners(self.system, num_listeners) }
    }

    pub fn get_3D_num_listeners(&self) -> Result<i32, FMOD_RESULT> {
        let num_listeners = 0i32;

        match unsafe { ffi::FMOD_System_Get3DNumListeners(self.system, &num_listeners) } {
            FMOD_OK => Ok(num_listeners),
            e => Err(e)
        }
    }

    pub fn set_3D_listener_attributes(&self, listener: i32, pos: FmodVector, vel: FmodVector, forward: FmodVector,
        up: FmodVector) -> FMOD_RESULT {
        let c_p = pos.convert_to_c();
        let c_v = vel.convert_to_c();
        let c_f = forward.convert_to_c();
        let c_u = up.convert_to_c();

        unsafe { ffi::FMOD_System_Set3DListenerAttributes(self.system, listener, &c_p, &c_v, &c_f, &c_u) }
    }

    pub fn get_3D_listener_attributes(&self, listener: i32) -> Result<(FmodVector, FmodVector, FmodVector, FmodVector), FMOD_RESULT> {
        let pos = FmodVector::new().convert_to_c();
        let vel = FmodVector::new().convert_to_c();
        let forward = FmodVector::new().convert_to_c();
        let up = FmodVector::new().convert_to_c();

        match unsafe { ffi::FMOD_System_Get3DListenerAttributes(self.system, listener, &pos, &vel, &forward, &up) } {
            FMOD_OK => Ok((FmodVector::from_c(pos), FmodVector::from_c(vel), FmodVector::from_c(forward), FmodVector::from_c(up))),
            e => Err(e)
        }
    }

    pub fn set_3D_speaker_position(&self, speaker: FMOD_SPEAKER, x: f32, y: f32, active: bool) -> FMOD_RESULT {
        let t_active = match active {
            true => 1i32,
            false => 0i32
        };
        unsafe { ffi::FMOD_System_Set3DSpeakerPosition(self.system, speaker, x, y, t_active) }
    }

    pub fn get_3D_speaker_position(&self, speaker: FMOD_SPEAKER) -> Result<(f32, f32, bool), FMOD_RESULT> {
        let x = 0f32;
        let y = 0f32;
        let active = 0i32;

        match unsafe { ffi::FMOD_System_Get3DSpeakerPosition(self.system, speaker, &x, &y, &active) } {
            FMOD_OK => Ok((x, y, match active {
                1 => true,
                _ => false
            })),
            e => Err(e)
        }
    }

    pub fn set_stream_buffer_size(&self, file_buffer_size: u32, FmodTimeUnit(file_buffer_size_type): FmodTimeUnit) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetStreamBufferSize(self.system, file_buffer_size, file_buffer_size_type) }
    }

    pub fn get_stream_buffer_size(&self) -> Result<(u32, FmodTimeUnit), FMOD_RESULT> {
        let file_buffer_size = 0u32;
        let file_buffer_size_type = 0u32;

        match unsafe { ffi::FMOD_System_GetStreamBufferSize(self.system, &file_buffer_size, &file_buffer_size_type) } {
            FMOD_OK => Ok((file_buffer_size, FmodTimeUnit(file_buffer_size_type))),
            e => Err(e)
        }
    }

    pub fn get_version(&self) -> Result<u32, FMOD_RESULT> {
        let version = 0u32;

        match unsafe { ffi::FMOD_System_GetVersion(self.system, &version) } {
            FMOD_OK => Ok(version),
            e => Err(e)
        }
    }

    pub fn get_output_handle(&self) -> Result<FmodOutputHandle, FMOD_RESULT> {
        let output_h = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_GetOutputHandle(self.system, &output_h) } {
            FMOD_OK => Ok(FmodOutputHandle{handle: output_h}),
            e => Err(e)
        }
    }

    pub fn get_channels_playing(&self) -> Result<i32, FMOD_RESULT> {
        let playing_chans = 0i32;

        match unsafe { ffi::FMOD_System_GetChannelsPlaying(self.system, &playing_chans) } {
            FMOD_OK => Ok(playing_chans),
            e => Err(e)
        }
    }

    pub fn get_CPU_usage(&self) -> Result<(f32, f32, f32, f32, f32), FMOD_RESULT> {
        let dsp = 0f32;
        let stream = 0f32;
        let geometry = 0f32;
        let update = 0f32;
        let total = 0f32;

        match unsafe { ffi::FMOD_System_GetCPUUsage(self.system, &dsp, &stream, &geometry, &update, &total) } {
            FMOD_OK => Ok((dsp, stream, geometry, update, total)),
            e => Err(e)
        }
    }

    pub fn get_sound_RAM(&self) -> Result<(i32, i32, i32), FMOD_RESULT> {
        let current_alloced = 0i32;
        let max_alloced = 0i32;
        let total = 0i32;

        match unsafe { ffi::FMOD_System_GetSoundRAM(self.system, &current_alloced, &max_alloced, &total) } {
            FMOD_OK => Ok((current_alloced, max_alloced, total)),
            e => Err(e)
        }
    }

    pub fn get_num_CDROM_drives(&self) -> Result<i32, FMOD_RESULT> {
        let num_drives = 0i32;

        match unsafe { ffi::FMOD_System_GetNumCDROMDrives(self.system, &num_drives) } {
            FMOD_OK => Ok(num_drives),
            e => Err(e)
        }
    }

    pub fn get_CDROM_drive_name(&self, drive: i32, drive_name_len: u32, scsi_name_len: u32, device_name_len: u32) -> Result<(StrBuf, StrBuf, StrBuf), FMOD_RESULT> {
        let drive_name = StrBuf::with_capacity(drive_name_len as uint).into_owned();
        let scsi_name = StrBuf::with_capacity(scsi_name_len as uint).into_owned();
        let device_name = StrBuf::with_capacity(device_name_len as uint).into_owned();

        drive_name.with_c_str(|c_drive_name|{
            scsi_name.with_c_str(|c_scsi_name|{
                device_name.with_c_str(|c_device_name|{
                    match unsafe { ffi::FMOD_System_GetCDROMDriveName(self.system, drive, c_drive_name, drive_name_len as i32, c_scsi_name, scsi_name_len as i32,
                        c_device_name, device_name_len as i32) } {
                        FMOD_OK => Ok((StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_drive_name) }).clone(),
                                        StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_scsi_name) }).clone(),
                                        StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_device_name) }).clone())),
                        e => Err(e)
                    }
                })
            })
        })
    }
}