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

use types::*;
use libc::{c_int, c_void};
use ffi;
use dsp::Dsp;
use dsp_connection::DspConnection;
use channel_group::ChannelGroup;
use fmod_sys;
use fmod_sys::{MemoryUsageDetails, Sys};
use vector;
use sound::Sound;
use std::mem::transmute;
use std::default::Default;

/// Structure which contains data for [`Channel::set_speaker_mix`](struct.Channel.html#method.set_speaker_mix) and [`Channel::get_speaker_mix`](struct.Channel.html#method.get_speaker_mix)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct SpeakerMixOptions {
    pub front_left : f32,
    pub front_right: f32,
    pub center     : f32,
    pub lfe        : f32,
    pub back_left  : f32,
    pub back_right : f32,
    pub side_left  : f32,
    pub side_right : f32
}

impl Default for SpeakerMixOptions {
    fn default() -> SpeakerMixOptions {
        SpeakerMixOptions {
            front_left: 0f32,
            front_right: 0f32,
            center: 0f32,
            lfe: 0f32,
            back_left: 0f32,
            back_right: 0f32,
            side_left: 0f32,
            side_right: 0f32
        }
    }
}

/// Structure defining the properties for a reverb source, related to a FMOD channel.
pub struct ReverbChannelProperties {
    /// [r/w] MIN: -10000 MAX: 1000 DEFAULT: 0 - Direct path level
    pub direct          : i32,
    /// [r/w] MIN: -10000 MAX: 1000 DEFAULT: 0 - Room effect level
    pub room            : i32,
    /// [r/w] FMOD_REVERB_CHANNELFLAGS         - modifies the behavior of properties
    pub flags           : u32,
    /// [r/w] See remarks.                    - DSP network location to connect reverb for this channel.
    pub connection_point: Dsp
}

/// Channel Object
pub struct Channel {
    channel: *mut ffi::FMOD_CHANNEL
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.release();
    }
}

impl ffi::FFI<ffi::FMOD_CHANNEL> for Channel {
    fn wrap(channel: *mut ffi::FMOD_CHANNEL) -> Channel {
        Channel {channel: channel}
    }

    fn unwrap(c: &Channel) -> *mut ffi::FMOD_CHANNEL {
        c.channel
    }
}

impl Channel {
    pub fn new() -> Channel {
        Channel {channel: ::std::ptr::null_mut()}
    }

    pub fn release(&mut self) {
        self.channel = ::std::ptr::null_mut();
    }

    pub fn get_system_object(&self) -> Result<Sys, ::Result> {
        let mut system = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_Channel_GetSystemObject(self.channel, &mut system) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(system)),
            e => Err(e)
        }
    }

    pub fn stop(&self) -> ::Result {
        unsafe { ffi::FMOD_Channel_Stop(self.channel) }
    }

    /// channel_offset:  0/1 -> left channel/right channel
    pub fn get_spectrum(&self, spectrum_size: usize, channel_offset: Option<i32>, window_type: Option<::DspFftWindow>) -> Result<Vec<f32>, ::Result> {
        let mut ptr : Vec<f32> = ::std::iter::repeat(0f32).take(spectrum_size).collect();
        let c_window_type = match window_type {
            Some(wt) => wt,
            None => ::DspFftWindow::Rect
        };
        let c_channel_offset = match channel_offset {
            Some(co) => co,
            None => 0i32
        };

        match unsafe { ffi::FMOD_Channel_GetSpectrum(self.channel, ptr.as_mut_ptr(), spectrum_size as c_int, c_channel_offset, c_window_type) } {
            ::Result::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size: usize, channel_offset: i32) -> Result<Vec<f32>, ::Result> {
        let mut ptr : Vec<f32> = ::std::iter::repeat(0f32).take(wave_size).collect();

        match unsafe { ffi::FMOD_Channel_GetWaveData(self.channel, ptr.as_mut_ptr(), wave_size as c_int, channel_offset) } {
            ::Result::Ok => Ok(ptr),
            e => Err(e)
        }
    }

    pub fn is_init(&self) -> bool {
        !self.channel.is_null()
    }

    pub fn is_playing(&self) -> Result<bool, ::Result> {
        let mut is_playing = 0;

        match unsafe { ffi::FMOD_Channel_IsPlaying(self.channel, &mut is_playing) } {
            ::Result::Ok => Ok(is_playing == 1),
            err => Err(err),
        }
    }

    pub fn is_virtual(&self) -> Result<bool, ::Result> {
        let mut is_virtual = 0i32;

        match unsafe { ffi::FMOD_Channel_IsVirtual(self.channel, &mut is_virtual) } {
            ::Result::Ok => Ok(is_virtual == 1),
            e => Err(e)
        }
    }

    pub fn get_audibility(&self) -> Result<f32, ::Result> {
        let mut audibility = 0f32;

        match unsafe { ffi::FMOD_Channel_GetAudibility(self.channel, &mut audibility) } {
            ::Result::Ok => Ok(audibility),
            e => Err(e)
        }
    }

    pub fn get_current_sound(&self) -> Result<Sound, ::Result> {
        let mut sound = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_Channel_GetCurrentSound(self.channel, &mut sound) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(sound)),
            e => Err(e)
        }
    }

    pub fn get_index(&self) -> Result<i32, ::Result> {
        let mut index = 0i32;

        match unsafe { ffi::FMOD_Channel_GetIndex(self.channel, &mut index) } {
            ::Result::Ok => Ok(index),
            e => Err(e)
        }
    }

    pub fn set_volume(&self, volume: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetVolume(self.channel, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, ::Result> {
        let mut volume = 0f32;

        match unsafe { ffi::FMOD_Channel_GetVolume(self.channel, &mut volume) } {
            ::Result::Ok => Ok(volume),
            e => Err(e),
        }
    }

    pub fn set_frequency(&self, frequency: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetFrequency(self.channel, frequency) }
    }

    pub fn get_frequency(&self) -> Result<f32, ::Result> {
        let mut frequency = 0f32;

        match unsafe { ffi::FMOD_Channel_GetFrequency(self.channel, &mut frequency) } {
            ::Result::Ok => Ok(frequency),
            e => Err(e),
        }
    }

    pub fn set_pan(&self, pan: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetPan(self.channel, pan) }
    }

    pub fn get_pan(&self) -> Result<f32, ::Result> {
        let mut pan = 0f32;

        match unsafe { ffi::FMOD_Channel_GetPan(self.channel, &mut pan) } {
            ::Result::Ok => Ok(pan),
            e => Err(e),
        }
    }

    pub fn set_mute(&self, mute: bool) -> ::Result {
        let t = match mute {
            true => 1,
            false => 0,
        };
        unsafe { ffi::FMOD_Channel_SetMute(self.channel, t) }
    }

    pub fn get_mute(&self) -> Result<bool, ::Result> {
        let mut mute = 0;

        match unsafe { ffi::FMOD_Channel_GetMute(self.channel, &mut mute) } {
            ::Result::Ok => Ok(match mute {
                1 => true,
                _ => false,
            }),
            e => Err(e),
        }
    }

    pub fn set_paused(&self, paused: bool) -> ::Result {
        let t: ffi::FMOD_BOOL = match paused {
            true => 1,
            false => 0,
        };
        unsafe { ffi::FMOD_Channel_SetPaused(self.channel, t) }
    }

    pub fn get_paused(&self) -> Result<bool, ::Result> {
        let mut t = 0;

        match unsafe { ffi::FMOD_Channel_GetPaused(self.channel, &mut t) } {
            ::Result::Ok => Ok(match t {
                1 => true,
                _ => false,
            }),
            e => Err(e),
        }
    }

    pub fn set_delay(&self, delay_type: ::DelayType, delay_hi: usize, delay_lo: usize) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetDelay(self.channel, delay_type, delay_hi as u32, delay_lo as u32) }
    }

    pub fn get_delay(&self, delay_type: ::DelayType) -> Result<(::DelayType, usize, usize), ::Result> {
        let mut delaylo = 0u32;
        let mut delayhi = 0u32;

        match unsafe { ffi::FMOD_Channel_GetDelay(self.channel, delay_type, &mut delayhi, &mut delaylo) } {
            ::Result::Ok => Ok((delay_type, delayhi as usize, delaylo as usize)),
            e => Err(e),
        }
    }

    pub fn set_speaker_mix(&self, smo: &SpeakerMixOptions) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetSpeakerMix(self.channel, smo.front_left, smo.front_right, smo.center, smo.lfe,
                                            smo.back_left, smo.back_right, smo.side_left, smo.side_right) }
    }

    pub fn get_speaker_mix(&self) -> Result<SpeakerMixOptions, ::Result> {
        let mut smo = SpeakerMixOptions{front_left: 0f32, front_right: 0f32, center: 0f32, lfe: 0f32, back_left: 0f32,
                                    back_right: 0f32, side_left: 0f32, side_right: 0f32};

        match unsafe { ffi::FMOD_Channel_GetSpeakerMix(self.channel, &mut smo.front_left, &mut smo.front_right, &mut smo.center, &mut smo.lfe,
                                                &mut smo.back_left, &mut smo.back_right, &mut smo.side_left, &mut smo.side_right) } {
            ::Result::Ok => Ok(smo),
            e => Err(e),
        }
    }

    pub fn set_speaker_level(&self, speaker: ::Speaker, levels: &mut Vec<f32>) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetSpeakerLevels(self.channel, speaker, levels.as_mut_ptr(), levels.len() as i32) }
    }

    pub fn get_speaker_level(&self, speaker: ::Speaker, num_levels: usize) -> Result<Vec<f32>, ::Result> {
        let mut ptr : Vec<f32> = ::std::iter::repeat(0f32).take(num_levels).collect();

        match unsafe { ffi::FMOD_Channel_GetSpeakerLevels(self.channel, speaker, ptr.as_mut_ptr(), num_levels as i32) } {
            ::Result::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn set_input_channel_mix(&self, levels: &mut Vec<f32>) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetInputChannelMix(self.channel, levels.as_mut_ptr(), levels.len() as i32) }
    }

    pub fn get_input_channel_mix(&self, num_levels: usize) -> Result<Vec<f32>, ::Result> {
        let mut ptr : Vec<f32> = ::std::iter::repeat(0f32).take(num_levels).collect();

        match unsafe { ffi::FMOD_Channel_GetInputChannelMix(self.channel, ptr.as_mut_ptr(), num_levels as i32) } {
            ::Result::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn set_priority(&self, priority: i32) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetPriority(self.channel, priority) }
    }

    pub fn get_priority(&self) -> Result<i32, ::Result> {
        let mut t = 0i32;

        match unsafe { ffi::FMOD_Channel_GetPriority(self.channel, &mut t) } {
            ::Result::Ok => Ok(t),
            e => Err(e),
        }
    }

    pub fn set_position(&self, position: usize, TimeUnit(postype): TimeUnit) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetPosition(self.channel, position as u32, postype) }
    }

    pub fn get_position(&self, TimeUnit(postype): TimeUnit) -> Result<usize, ::Result> {
        let mut t = 0u32;

        match unsafe { ffi::FMOD_Channel_GetPosition(self.channel, &mut t, postype) } {
            ::Result::Ok => Ok(t as usize),
            e => Err(e),
        }
    }

    pub fn set_reverb_properties(&self, prop: &ReverbChannelProperties) -> ::Result {
        let t = ffi::FMOD_REVERB_CHANNELPROPERTIES{Direct: prop.direct, Room: prop.room, Flags: prop.flags, ConnectionPoint: ::std::ptr::null_mut()};

        unsafe { ffi::FMOD_Channel_SetReverbProperties(self.channel, &t) }
    }

    pub fn get_reverb_properties(&self) -> Result<ReverbChannelProperties, ::Result> {
        let mut t = ffi::FMOD_REVERB_CHANNELPROPERTIES{Direct: 0, Room: 0, Flags: 0, ConnectionPoint: ::std::ptr::null_mut()};

        match unsafe { ffi::FMOD_Channel_GetReverbProperties(self.channel, &mut t) } {
            ::Result::Ok => Ok(ReverbChannelProperties{
                direct: t.Direct,
                room: t.Room,
                flags: t.Flags,
                connection_point: ffi::FFI::wrap(t.ConnectionPoint)}),
            e => Err(e),
        }
    }

    pub fn set_low_pass_gain(&self, gain: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetLowPassGain(self.channel, gain) }
    }

    pub fn get_low_pass_gain(&self) -> Result<f32, ::Result> {
        let mut t = 0f32;

        match unsafe { ffi::FMOD_Channel_GetLowPassGain(self.channel, &mut t) } {
            ::Result::Ok => Ok(t),
            e => Err(e),
        }
    }

    pub fn set_channel_group(&mut self, channel_group: &ChannelGroup) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetChannelGroup(self.channel, ffi::FFI::unwrap(channel_group)) }
    }

    pub fn get_channel_group(&self) -> Result<ChannelGroup, ::Result> {
        let mut channel_group = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_Channel_GetChannelGroup(self.channel, &mut channel_group) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(channel_group)),
            e => Err(e)
        }
    }

    pub fn set_3D_attributes(&self, position: &vector::Vector, velocity: &vector::Vector) -> ::Result {
        let mut t_position = vector::get_ffi(position);
        let mut t_velocity = vector::get_ffi(velocity);

        unsafe { ffi::FMOD_Channel_Set3DAttributes(self.channel, &mut t_position, &mut t_velocity) }
    }

    pub fn get_3D_attributes(&self) -> Result<(vector::Vector, vector::Vector), ::Result> {
        let mut position = vector::get_ffi(&vector::Vector::new());
        let mut velocity = vector::get_ffi(&vector::Vector::new());

        match unsafe { ffi::FMOD_Channel_Get3DAttributes(self.channel, &mut position, &mut velocity) } {
            ::Result::Ok => Ok((vector::from_ptr(position), vector::from_ptr(velocity))),
            e => Err(e)
        }
    }

    pub fn set_3D_min_max_distance(&self, min_distance: f32, max_distance: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DMinMaxDistance(self.channel, min_distance, max_distance) }
    }

    pub fn get_3D_min_max_distance(&self) -> Result<(f32, f32), ::Result> {
        let mut min_distance = 0f32;
        let mut max_distance = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DMinMaxDistance(self.channel, &mut min_distance, &mut max_distance) } {
            ::Result::Ok => Ok((min_distance, max_distance)),
            e => Err(e)
        }
    }

    pub fn set_3D_cone_settings(&self, inside_cone_angle: f32, outside_cone_angle: f32, outside_volume: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DConeSettings(self.channel, inside_cone_angle, outside_cone_angle, outside_volume) }
    }

    pub fn get_3D_cone_settings(&self) -> Result<(f32, f32, f32), ::Result> {
        let mut inside_cone_angle = 0f32;
        let mut outside_cone_angle = 0f32;
        let mut outside_volume = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DConeSettings(self.channel, &mut inside_cone_angle, &mut outside_cone_angle, &mut outside_volume) } {
            ::Result::Ok => Ok((inside_cone_angle, outside_cone_angle, outside_volume)),
            e => Err(e)
        }
    }

    pub fn set_3D_cone_orientation(&self, orientation: &vector::Vector) -> ::Result {
        let mut t_orientation = vector::get_ffi(orientation);

        unsafe { ffi::FMOD_Channel_Set3DConeOrientation(self.channel, &mut t_orientation) }
    }

    pub fn get_3D_cone_orientation(&self) -> Result<vector::Vector, ::Result> {
        let mut orientation = vector::get_ffi(&vector::Vector::new());

        match unsafe { ffi::FMOD_Channel_Get3DConeOrientation(self.channel, &mut orientation) } {
            ::Result::Ok => Ok(vector::from_ptr(orientation)),
            e => Err(e)
        }
    }

    pub fn set_3D_custom_rolloff(&self, points: &Vec<vector::Vector>) -> ::Result {
        let mut t_points = Vec::new();

        for tmp in points.iter() {
            t_points.push(vector::get_ffi(tmp));
        }
        unsafe { ffi::FMOD_Channel_Set3DCustomRolloff(self.channel, t_points.as_mut_ptr(), points.len() as c_int) }
    }

    pub fn get_3D_custom_rolloff(&self) -> Result<Vec<vector::Vector>, ::Result> {
        let mut points = ::std::ptr::null_mut();
        let mut num_points = 0i32;

        unsafe {
            match ffi::FMOD_Channel_Get3DCustomRolloff(self.channel, &mut points, &mut num_points) {
               ::Result::Ok => {
                    let mut ret_points = Vec::new();

                    for it in 0i32..num_points {
                        ret_points.push(vector::from_ptr(::std::ptr::read(points.offset(it as isize) as *const ffi::FMOD_VECTOR)));
                    }
                    Ok(ret_points)
                }
                e => Err(e)
            }
        }
    }

    pub fn set_3D_occlusion(&self, direct_occlusion: f32, reverb_occlusion: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DOcclusion(self.channel, direct_occlusion, reverb_occlusion) }
    }

    pub fn get_3D_occlusion(&self) -> Result<(f32, f32), ::Result> {
        let mut direct_occlusion = 0f32;
        let mut reverb_occlusion = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DOcclusion(self.channel, &mut direct_occlusion, &mut reverb_occlusion) } {
            ::Result::Ok => Ok((direct_occlusion, reverb_occlusion)),
            e => Err(e)
        }
    }

    pub fn set_3D_spread(&self, angle: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DSpread(self.channel, angle) }
    }

    pub fn get_3D_spread(&self) -> Result<f32, ::Result> {
        let mut angle = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DSpread(self.channel, &mut angle) } {
            ::Result::Ok => Ok(angle),
            e => Err(e)
        }
    }

    pub fn set_3D_pan_level(&self, level: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DPanLevel(self.channel, level) }
    }

    pub fn get_3D_pan_level(&self) -> Result<f32, ::Result> {
        let mut level = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DPanLevel(self.channel, &mut level) } {
            ::Result::Ok => Ok(level),
            e => Err(e)
        }
    }

    pub fn set_3D_doppler_level(&self, level: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DDopplerLevel(self.channel, level) }
    }

    pub fn get_3D_doppler_level(&self) -> Result<f32, ::Result> {
        let mut level = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DDopplerLevel(self.channel, &mut level) } {
            ::Result::Ok => Ok(level),
            e => Err(e)
        }
    }

    pub fn set_3D_distance_filter(&self, custom: bool, custom_level: f32, center_freq: f32) -> ::Result {
        unsafe { ffi::FMOD_Channel_Set3DDistanceFilter(self.channel, if custom {
                1
            } else {
                0
            }, custom_level, center_freq) }
    }

    pub fn get_3D_distance_filter(&self) -> Result<(bool, f32, f32), ::Result> {
        let mut custom = 0i32;
        let mut custom_level = 0f32;
        let mut center_freq = 0f32;

        match unsafe { ffi::FMOD_Channel_Get3DDistanceFilter(self.channel, &mut custom, &mut custom_level, &mut center_freq) } {
            ::Result::Ok => Ok((custom == 1, custom_level, center_freq)),
            e => Err(e)
        }
    }

    pub fn get_DSP_head(&self) -> Result<Dsp, ::Result> {
        let mut dsp = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_Channel_GetDSPHead(self.channel, &mut dsp) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(dsp)),
            e => Err(e)
        }
    }

    pub fn add_DSP(&self, dsp: &Dsp) -> Result<DspConnection, ::Result> {
        let mut connection = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_Channel_AddDSP(self.channel, ffi::FFI::unwrap(dsp), &mut connection) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(connection)),
            e => Err(e)
        }
    }

    pub fn set_mode(&self, Mode(mode): Mode) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetMode(self.channel, mode) }
    }

    pub fn get_mode(&self) -> Result<Mode, ::Result> {
        let mut mode = 0u32;

        match unsafe { ffi::FMOD_Channel_GetMode(self.channel, &mut mode) } {
            ::Result::Ok => Ok(Mode(mode)),
            e => Err(e)
        }
    }

    pub fn set_loop_count(&self, loop_count: i32) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetLoopCount(self.channel, loop_count) }
    }

    pub fn get_loop_count(&self) -> Result<i32, ::Result> {
        let mut loop_count = 0i32;

        match unsafe { ffi::FMOD_Channel_GetLoopCount(self.channel, &mut loop_count) } {
            ::Result::Ok => Ok(loop_count),
            e => Err(e)
        }
    }

    pub fn set_loop_points(&self, loop_start: u32, TimeUnit(loop_start_type): TimeUnit,
        loop_end: u32, TimeUnit(loop_end_type): TimeUnit) -> ::Result {
            unsafe { ffi::FMOD_Channel_SetLoopPoints(self.channel, loop_start, loop_start_type, loop_end, loop_end_type) }
    }

    pub fn get_loop_points(&self, TimeUnit(loop_start_type): TimeUnit, TimeUnit(loop_end_type): TimeUnit) -> Result<(u32, u32), ::Result> {
        let mut loop_start = 0u32;
        let mut loop_end = 0u32;

        match unsafe { ffi::FMOD_Channel_GetLoopPoints(self.channel, &mut loop_start, loop_start_type, &mut loop_end, loop_end_type) } {
            ::Result::Ok => Ok((loop_start, loop_end)),
            e => Err(e)
        }
    }

    pub fn set_user_data<T>(&self, user_data: &mut T) -> ::Result {
        unsafe { ffi::FMOD_Channel_SetUserData(self.channel, transmute(user_data)) }
    }

    pub fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, ::Result> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::null_mut();

            match ffi::FMOD_Channel_GetUserData(self.channel, &mut user_data) {
               ::Result::Ok => {
                    let tmp : &mut T = transmute::<*mut c_void, &mut T>(user_data);
                    
                    Ok(tmp)
                },
                e => Err(e)
            }
        }
    }

    pub fn get_memory_info(&self, MemoryBits(memory_bits): MemoryBits,
        EventMemoryBits(event_memory_bits): EventMemoryBits) -> Result<(u32, MemoryUsageDetails), ::Result> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_Channel_GetMemoryInfo(self.channel, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            ::Result::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }
}