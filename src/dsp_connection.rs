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
use enums::*;
use dsp;
use libc::c_int;
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;

pub fn from_ptr(dsp_connection: ffi::FMOD_DSPCONNECTION) -> DspConnection {
    DspConnection{dsp_connection: dsp_connection}
}

pub fn get_ffi(dsp_connection: DspConnection) -> ffi::FMOD_DSPCONNECTION {
    dsp_connection.dsp_connection
}

pub struct DspConnection {
    dsp_connection: ffi::FMOD_DSPCONNECTION
}

impl Drop for DspConnection {
    fn drop(&mut self) {
        self.release();
    }
}

impl DspConnection {
    pub fn release(&mut self) {
        self.dsp_connection = ::std::ptr::null();
    }

    pub fn get_input(&self) -> Result<dsp::Dsp, fmod::Result> {
        let input = ::std::ptr::null();

        match unsafe { ffi::FMOD_DSPConnection_GetInput(self.dsp_connection, &input) } {
            fmod::Ok => Ok(dsp::from_ptr(input)),
            e => Err(e)
        }
    }

    pub fn get_output(&self) -> Result<dsp::Dsp, fmod::Result> {
        let output = ::std::ptr::null();

        match unsafe { ffi::FMOD_DSPConnection_GetOutput(self.dsp_connection, &output) } {
            fmod::Ok => Ok(dsp::from_ptr(output)),
            e => Err(e)
        }
    }

    pub fn set_mix(&self, volume: f32) -> fmod::Result {
        unsafe { ffi::FMOD_DSPConnection_SetMix(self.dsp_connection, volume) }
    }

    pub fn get_mix(&self) -> Result<f32, fmod::Result> {
        let volume = 0f32;

        match unsafe { ffi::FMOD_DSPConnection_GetMix(self.dsp_connection, &volume) } {
            fmod::Ok => Ok(volume),
            e => Err(e)
        }
    }

    pub fn set_levels(&self, speaker: fmod::Speaker, levels: Vec<f32>) -> fmod::Result {
        unsafe { ffi::FMOD_DSPConnection_SetLevels(self.dsp_connection, speaker, levels.as_ptr(), levels.len() as c_int) }
    }

    pub fn get_levels(&self, speaker: fmod::Speaker, num_levels: uint) -> Result<Vec<f32>, fmod::Result> {
        let levels = Vec::from_elem(num_levels, 0f32);

        match unsafe { ffi::FMOD_DSPConnection_GetLevels(self.dsp_connection, speaker, levels.as_ptr(), levels.len() as c_int) } {
            fmod::Ok => Ok(levels),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let details = fmod_sys::get_memory_usage_details_ffi(FmodMemoryUsageDetails::new());
        let memory_used = 0u32;

        match unsafe { ffi::FMOD_DSPCONNECTION_GetMemoryInfo(self.dsp_connection, memory_bits, event_memory_bits, &memory_used, &details) } {
            fmod::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }
}