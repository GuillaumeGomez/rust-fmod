/*
* Rust-FMOD - Copyright (c) 2016 Gomez Guillaume.
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
use ffi;
use sound;
use libc::c_void;
use fmod_sys;
use fmod_sys::MemoryUsageDetails;
use std::mem::transmute;
use libc::{c_char};
use std::default::Default;

/// SoundGroup object
pub struct SoundGroup {
    sound_group: *mut ffi::FMOD_SOUNDGROUP,
}

impl ffi::FFI<ffi::FMOD_SOUNDGROUP> for SoundGroup {
    fn wrap(s: *mut ffi::FMOD_SOUNDGROUP) -> SoundGroup {
        SoundGroup {sound_group: s}
    }

    fn unwrap(s: &SoundGroup) -> *mut ffi::FMOD_SOUNDGROUP {
        s.sound_group
    }
}

impl Drop for SoundGroup {
    fn drop(&mut self) {
        self.release();
    }
}

impl SoundGroup {
    pub fn release(&mut self) -> ::Status {
        if !self.sound_group.is_null() {
            match unsafe { ffi::FMOD_SoundGroup_Release(self.sound_group) } {
               ::Status::Ok => {
                    self.sound_group =::std::ptr::null_mut();
                   ::Status::Ok
                }
                e => e
            }
        } else {
           ::Status::Ok
        }
    }

    pub fn set_max_audible(&self, max_audible: i32) -> ::Status {
        unsafe { ffi::FMOD_SoundGroup_SetMaxAudible(self.sound_group, max_audible) }
    }

    pub fn get_max_audible(&self) -> Result<i32, ::Status> {
        let mut max_audible = 0i32;

        match unsafe { ffi::FMOD_SoundGroup_GetMaxAudible(self.sound_group, &mut max_audible) } {
            ::Status::Ok => Ok(max_audible),
            e => Err(e)
        }
    }

    pub fn set_max_audible_behavior(&self, max_audible_behavior: ::SoundGroupBehavior) -> ::Status {
        unsafe { ffi::FMOD_SoundGroup_SetMaxAudibleBehavior(self.sound_group,
                                                            max_audible_behavior) }
    }

    pub fn get_max_audible_behavior(&self) -> Result<::SoundGroupBehavior, ::Status> {
        let mut max_audible_behavior = ::SoundGroupBehavior::Fail;

        match unsafe { ffi::FMOD_SoundGroup_GetMaxAudibleBehavior(self.sound_group,
                                                                  &mut max_audible_behavior) } {
            ::Status::Ok => Ok(max_audible_behavior),
            e => Err(e)
        }
    }

    pub fn set_mute_fade_speed(&self, speed: f32) -> ::Status {
        unsafe { ffi::FMOD_SoundGroup_SetMuteFadeSpeed(self.sound_group, speed) }
    }

    pub fn get_mute_fade_speed(&self) -> Result<f32, ::Status> {
        let mut speed = 0f32;

        match unsafe { ffi::FMOD_SoundGroup_GetMuteFadeSpeed(self.sound_group, &mut speed) } {
            ::Status::Ok => Ok(speed),
            e => Err(e)
        }
    }

    pub fn set_volume(&self, volume: f32) -> ::Status {
        unsafe { ffi::FMOD_SoundGroup_SetVolume(self.sound_group, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, ::Status> {
        let mut volume = 0f32;

        match unsafe { ffi::FMOD_SoundGroup_GetVolume(self.sound_group, &mut volume) } {
            ::Status::Ok => Ok(volume),
            e => Err(e)
        }
    }

    pub fn stop(&self) -> ::Status {
        unsafe { ffi::FMOD_SoundGroup_Stop(self.sound_group) }
    }

    pub fn get_name(&self, name_len: usize) -> Result<String, ::RStatus> {
        let mut c = Vec::with_capacity(name_len + 1);

        for _ in 0..(name_len + 1) {
            c.push(0);
        }

        match unsafe { ffi::FMOD_SoundGroup_GetName(self.sound_group, c.as_mut_ptr() as *mut c_char,
                                                    name_len as i32) } {
            ::Status::Ok => Ok(from_utf8!(c)),
            e => Err(::RStatus::FMOD(e)),
        }
    }

    pub fn get_num_sounds(&self) -> Result<i32, ::Status> {
        let mut num_sounds = 0i32;

        match unsafe { ffi::FMOD_SoundGroup_GetNumSounds(self.sound_group, &mut num_sounds) } {
            ::Status::Ok => Ok(num_sounds),
            e => Err(e)
        }
    }

    pub fn get_sound(&self, index: i32) -> Result<sound::Sound, ::Status> {
        let mut sound = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_SoundGroup_GetSound(self.sound_group, index, &mut sound) } {
            ::Status::Ok => Ok(ffi::FFI::wrap(sound)),
            e => Err(e)
        }
    }

    pub fn get_num_playing(&self) -> Result<i32, ::Status> {
        let mut num_playing = 0i32;

        match unsafe { ffi::FMOD_SoundGroup_GetNumPlaying(self.sound_group, &mut num_playing) } {
            ::Status::Ok => Ok(num_playing),
            e => Err(e)
        }
    }

    /// Returns:
    ///
    /// Ok(memory_used, details)
    pub fn get_memory_info(&self, MemoryBits(memory_bits): MemoryBits,
                           EventMemoryBits(event_memory_bits): EventMemoryBits)
                           -> Result<(u32, MemoryUsageDetails), ::Status> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_SoundGroup_GetMemoryInfo(self.sound_group, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            ::Status::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    pub fn set_user_data<'r, T>(&'r self, user_data: &'r mut T) -> ::Status {
        unsafe { ffi::FMOD_SoundGroup_SetUserData(self.sound_group, transmute(user_data)) }
    }

    pub fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, ::Status> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::null_mut();

            match ffi::FMOD_SoundGroup_GetUserData(self.sound_group, &mut user_data) {
               ::Status::Ok => {
                    let tmp : &mut T = transmute::<*mut c_void, &mut T>(user_data);
                    
                    Ok(tmp)
                },
                e => Err(e)
            }
        }
    }
}
