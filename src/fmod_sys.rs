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
}