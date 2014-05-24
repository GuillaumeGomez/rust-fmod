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
use channel_group;
use channel_group::ChannelGroup;

pub struct FmodSpectrumOptions {
    pub window_type     : fmod::DSP_FFT_Window,
    pub channel_offset  : i32
}

pub struct FmodDelayOptions {
    pub delaytype   : fmod::DelayType,
    pub delayhi     : uint,
    pub delaylo     : uint
}

pub struct FmodSpeakerMixOptions {
    pub front_left  : f32,
    pub front_right : f32,
    pub center      : f32,
    pub lfe         : f32,
    pub back_left   : f32,
    pub back_right  : f32,
    pub side_left   : f32,
    pub side_right  : f32
}

pub struct FmodReverbChannelProperties {
    pub direct           : c_int,
    pub room             : c_int,
    pub flags            : c_uint,
    pub connection_point : dsp::Dsp
}

pub fn get_ffi(channel : &Channel) -> *ffi::FMOD_CHANNEL {
    &channel.channel as *ffi::FMOD_CHANNEL
}

pub fn new() -> Channel {
    Channel{channel : ::std::ptr::null()}
}

pub fn from_ptr(channel: ffi::FMOD_CHANNEL) -> Channel {
    Channel{channel: channel}
}

pub struct Channel {
    channel : ffi::FMOD_CHANNEL
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.release();
    }
}

impl Channel {
    pub fn release(&mut self) {
        self.channel = ::std::ptr::null();
    }

    pub fn get_spectrum(&self, spectrum_size : uint, options : Option<FmodSpectrumOptions>) -> Result<Vec<f32>, fmod::Result> {
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
        match unsafe { ffi::FMOD_Channel_GetSpectrum(self.channel, ptr.as_ptr(), spectrum_size as c_int, channel_offset, window_type) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size : uint, channel_offset : i32) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(wave_size, 0f32);

        match unsafe { ffi::FMOD_Channel_GetWaveData(self.channel, ptr.as_ptr(), wave_size as c_int, channel_offset) } {
            fmod::Ok => Ok(ptr),
            e => Err(e)
        }
    }

    pub fn is_init(&self) -> bool {
        self.channel != ::std::ptr::null()
    }

    pub fn is_playing(&self) -> Result<bool, fmod::Result> {
        let is_playing = 0;

        match unsafe { ffi::FMOD_Channel_IsPlaying(self.channel, &is_playing) } {
            fmod::Ok => Ok(is_playing == 1),
            err => Err(err),
        }
    }

    pub fn set_volume(&self, volume : f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetVolume(self.channel, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, fmod::Result> {
        let volume = 0f32;

        match unsafe { ffi::FMOD_Channel_GetVolume(self.channel, &volume) } {
            fmod::Ok => Ok(volume),
            e => Err(e),
        }
    }

    pub fn set_frequency(&self, frequency : f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetFrequency(self.channel, frequency) }
    }

    pub fn get_frequency(&self) -> Result<f32, fmod::Result> {
        let frequency = 0f32;

        match unsafe { ffi::FMOD_Channel_GetFrequency(self.channel, &frequency) } {
            fmod::Ok => Ok(frequency),
            e => Err(e),
        }
    }

    pub fn set_pan(&self, pan : f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetPan(self.channel, pan) }
    }

    pub fn get_pan(&self) -> Result<f32, fmod::Result> {
        let pan = 0f32;

        match unsafe { ffi::FMOD_Channel_GetPan(self.channel, &pan) } {
            fmod::Ok => Ok(pan),
            e => Err(e),
        }
    }

    pub fn set_mute(&self, mute : bool) -> fmod::Result {
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

    pub fn set_paused(&self, paused : bool) -> fmod::Result {
        let t : ffi::FMOD_BOOL = match paused {
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

    pub fn set_delay(&self, d_o : FmodDelayOptions) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetDelay(self.channel, d_o.delaytype, d_o.delayhi as u32, d_o.delaylo as u32) }
    }

    pub fn get_delay(&self, delaytype : fmod::DelayType) -> Result<FmodDelayOptions, fmod::Result> {
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

    pub fn set_speaker_mix(&self, smo : FmodSpeakerMixOptions) -> fmod::Result {
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

    pub fn set_speaker_level(&self, speaker : fmod::Speaker, levels : Vec<f32>) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetSpeakerLevels(self.channel, speaker, levels.as_ptr(), levels.len() as i32) }
    }

    pub fn get_speaker_level(&self, speaker : fmod::Speaker, num_levels : uint) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(num_levels, 0f32);

        match unsafe { ffi::FMOD_Channel_GetSpeakerLevels(self.channel, speaker, ptr.as_ptr(), num_levels as i32) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn set_input_channel_mix(&self, levels : Vec<f32>) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetInputChannelMix(self.channel, levels.as_ptr(), levels.len() as i32) }
    }

    pub fn get_input_channel_mix(&self, num_levels : uint) -> Result<Vec<f32>, fmod::Result> {
        let ptr = Vec::from_elem(num_levels, 0f32);

        match unsafe { ffi::FMOD_Channel_GetInputChannelMix(self.channel, ptr.as_ptr(), num_levels as i32) } {
            fmod::Ok => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn set_priority(&self, priority : i32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetPriority(self.channel, priority) }
    }

    pub fn get_priority(&self) -> Result<i32, fmod::Result> {
        let t = 0i32;

        match unsafe { ffi::FMOD_Channel_GetPriority(self.channel, &t) } {
            fmod::Ok => Ok(t),
            e => Err(e),
        }
    }

    pub fn set_position(&self, position : uint, FmodTimeUnit(postype) : FmodTimeUnit) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetPosition(self.channel, position as u32, postype) }
    }

    pub fn get_position(&self, FmodTimeUnit(postype) : FmodTimeUnit) -> Result<uint, fmod::Result> {
        let t = 0u32;

        match unsafe { ffi::FMOD_Channel_GetPosition(self.channel, &t, postype) } {
            fmod::Ok => Ok(t as uint),
            e => Err(e),
        }
    }

    pub fn set_reverb_properties(&self, prop : FmodReverbChannelProperties) -> fmod::Result {
        let t = ffi::FMOD_REVERB_CHANNELPROPERTIES{Direct: prop.direct, Room: prop.room, Flags: prop.flags, ConnectionPoint: ::std::ptr::null()};

        unsafe { ffi::FMOD_Channel_SetReverbProperties(self.channel, &t) }
    }

    pub fn get_reverb_properties(&self) -> Result<FmodReverbChannelProperties, fmod::Result> {
        let t = ffi::FMOD_REVERB_CHANNELPROPERTIES{Direct : 0, Room : 0, Flags : 0, ConnectionPoint : ::std::ptr::null()};

        match unsafe { ffi::FMOD_Channel_GetReverbProperties(self.channel, &t) } {
            fmod::Ok => Ok(FmodReverbChannelProperties{
                direct: t.Direct,
                room: t.Room,
                flags: t.Flags,
                connection_point: dsp::from_ptr(t.ConnectionPoint)}),
            e => Err(e),
        }
    }

    pub fn set_low_pass_gain(&self, gain : f32) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetLowPassGain(self.channel, gain) }
    }

    pub fn get_low_pass_gain(&self) -> Result<f32, fmod::Result> {
        let t = 0f32;

        match unsafe { ffi::FMOD_Channel_GetLowPassGain(self.channel, &t) } {
            fmod::Ok => Ok(t),
            e => Err(e),
        }
    }

    pub fn set_channel_group(&mut self, channel_group : ChannelGroup) -> fmod::Result {
        unsafe { ffi::FMOD_Channel_SetChannelGroup(self.channel, channel_group::get_ffi(channel_group)) }
    }

    pub fn get_channel_group(&self) -> Result<ChannelGroup, fmod::Result> {
        let channel_group = ::std::ptr::null();

        match unsafe { ffi::FMOD_Channel_GetChannelGroup(self.channel, &channel_group) } {
            fmod::Ok => Ok(channel_group::from_ptr(channel_group)),
            e => Err(e)
        }
    }
}