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
use dsp;
use dsp_connection;
use libc::{c_int};
use vector;
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;
use std::mem::transmute;

pub struct ChannelGroup {
    channel_group: ffi::FMOD_CHANNELGROUP,
}

pub fn get_ffi(channel_group: ChannelGroup) -> ffi::FMOD_CHANNELGROUP {
    channel_group.channel_group
}

pub fn from_ptr(channel_group: ffi::FMOD_CHANNELGROUP) -> ChannelGroup {
    ChannelGroup{channel_group: channel_group}
}

impl Drop for ChannelGroup {
    fn drop(&mut self) {
        self.release();
    }
}

impl ChannelGroup {
    pub fn release(&mut self) -> fmod::Result {
        if self.channel_group !=::std::ptr::null() {
            match unsafe { ffi::FMOD_ChannelGroup_Release(self.channel_group) } {
                fmod::Ok => {
                    self.channel_group =::std::ptr::null();
                    fmod::Ok
                }
                e => e
            }
        } else {
            fmod::Ok
        }
    }

    pub fn set_volume(&self, volume: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_SetVolume(self.channel_group, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, fmod::Result> {
        let volume = 0f32;

        match unsafe { ffi::FMOD_ChannelGroup_GetVolume(self.channel_group, &volume) } {
            fmod::Ok => Ok(volume),
            e => Err(e)
        }
    }

    pub fn set_pitch(&self, pitch: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_SetPitch(self.channel_group, pitch) }
    }

    pub fn get_pitch(&self) -> Result<f32, fmod::Result> {
        let pitch = 0f32;

        match unsafe { ffi::FMOD_ChannelGroup_GetPitch(self.channel_group, &pitch) } {
            fmod::Ok => Ok(pitch),
            e => Err(e)
        }
    }

    pub fn set_paused(&self, paused: bool) -> fmod::Result {
        let t_paused = match paused {
            true => 1,
            _ => 0
        };

        unsafe { ffi::FMOD_ChannelGroup_SetPaused(self.channel_group, t_paused) }
    }

    pub fn get_paused(&self) -> Result<bool, fmod::Result> {
        let paused = 0;

        match unsafe { ffi::FMOD_ChannelGroup_GetPaused(self.channel_group, &paused) } {
            fmod::Ok => Ok(match paused {
                1 => true,
                _ => false
            }),
            e => Err(e)
        }
    }

    pub fn set_mute(&self, mute: bool) -> fmod::Result {
        let t_mute = match mute {
            true => 1,
            _ => 0
        };

        unsafe { ffi::FMOD_ChannelGroup_SetMute(self.channel_group, t_mute) }
    }

    pub fn get_mute(&self) -> Result<bool, fmod::Result> {
        let mute = 0;

        match unsafe { ffi::FMOD_ChannelGroup_GetMute(self.channel_group, &mute) } {
            fmod::Ok => Ok(match mute {
                1 => true,
                _ => false
            }),
            e => Err(e)
        }
    }

    pub fn set_3D_occlusion(&self, direct_occlusion: f32, reverb_occlusion: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_Set3DOcclusion(self.channel_group, direct_occlusion, reverb_occlusion) }
    }

    pub fn get_3D_occlusion(&self) -> Result<(f32, f32), fmod::Result> {
        let direct_occlusion = 0f32;
        let reverb_occlusion = 0f32;

        match unsafe { ffi::FMOD_ChannelGroup_Get3DOcclusion(self.channel_group, &direct_occlusion, &reverb_occlusion) } {
            fmod::Ok => Ok((direct_occlusion, reverb_occlusion)),
            e => Err(e)
        }
    }

    pub fn stop(&self) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_Stop(self.channel_group) }
    }

    pub fn override_volume(&self, volume: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_OverrideVolume(self.channel_group, volume) }
    }

    pub fn override_frequency(&self, frequency: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_OverrideFrequency(self.channel_group, frequency) }
    }

    pub fn override_pan(&self, pan: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_OverridePan(self.channel_group, pan) }
    }

    pub fn override_reverb_properties(&self, properties: channel::FmodReverbChannelProperties) -> fmod::Result {
        let prop = ffi::FMOD_REVERB_CHANNELPROPERTIES {Direct: properties.direct, Room: properties.room, Flags: properties.flags,
            ConnectionPoint: dsp::get_ffi(&properties.connection_point)};

        unsafe { ffi::FMOD_ChannelGroup_OverrideReverbProperties(self.channel_group, &prop) }
    }

    pub fn override_3D_attributes(&self, pos: vector::FmodVector, vel: vector::FmodVector) -> fmod::Result {
        let t_pos = vector::get_ffi(&pos);
        let t_vel = vector::get_ffi(&vel);

        unsafe { ffi::FMOD_ChannelGroup_Override3DAttributes(self.channel_group, &t_pos, &t_vel) }
    }

    pub fn override_speaker_mix(&self, front_left: f32, front_right: f32, center: f32, lfe: f32, back_left: f32, back_right: f32,
        side_left: f32, side_right: f32) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_OverrideSpeakerMix(self.channel_group, front_left, front_right, center, lfe, back_left, back_right, side_left, side_right) }
    }

    pub fn add_group(&self, group: ChannelGroup) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_AddGroup(self.channel_group, group.channel_group) }
    }

    pub fn get_num_groups(&self) -> Result<i32, fmod::Result> {
        let index = 0i32;

        match unsafe { ffi::FMOD_ChannelGroup_GetNumGroups(self.channel_group, &index) } {
            fmod::Ok => Ok(index),
            e => Err(e)
        }
    }

    pub fn get_group(&self, index: i32) -> Result<ChannelGroup, fmod::Result> {
        let group =::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetGroup(self.channel_group, index, &group) } {
            fmod::Ok => Ok(ChannelGroup{channel_group: group}),
            e => Err(e)
        }
    }

    pub fn get_parent_group(&self) -> Result<ChannelGroup, fmod::Result> {
        let parent_group =::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetParentGroup(self.channel_group, &parent_group) } {
            fmod::Ok => Ok(ChannelGroup{channel_group: parent_group}),
            e => Err(e)
        }
    }

    pub fn get_DSP_head(&self) -> Result<dsp::Dsp, fmod::Result> {
        let dsp =::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetDSPHead(self.channel_group, &dsp) } {
            fmod::Ok => Ok(dsp::from_ptr(dsp)),
            e => Err(e)
        }
    }

    pub fn add_DSP(&self, dsp: dsp::Dsp) -> Result<dsp_connection::DspConnection, fmod::Result> {
        let dsp_connection =::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_AddDSP(self.channel_group, dsp::get_ffi(&dsp), &dsp_connection) } {
            fmod::Ok => Ok(dsp_connection::from_ptr(dsp_connection)),
            e => Err(e)
        }
    }

    pub fn get_name(&self, name_len: u32) -> Result<String, fmod::Result> {
        let name = String::with_capacity(name_len as uint);

        name.with_c_str(|c_name|{
            match unsafe { ffi::FMOD_ChannelGroup_GetName(self.channel_group, c_name, name_len as i32) } {
                fmod::Ok => Ok(unsafe {::std::str::raw::from_c_str(c_name).clone() }),
                e => Err(e)
            }
        })
    }

    pub fn get_num_channels(&self) -> Result<u32, fmod::Result> {
        let num_channels = 0i32;

        match unsafe { ffi::FMOD_ChannelGroup_GetNumChannels(self.channel_group, &num_channels) } {
            fmod::Ok => Ok(num_channels as u32),
            e => Err(e)
        }
    }

    pub fn get_channel(&self, index: i32) -> Result<channel::Channel, fmod::Result> {
        let channel =::std::ptr::null();

        match unsafe { ffi::FMOD_ChannelGroup_GetChannel(self.channel_group, index, &channel) } {
            fmod::Ok => Ok(channel::from_ptr(channel)),
            e => Err(e)
        }
    }

    pub fn get_spectrum(&self, spectrum_size: uint, options: Option<channel::FmodSpectrumOptions>) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(spectrum_size, 0f32);
        let mut window_type = fmod::DSP_FFT_WindowRect;
        let mut channel_offset = 0;

        match options {
            Some(v) => {
                window_type = v.window_type;
                channel_offset = v.channel_offset;
            }
            None => {}
        };
        match unsafe { ffi::FMOD_ChannelGroup_GetSpectrum(self.channel_group, ptr.as_ptr(), spectrum_size as c_int, channel_offset, window_type) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size: uint, channel_offset: i32) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(wave_size, 0f32);

        match unsafe { ffi::FMOD_ChannelGroup_GetWaveData(self.channel_group, ptr.as_ptr(), wave_size as c_int, channel_offset) } {
            fmod::Ok => Ok(ptr),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let details = fmod_sys::get_memory_usage_details_ffi(FmodMemoryUsageDetails::new());
        let memory_used = 0u32;

        match unsafe { ffi::FMOD_ChannelGroup_GetMemoryInfo(self.channel_group, memory_bits, event_memory_bits, &memory_used, &details) } {
            fmod::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    /* to test ! */
    pub fn set_user_data<T>(&self, user_data: T) -> fmod::Result {
        unsafe { ffi::FMOD_ChannelGroup_SetUserData(self.channel_group, transmute(user_data)) }
    }

    /* to test ! */
    pub fn get_user_data<T>(&self) -> Result<T, fmod::Result> {
        unsafe {
            let user_data =::std::ptr::null();

            match ffi::FMOD_ChannelGroup_GetUserData(self.channel_group, &user_data) {
                fmod::Ok => Ok(transmute(user_data)),
                e => Err(e)
            }
        }
    }
}