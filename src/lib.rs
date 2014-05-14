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

#![crate_id = "github.com/GuillaumeGomez/rust-fmod#rfmod:0.1"]
#![desc = "Rust binding for FMOD"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(uppercase_variables)]

#![feature(globs)]

extern crate libc;

use libc::{c_void, c_uint, c_int, c_char, c_float};
use enums::*;
pub use channel::{Channel, SpectrumOptions, DelayOptions, SpeakerMixOptions, ReverbChannelProperties};
pub use sound::Sound;

mod ffi;
mod sound;
mod channel;
pub mod enums;

#[link(name = "fmodex64")] extern{}

pub struct Fmod {
	system : ffi::FMOD_SYSTEM,
}

impl Fmod {
	pub fn new() -> Result<Fmod, FMOD_RESULT> {
		let tmp = ::std::ptr::null();

		match unsafe { ffi::FMOD_System_Create(&tmp) } {
			FMOD_OK => {
				match unsafe { ffi::FMOD_System_Init(tmp, 1, FMOD_INIT_NORMAL, ::std::ptr::null()) } {
					FMOD_OK => Ok(Fmod{system : tmp}),
					err => {
						unsafe { ffi::FMOD_System_Release(tmp) };
						Err(err)
					}
				}
			},
			err => Err(err),
		}
	}

	pub fn release(&self) -> FMOD_RESULT {
		unsafe {
			ffi::FMOD_System_Close(self.system);
			ffi::FMOD_System_Release(self.system)
		}
	}

	pub fn create_sound(&self, music : StrBuf) -> Result<Sound, FMOD_RESULT> {
	   	let tmp_v = music.clone().into_owned();
	   	let sound = ::std::ptr::null();

	    tmp_v.with_c_str(|c_str|{
	        match unsafe { ffi::FMOD_System_CreateSound(self.system, c_str,
	           								FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM, ::std::ptr::null(), &sound) } {
	           	FMOD_OK => {Ok(sound::new(self.system, music.clone(), sound))},
	           	err => Err(err),
	        }
	    })
	}	
}