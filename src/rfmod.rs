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

#![crate_id = "github.com/GuillaumeGomez/rust-fmod#rfmod:0.1"]
#![desc = "Rust binding for FMOD"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(uppercase_variables)]

#![feature(globs)]

extern crate libc;

pub use channel::{Channel, FmodSpectrumOptions, FmodDelayOptions, FmodSpeakerMixOptions, FmodReverbChannelProperties};
pub use sound::{Sound, FmodTag, FmodSyncPoint};
pub use fmod_sys::{FmodSys, FmodGuid, FmodSoftwareFormat, FmodAdvancedSettings, FmodOutputHandle, FmodCreateSoundexInfo, FmodMemoryUsageDetails};
pub use channel_group::ChannelGroup;
pub use sound_group::SoundGroup;
pub use dsp::{Dsp, DspParameterDesc, DspDescription, DspState};
pub use dsp_connection::DspConnection;
pub use reverb::Reverb;
pub use reverb_properties::ReverbProperties;
pub use vector::FmodVector;
pub use geometry::Geometry;

mod ffi;
mod sound;
mod channel;
mod channel_group;
mod sound_group;
mod fmod_sys;
mod dsp;
mod dsp_connection;
mod geometry;
mod vector;
mod reverb;
mod reverb_properties;
pub mod types;
pub mod enums;
pub mod callbacks;

#[cfg(target_arch="x86")]
#[link(name = "fmodex")] extern{}
#[cfg(target_arch="x86_64")]
#[link(name = "fmodex64")] extern{}