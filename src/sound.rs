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
use libc::c_int;
use ffi;
use channel;
use channel::Channel;
use std::io::timer::sleep;

pub struct Sound {
    sound       : ffi::FMOD_SOUND,
    pub name    : StrBuf,
    system      : ffi::FMOD_SYSTEM,
    channel     : Channel,
}

pub fn new(system : ffi::FMOD_SYSTEM, name : StrBuf, sound: ffi::FMOD_SOUND) -> Sound {
        Sound{sound: sound, channel: channel::new(), name: name.clone(), system: system}
}

impl Sound {
    pub fn release(&mut self) -> FMOD_RESULT {
        self.system = ::std::ptr::null();
        self.channel.release();
        unsafe { ffi::FMOD_Sound_Release(self.sound) }
    }

    pub fn play(&self) -> FMOD_RESULT {
        if unsafe { *channel::get_ffi(&self.channel) == ::std::ptr::null() } {
            unsafe { ffi::FMOD_System_PlaySound(self.system, FMOD_CHANNEL_FREE, self.sound, 0, channel::get_ffi(&self.channel)) }
        } else {
            self.channel.set_paused(false)
        }
    }

    pub fn pause(&self) -> FMOD_RESULT {
        self.channel.set_paused(true)
    }

    pub fn is_playing(&self) -> Result<bool, FMOD_RESULT> {
        self.channel.is_playing()
    }

    pub fn play_to_the_end(&self) -> FMOD_RESULT {
        match self.play() {
            FMOD_OK => {
                loop {
                    match self.is_playing() {
                        Ok(b) => {
                            if b == true {
                                sleep(30)
                            } else {
                                break;
                            }
                        },
                        Err(e) => return e,
                    }
                }
                FMOD_OK
            }
            err => err,
        }
    }

    pub fn get_channel<'a>(&'a self) -> &'a Channel {
        &self.channel
    }
}