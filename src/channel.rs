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
use libc::{c_int, c_uint};
use ffi;
use dsp;
use dsp::Dsp;
use dsp_connection;
use dsp_connection::DspConnection;
use channel_group;
use channel_group::ChannelGroup;
use fmod_sys;
use fmod_sys::{FmodMemoryUsageDetails, FmodSys};
use vector;
use sound;
use sound::Sound;
use std::mem::transmute;

pub struct FmodSpectrumOptions {
    pub window_type    : fmod::DSP_FFT_Window,
    pub channel_offset : i32
}

pub struct FmodDelayOptions {
    pub delaytype  : fmod::DelayType,
    pub delayhi    : uint,
    pub delaylo    : uint
}

pub struct FmodSpeakerMixOptions {
    pub front_left : f32,
    pub front_right: f32,
    pub center     : f32,
    pub lfe        : f32,
    pub back_left  : f32,
    pub back_right : f32,
    pub side_left  : f32,
    pub side_right : f32
}

pub struct FmodReverbChannelProperties {
    pub direct          : c_int,
    pub room            : c_int,
    pub flags           : c_uint,
    pub connection_point: Dsp
}

pub fn get_ffi(channel: &Channel) -> *ffi::FMOD_CHANNEL {
    &channel.channel as *ffi::FMOD_CHANNEL
}

pub fn new() -> Channel {
    Channel{channel: ::std::ptr::null()}
}

pub fn from_ptr(channel: ffi::FMOD_CHANNEL) -> Channel {
    Channel{channel: channel}
}

pub struct Channel {
    channel: ffi::FMOD_CHANNEL
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.release();
    }
}

impl Channel {
    pub fn release(&mut self) {
        self.channel =::std::ptr::null();
    }

    pub fn get_system_object(&self) -> Result<FmodSys, fmod::Result> {
        let system =::std::ptr::null();

        match unsafe { ffi::FMOD_Channel_GetSystemObject(self.channel, &system) } {
            fmod::Ok => Ok(fmod_sys::from_ptr(system)),
            e => Err(e)
        }
    }

    pub fn stop(&self) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Stop(self.channel) }
    }

    // channel_offset: 0 -> left channel, 1 -> right channel
    pub fn get_spectrum(&self, spectrum_size: uint, channel_offset: i32, window_type: fmod::DSP_FFT_Window) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(spectrum_size, 0f32);

        match unsafe { ffi::FMOD_Channel_GetSpectrum(self.channel, ptr.as_ptr(), spectrum_size as c_int, channel_offset, window_type) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size: uint, channel_offset: i32) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(wave_size, 0f32);

        match unsafe { ffi::FMOD_Channel_GetWaveData(self.channel, ptr.as_ptr(), wave_size as c_int, channel_offset) } {
            fmod::Ok => Ok(ptr),
            e => Err(e)
        }
    }

    pub fn is_init(&self) -> bool {
        self.channel !=::std::ptr::null()
    }

    pub fn is_playing(&self) -> Result<bool, fmod::Result> {
        let is_playing = 0;

        match unsafe { ffi::FMOD_Channel_IsPlaying(self.channel, &is_playing) } {
            fmod::Ok => Ok(is_playing == 1),
            err => Err(err),
        }
    }

    pub fn is_virtual(&self) -> Result<bool, fmod::Result> {
        let is_virtual = 0i32;

        match unsafe { ffi::FMOD_Channel_IsVirtual(self.channel, &is_virtual) } {
            fmod::Ok => Ok(is_virtual == 1),
            e => Err(e)
        }
    }

    pub fn get_audibility(&self) -> Result<f32, fmod::Result> {
        let audibility = 0f32;

        match unsafe { ffi::FMOD_Channel_GetAudibility(self.channel, &audibility) } {
            fmod::Ok => Ok(audibility),
            e => Err(e)
        }
    }

    pub fn get_current_sound(&self) -> Result<Sound, fmod::Result> {
        let sound = ::std::ptr::null();

        match unsafe { ffi::FMOD_Channel_GetCurrentSound(self.channel, &sound) } {
            fmod::Ok => Ok(sound::from_ptr(sound)),
            e => Err(e)
        }
    }

    pub fn get_index(&self) -> Result<i32, fmod::Result> {
        let index = 0i32;

        match unsafe { ffi::FMOD_Channel_GetIndex(self.channel, &index) } {
            fmod::Ok => Ok(index),
            e => Err(e)
        }
    }

    pub fn set_volume(&self, volume: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetVolume(self.channel, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, fmod::Result> {
        let volume = 0f32;

        match unsafe { ffi::FMOD_Channel_GetVolume(self.channel, &volume) } {
            fmod::Ok => Ok(volume),
            e => Err(e),
        }
    }

    pub fn set_frequency(&self, frequency: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetFrequency(self.channel, frequency) }
    }

    pub fn get_frequency(&self) -> Result<f32, fmod::Result> {
        let frequency = 0f32;

        match unsafe { ffi::FMOD_Channel_GetFrequency(self.channel, &frequency) } {
            fmod::Ok => Ok(frequency),
            e => Err(e),
        }
    }

    pub fn set_pan(&self, pan: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetPan(self.channel, pan) }
    }

    pub fn get_pan(&self) -> Result<f32, fmod::Result> {
        let pan = 0f32;

        match unsafe { ffi::FMOD_Channel_GetPan(self.channel, &pan) } {
            fmod::Ok => Ok(pan),
            e => Err(e),
        }
    }

    pub fn set_mute(&self, mute: bool) -> fmod::Result {
        let t = match mute {
            true => 1,
            false => 0,
        };
        unsafe { ffi::FMOD_Channel_SetMute(self.channel, t) }
    }

    pub fn get_mute(&self) -> Result<bool, fmod::Result> {
        let mute = 0;

        match unsafe { ffi::FMOD_Channel_GetMute(self.channel, &mute) } {
            fmod::Ok => Ok(match mute {
                1 => true,
                _ => false,
            }),
            e => Err(e),
        }
    }

    pub fn set_paused(&self, paused: bool) -> fmod::Result {
        let t: ffi::FMOD_BOOL = match paused {
            true => 1,
            false => 0,
        };
        unsafe { ffi::FMOD_Channel_SetPaused(self.channel, t) }
    }

    pub fn get_paused(&self) -> Result<bool, fmod::Result> {
        let t = 0;

        match unsafe { ffi::FMOD_Channel_GetPaused(self.channel, &t) } {
            fmod::Ok => Ok(match t {
                1 => true,
                _ => false,
            }),
            e => Err(e),
        }
    }

    pub fn set_delay(&self, d_o: FmodDelayOptions) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetDelay(self.channel, d_o.delaytype, d_o.delayhi as u32, d_o.delaylo as u32) }
    }

    pub fn get_delay(&self, delaytype: fmod::DelayType) -> Result<FmodDelayOptions, fmod::Result> {
        let delaylo = 0u32;
        let delayhi = 0u32;

        match unsafe { ffi::FMOD_Channel_GetDelay(self.channel, delaytype, &delayhi, &delaylo) } {
            fmod::Ok => Ok(FmodDelayOptions{
                delaytype: delaytype,
                delayhi: delayhi as uint,
                delaylo: delaylo as uint}),
            e => Err(e),
        }
    }

    pub fn set_speaker_mix(&self, smo: FmodSpeakerMixOptions) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetSpeakerMix(self.channel, smo.front_left, smo.front_right, smo.center, smo.lfe,
                                            smo.back_left, smo.back_right, smo.side_left, smo.side_right) }
    }

    pub fn get_speaker_mix(&self) -> Result<FmodSpeakerMixOptions, fmod::Result> {
        let smo = FmodSpeakerMixOptions{front_left: 0f32, front_right: 0f32, center: 0f32, lfe: 0f32, back_left: 0f32,
                                    back_right: 0f32, side_left: 0f32, side_right: 0f32};

        match unsafe { ffi::FMOD_Channel_GetSpeakerMix(self.channel, &smo.front_left, &smo.front_right, &smo.center, &smo.lfe,
                                                &smo.back_left, &smo.back_right, &smo.side_left, &smo.side_right) } {
            fmod::Ok => Ok(smo),
            e => Err(e),
        }
    }

    pub fn set_speaker_level(&self, speaker: fmod::Speaker, levels: Vec<f32>) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetSpeakerLevels(self.channel, speaker, levels.as_ptr(), levels.len() as i32) }
    }

    pub fn get_speaker_level(&self, speaker: fmod::Speaker, num_levels: uint) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(num_levels, 0f32);

        match unsafe { ffi::FMOD_Channel_GetSpeakerLevels(self.channel, speaker, ptr.as_ptr(), num_levels as i32) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn set_input_channel_mix(&self, levels: Vec<f32>) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetInputChannelMix(self.channel, levels.as_ptr(), levels.len() as i32) }
    }

    pub fn get_input_channel_mix(&self, num_levels: uint) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(num_levels, 0f32);

        match unsafe { ffi::FMOD_Channel_GetInputChannelMix(self.channel, ptr.as_ptr(), num_levels as i32) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn set_priority(&self, priority: i32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetPriority(self.channel, priority) }
    }

    pub fn get_priority(&self) -> Result<i32, fmod::Result> {
        let t = 0i32;

        match unsafe { ffi::FMOD_Channel_GetPriority(self.channel, &t) } {
            fmod::Ok => Ok(t),
            e => Err(e),
        }
    }

    pub fn set_position(&self, position: uint, FmodTimeUnit(postype): FmodTimeUnit) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetPosition(self.channel, position as u32, postype) }
    }

    pub fn get_position(&self, FmodTimeUnit(postype): FmodTimeUnit) -> Result<uint, fmod::Result> {
        let t = 0u32;

        match unsafe { ffi::FMOD_Channel_GetPosition(self.channel, &t, postype) } {
            fmod::Ok => Ok(t as uint),
            e => Err(e),
        }
    }

    pub fn set_reverb_properties(&self, prop: FmodReverbChannelProperties) -> fmod::Result {
        let t = ffi::FMOD_REVERB_CHANNELPROPERTIES{Direct: prop.direct, Room: prop.room, Flags: prop.flags, ConnectionPoint: ::std::ptr::null()};

        unsafe { ffi::FMOD_Channel_SetReverbProperties(self.channel, &t) }
    }

    pub fn get_reverb_properties(&self) -> Result<FmodReverbChannelProperties, fmod::Result> {
        let t = ffi::FMOD_REVERB_CHANNELPROPERTIES{Direct: 0, Room: 0, Flags: 0, ConnectionPoint: ::std::ptr::null()};

        match unsafe { ffi::FMOD_Channel_GetReverbProperties(self.channel, &t) } {
            fmod::Ok => Ok(FmodReverbChannelProperties{
                direct: t.Direct,
                room: t.Room,
                flags: t.Flags,
                connection_point: dsp::from_ptr(t.ConnectionPoint)}),
            e => Err(e),
        }
    }

    pub fn set_low_pass_gain(&self, gain: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetLowPassGain(self.channel, gain) }
    }

    pub fn get_low_pass_gain(&self) -> Result<f32, fmod::Result> {
        let t = 0f32;

        match unsafe { ffi::FMOD_Channel_GetLowPassGain(self.channel, &t) } {
            fmod::Ok => Ok(t),
            e => Err(e),
        }
    }

    pub fn set_channel_group(&mut self, channel_group: ChannelGroup) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetChannelGroup(self.channel, channel_group::get_ffi(channel_group)) }
    }

    pub fn get_channel_group(&self) -> Result<ChannelGroup, fmod::Result> {
        let channel_group =::std::ptr::null();

        match unsafe { ffi::FMOD_Channel_GetChannelGroup(self.channel, &channel_group) } {
            fmod::Ok => Ok(channel_group::from_ptr(channel_group)),
            e => Err(e)
        }
    }

    pub fn set_3D_attributes(&self, position: vector::FmodVector, velocity: vector::FmodVector) -> fmod::Result {
        let t_position = vector::get_ffi(&position);
        let t_velocity = vector::get_ffi(&velocity);

        unsafe { ffi::FMOD_Channel_Set3DAttributes(self.channel, &t_position, &t_velocity) }
    }

    pub fn get_3D_attributes(&self) -> Result<(vector::FmodVector, vector::FmodVector), fmod::Result> {
        let position = vector::get_ffi(&vector::new());
        let velocity = vector::get_ffi(&vector::new());

        match unsafe { ffi::FMOD_Channel_Get3DAttributes(self.channel, &position, &velocity) } {
            fmod::Ok => Ok((vector::from_ptr(position), vector::from_ptr(velocity))),
            e => Err(e)
        }
    }

    pub fn set_3D_min_max_distance(&self, min_distance: f32, max_distance: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DMinMaxDistance(self.channel, min_distance, max_distance) }
    }

    pub fn get_3D_min_max_distance(&self) -> Result<(f32, f32), fmod::Result> {
        let min_distance = 0f32;
        let max_distance = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DMinMaxDistance(self.channel, &min_distance, &max_distance) } {
            fmod::Ok => Ok((min_distance, max_distance)),
            e => Err(e)
        }
    }

    pub fn set_3D_cone_settings(&self, inside_cone_angle: f32, outside_cone_angle: f32, outside_volume: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DConeSettings(self.channel, inside_cone_angle, outside_cone_angle, outside_volume) }
    }

    pub fn get_3D_cone_settings(&self) -> Result<(f32, f32, f32), fmod::Result> {
        let inside_cone_angle = 0f32;
        let outside_cone_angle = 0f32;
        let outside_volume = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DConeSettings(self.channel, &inside_cone_angle, &outside_cone_angle, &outside_volume) } {
            fmod::Ok => Ok((inside_cone_angle, outside_cone_angle, outside_volume)),
            e => Err(e)
        }
    }

    pub fn set_3D_cone_orientation(&self, orientation: vector::FmodVector) -> fmod::Result {
        let t_orientation = vector::get_ffi(&orientation);

        unsafe { ffi::FMOD_Channel_Set3DConeOrientation(self.channel, &t_orientation) }
    }

    pub fn get_3D_cone_orientation(&self) -> Result<vector::FmodVector, fmod::Result> {
        let orientation = vector::get_ffi(&vector::new());

        match unsafe { ffi::FMOD_Channel_Get3DConeOrientation(self.channel, &orientation) } {
            fmod::Ok => Ok(vector::from_ptr(orientation)),
            e => Err(e)
        }
    }

    pub fn set_3D_custom_roll_off(&self, points: Vec<vector::FmodVector>) -> fmod::Result {
        let mut t_points = Vec::new();

        for tmp in points.iter() {
            t_points.push(vector::get_ffi(tmp));
        }
        unsafe { ffi::FMOD_Channel_Set3DCustomRolloff(self.channel, t_points.as_ptr(), points.len() as c_int) }
    }

    pub fn get_3D_custom_roll_off(&self) -> Result<Vec<vector::FmodVector>, fmod::Result> {
        let points = ::std::ptr::null();
        let num_points = 0i32;

        unsafe {
            match ffi::FMOD_Channel_Get3DCustomRolloff(self.channel, &points, &num_points) {
                fmod::Ok => {
                    let mut ret_points = Vec::new();
                    let mut it = 0i32;

                    while it < num_points {
                        let tmp = points.offset(it as int);

                        ret_points.push(vector::from_ptr(*tmp));
                        it += 1;
                    }
                    Ok(ret_points)
                }
                e => Err(e)
            }
        }
    }

    pub fn set_3D_occlusion(&self, direct_occlusion: f32, reverb_occlusion: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DOcclusion(self.channel, direct_occlusion, reverb_occlusion) }
    }

    pub fn get_3D_occlusion(&self) -> Result<(f32, f32), fmod::Result> {
        let direct_occlusion = 0f32;
        let reverb_occlusion = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DOcclusion(self.channel, &direct_occlusion, &reverb_occlusion) } {
            fmod::Ok => Ok((direct_occlusion, reverb_occlusion)),
            e => Err(e)
        }
    }

    pub fn set_3D_spread(&self, angle: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DSpread(self.channel, angle) }
    }

    pub fn get_3D_spread(&self) -> Result<f32, fmod::Result> {
        let angle = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DSpread(self.channel, &angle) } {
            fmod::Ok => Ok(angle),
            e => Err(e)
        }
    }

    pub fn set_3D_pan_level(&self, level: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DPanLevel(self.channel, level) }
    }

    pub fn get_3D_pan_level(&self) -> Result<f32, fmod::Result> {
        let level = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DPanLevel(self.channel, &level) } {
            fmod::Ok => Ok(level),
            e => Err(e)
        }
    }

    pub fn set_3D_doppler_level(&self, level: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DDopplerLevel(self.channel, level) }
    }

    pub fn get_3D_doppler_level(&self) -> Result<f32, fmod::Result> {
        let level = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DDopplerLevel(self.channel, &level) } {
            fmod::Ok => Ok(level),
            e => Err(e)
        }
    }

    pub fn set_3D_distance_filter(&self, custom: bool, custom_level: f32, center_freq: f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_Set3DDistanceFilter(self.channel, if custom {
                1
            } else {
                0
            }, custom_level, center_freq) }
    }

    pub fn get_3D_distance_filter(&self) -> Result<(bool, f32, f32), fmod::Result> {
        let custom = 0i32;
        let custom_level = 0f32;
        let center_freq = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DDistanceFilter(self.channel, &custom, &custom_level, &center_freq) } {
            fmod::Ok => Ok((custom == 1, custom_level, center_freq)),
            e => Err(e)
        }
    }

    pub fn get_DSP_head(&self) -> Result<Dsp, fmod::Result> {
        let dsp = ::std::ptr::null();

        match unsafe { ffi::FMOD_Channel_GetDSPHead(self.channel, &dsp) } {
            fmod::Ok => Ok(dsp::from_ptr(dsp)),
            e => Err(e)
        }
    }

    pub fn add_DSP(&self, dsp: &Dsp) -> Result<DspConnection, fmod::Result> {
        let connection = ::std::ptr::null();

        match unsafe { ffi::FMOD_Channel_AddDSP(self.channel, dsp::get_ffi(dsp), &connection) } {
            fmod::Ok => Ok(dsp_connection::from_ptr(connection)),
            e => Err(e)
        }
    }

    pub fn set_mode(&self, FmodMode(mode): FmodMode) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetMode(self.channel, mode) }
    }

    pub fn get_mode(&self) -> Result<FmodMode, fmod::Result> {
        let mode = 0u32;

        match unsafe { ffi::FMOD_Channel_GetMode(self.channel, &mode) } {
            fmod::Ok => Ok(FmodMode(mode)),
            e => Err(e)
        }
    }

    pub fn set_loop_count(&self, loop_count: i32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetLoopCount(self.channel, loop_count) }
    }

    pub fn get_loop_count(&self) -> Result<i32, fmod::Result> {
        let loop_count = 0i32;

        match unsafe { ffi::FMOD_Channel_GetLoopCount(self.channel, &loop_count) } {
            fmod::Ok => Ok(loop_count),
            e => Err(e)
        }
    }

    pub fn set_loop_points(&self, loop_start: u32, FmodTimeUnit(loop_start_type): FmodTimeUnit,
        loop_end: u32, FmodTimeUnit(loop_end_type): FmodTimeUnit) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetLoopPoints(self.channel, loop_start, loop_start_type, loop_end, loop_end_type) }
    }

    pub fn get_loop_points(&self, FmodTimeUnit(loop_start_type): FmodTimeUnit, FmodTimeUnit(loop_end_type): FmodTimeUnit) -> Result<(u32, u32), fmod::Result> {
        let loop_start = 0u32;
        let loop_end = 0u32;

        match unsafe { ffi::FMOD_Channel_GetLoopPoints(self.channel, &loop_start, loop_start_type, &loop_end, loop_end_type) } {
            fmod::Ok => Ok((loop_start, loop_end)),
            e => Err(e)
        }
    }

    /* to test ! */
    pub fn set_user_data<T>(&self, user_data: T) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetUserData(self.channel, transmute(user_data)) }
    }

    /* to test ! */
    pub fn get_user_data<T>(&self) -> Result<T, fmod::Result> {
        unsafe {
            let user_data =::std::ptr::null();

            match ffi::FMOD_Channel_GetUserData(self.channel, &user_data) {
                fmod::Ok => Ok(transmute(user_data)),
                e => Err(e)
            }
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let details = fmod_sys::get_memory_usage_details_ffi(FmodMemoryUsageDetails::new());
        let memory_used = 0u32;

        match unsafe { ffi::FMOD_Channel_GetMemoryInfo(self.channel, memory_bits, event_memory_bits, &memory_used, &details) } {
            fmod::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }
}