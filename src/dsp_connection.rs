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

use ffi;
use types::*;
use dsp;
use libc::{c_int, c_void};
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;
use std::mem::transmute;
use std::default::Default;

/// DspConnection object
pub struct DspConnection {
    dsp_connection: *mut ffi::FMOD_DSPCONNECTION
}

impl ffi::FFI<ffi::FMOD_DSPCONNECTION> for DspConnection {
    fn wrap(d: *mut ffi::FMOD_DSPCONNECTION) -> DspConnection {
        DspConnection {dsp_connection: d}
    }

    fn unwrap(d: &DspConnection) -> *mut ffi::FMOD_DSPCONNECTION {
        d.dsp_connection
    }
}

impl Drop for DspConnection {
    fn drop(&mut self) {
        self.release();
    }
}

impl DspConnection {
    pub fn release(&mut self) {
        self.dsp_connection = ::std::ptr::null_mut();
    }

    pub fn get_input(&self) -> Result<dsp::Dsp, ::Result> {
        let mut input = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_DSPConnection_GetInput(self.dsp_connection, &mut input) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(input)),
            e => Err(e)
        }
    }

    pub fn get_output(&self) -> Result<dsp::Dsp, ::Result> {
        let mut output = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_DSPConnection_GetOutput(self.dsp_connection, &mut output) } {
            ::Result::Ok => Ok(ffi::FFI::wrap(output)),
            e => Err(e)
        }
    }

    pub fn set_mix(&self, volume: f32) -> ::Result {
        unsafe { ffi::FMOD_DSPConnection_SetMix(self.dsp_connection, volume) }
    }

    pub fn get_mix(&self) -> Result<f32, ::Result> {
        let mut volume = 0f32;

        match unsafe { ffi::FMOD_DSPConnection_GetMix(self.dsp_connection, &mut volume) } {
            ::Result::Ok => Ok(volume),
            e => Err(e)
        }
    }

    pub fn set_levels(&self, speaker: ::Speaker, levels: &mut Vec<f32>) -> ::Result {
        unsafe { ffi::FMOD_DSPConnection_SetLevels(self.dsp_connection, speaker, levels.as_mut_ptr(), levels.len() as c_int) }
    }

    pub fn get_levels(&self, speaker: ::Speaker, num_levels: uint) -> Result<Vec<f32>, ::Result> {
        let mut levels = Vec::from_elem(num_levels, 0f32);

        match unsafe { ffi::FMOD_DSPConnection_GetLevels(self.dsp_connection, speaker, levels.as_mut_ptr(), levels.len() as c_int) } {
            ::Result::Ok => Ok(levels),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), ::Result> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_DSPConnection_GetMemoryInfo(self.dsp_connection, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            ::Result::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    pub fn set_user_data<T>(&self, user_data: &mut T) -> ::Result {
        unsafe { ffi::FMOD_DSPConnection_SetUserData(self.dsp_connection, transmute(user_data)) }
    }

    pub fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, ::Result> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::null_mut();

            match ffi::FMOD_DSPConnection_GetUserData(self.dsp_connection, &mut user_data) {
               ::Result::Ok => {
                    let tmp : &mut T = transmute::<*mut c_void, &mut T>(user_data);
                    
                    Ok(tmp)
                },
                e => Err(e)
            }
        }
    }
}