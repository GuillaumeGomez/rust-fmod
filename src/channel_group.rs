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

use enums::*;
use types::*;
use ffi;
use channel;
use fmod_sys;
use libc::{c_int};

pub struct ChannelGroup {
    channel_group: ffi::FMOD_CHANNELGROUP,
}

pub fn get_ffi(channel_group : ChannelGroup) -> ffi::FMOD_CHANNELGROUP {
    channel_group.channel_group
}

pub fn new(channel_group : ffi::FMOD_CHANNELGROUP) -> ChannelGroup {
    ChannelGroup{channel_group: channel_group}
}

impl ChannelGroup {
    pub fn release(&mut self) -> FMOD_RESULT {
        match unsafe { ffi::FMOD_ChannelGroup_Release(self.channel_group) } {
            FMOD_OK => {
                self.channel_group = ::std::ptr::null();
                FMOD_OK
            }
            e => e
        }
    }

    pub fn set_volume(&self, volume: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_SetVolume(self.channel_group, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, FMOD_RESULT> {
        let volume = 0f32;

        match unsafe { ffi::FMOD_ChannelGroup_GetVolume(self.channel_group, &volume) } {
            FMOD_OK => Ok(volume),
            e => Err(e)
        }
    }

    pub fn set_pitch(&self, pitch: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_SetPitch(self.channel_group, pitch) }
    }

    pub fn get_pitch(&self) -> Result<f32, FMOD_RESULT> {
        let pitch = 0f32;

        match unsafe { ffi::FMOD_ChannelGroup_GetPitch(self.channel_group, &pitch) } {
            FMOD_OK => Ok(pitch),
            e => Err(e)
        }
    }

    pub fn set_paused(&self, paused: bool) -> FMOD_RESULT {
        let t_paused = match paused {
            true => 1,
            _ => 0
        };

        unsafe { ffi::FMOD_ChannelGroup_SetPaused(self.channel_group, t_paused) }
    }

    pub fn get_paused(&self) -> Result<bool, FMOD_RESULT> {
        let paused = 0;

        match unsafe { ffi::FMOD_ChannelGroup_GetPaused(self.channel_group, &paused) } {
            FMOD_OK => Ok(match paused {
                1 => true,
                _ => false
            }),
            e => Err(e)
        }
    }

    pub fn set_mute(&self, mute: bool) -> FMOD_RESULT {
        let t_mute = match mute {
            true => 1,
            _ => 0
        };

        unsafe { ffi::FMOD_ChannelGroup_SetMute(self.channel_group, t_mute) }
    }

    pub fn get_mute(&self) -> Result<bool, FMOD_RESULT> {
        let mute = 0;

        match unsafe { ffi::FMOD_ChannelGroup_GetMute(self.channel_group, &mute) } {
            FMOD_OK => Ok(match mute {
                1 => true,
                _ => false
            }),
            e => Err(e)
        }
    }

    pub fn set_3D_occlusion(&self, direct_occlusion: f32, reverb_occlusion: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_Set3DOcclusion(self.channel_group, direct_occlusion, reverb_occlusion) }
    }

    pub fn get_3D_occlusion(&self) -> Result<(f32, f32), FMOD_RESULT> {
        let direct_occlusion = 0f32;
        let reverb_occlusion = 0f32;

        match unsafe { ffi::FMOD_ChannelGroup_Get3DOcclusion(self.channel_group, &direct_occlusion, &reverb_occlusion) } {
            FMOD_OK => Ok((direct_occlusion, reverb_occlusion)),
            e => Err(e)
        }
    }

    pub fn stop(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_Stop(self.channel_group) }
    }

    pub fn override_volume(&self, volume: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_OverrideVolume(self.channel_group, volume) }
    }

    pub fn override_frequency(&self, frequency: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_OverrideFrequency(self.channel_group, frequency) }
    }

    pub fn override_pan(&self, pan: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_OverridePan(self.channel_group, pan) }
    }

    pub fn override_reverb_properties(&self, properties: channel::FmodReverbChannelProperties) -> FMOD_RESULT {
        let prop = ffi::FMOD_REVERB_CHANNELPROPERTIES {Direct: properties.direct, Room: properties.room, Flags: properties.flags,
            ConnectionPoint: ffi::get_DSP_ffi(properties.connection_point)};

        unsafe { ffi::FMOD_ChannelGroup_OverrideReverbProperties(self.channel_group, &prop) }
    }

    pub fn override_3D_attributes(&self, pos: fmod_sys::FmodVector, vel: fmod_sys::FmodVector) -> FMOD_RESULT {
        let t_pos = pos.convert_to_c();
        let t_vel = vel.convert_to_c();

        unsafe { ffi::FMOD_ChannelGroup_Override3DAttributes(self.channel_group, &t_pos, &t_vel) }
    }

    pub fn override_speaker_mix(&self, front_left: f32, front_right: f32, center: f32, lfe: f32, back_left: f32, back_right: f32,
        side_left: f32, side_right: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_OverrideSpeakerMix(self.channel_group, front_left, front_right, center, lfe, back_left, back_right, side_left, side_right) }
    }

    pub fn add_group(&self, group: ChannelGroup) -> FMOD_RESULT {
        unsafe { ffi::FMOD_ChannelGroup_AddGroup(self.channel_group, group.channel_group) }
    }

    pub fn get_num_groups(&self) -> Result<i32, FMOD_RESULT> {
        let index = 0i32;

        match unsafe { ffi::FMOD_ChannelGroup_GetNumGroups(self.channel_group, &index) } {
            FMOD_OK => Ok(index),
            e => Err(e)
        }
    }

    pub fn get_group(&self, index: i32) -> Result<ChannelGroup, FMOD_RESULT> {
        let group = ::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetGroup(self.channel_group, index, &group) } {
            FMOD_OK => Ok(ChannelGroup{channel_group: group}),
            e => Err(e)
        }
    }

    pub fn get_parent_group(&self) -> Result<ChannelGroup, FMOD_RESULT> {
        let parent_group = ::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetParentGroup(self.channel_group, &parent_group) } {
            FMOD_OK => Ok(ChannelGroup{channel_group: parent_group}),
            e => Err(e)
        }
    }

    pub fn get_DSP_head(&self) -> Result<ffi::FmodDSP, FMOD_RESULT> {
        let dsp = ::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetDSPHead(self.channel_group, &dsp) } {
            FMOD_OK => Ok(ffi::FmodDSP::from_ptr(dsp)),
            e => Err(e)
        }
    }

    pub fn add_DSP(&self, dsp: ffi::FmodDSP) -> Result<ffi::FmodDSPConnection, FMOD_RESULT> {
        let dsp_connection = ::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_AddDSP(self.channel_group, ffi::get_DSP_ffi(dsp), &dsp_connection) } {
            FMOD_OK => Ok(ffi::FmodDSPConnection::from_ptr(dsp_connection)),
            e => Err(e)
        }
    }

    pub fn get_name(&self, name_len: u32) -> Result<StrBuf, FMOD_RESULT> {
        let name = StrBuf::with_capacity(name_len as uint).into_owned();

        name.with_c_str(|c_name|{
            match unsafe { ffi::FMOD_ChannelGroup_GetName(self.channel_group, c_name, name_len as i32) } {
                FMOD_OK => Ok(StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_name) }).clone()),
                e => Err(e)
            }
        })
    }

    pub fn get_num_channels(&self) -> Result<u32, FMOD_RESULT> {
        let num_channels = 0i32;

        match unsafe { ffi::FMOD_ChannelGroup_GetNumChannels(self.channel_group, &num_channels) } {
            FMOD_OK => Ok(num_channels as u32),
            e => Err(e)
        }
    }

    pub fn get_channel(&self, index: i32) -> Result<channel::Channel, FMOD_RESULT> {
        let channel = ::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetChannel(self.channel_group, index, &channel) } {
            FMOD_OK => Ok(channel::from_ptr(channel)),
            e => Err(e)
        }
    }

    pub fn get_spectrum(&self, spectrum_size : uint, options : Option<channel::FmodSpectrumOptions>) -> Result<Vec<f32>, FMOD_RESULT> {
        let ptr = Vec::from_elem(spectrum_size, 0f32);
        let mut window_type = FMOD_DSP_FFT_WINDOW_RECT;
        let mut channel_offset = 0;

        match options {
            Some(v) => {
                window_type = v.window_type;
                channel_offset = v.channel_offset;
            }
            None => {}
        };
        match unsafe { ffi::FMOD_ChannelGroup_GetSpectrum(self.channel_group, ptr.as_ptr(), spectrum_size as c_int, channel_offset, window_type) } {
            FMOD_OK => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size : uint, channel_offset : i32) -> Result<Vec<f32>, FMOD_RESULT> {
        let ptr = Vec::from_elem(wave_size, 0f32);

        match unsafe { ffi::FMOD_ChannelGroup_GetWaveData(self.channel_group, ptr.as_ptr(), wave_size as c_int, channel_offset) } {
            FMOD_OK => Ok(ptr),
            e => Err(e)
        }
    }
}