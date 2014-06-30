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
use dsp_connection;
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;
use std::mem::transmute;
use libc::{c_char};

pub fn from_ptr(dsp: ffi::FMOD_DSP) -> Dsp {
    Dsp{dsp: dsp, can_be_deleted: false}
}

pub fn from_ptr_first(dsp: ffi::FMOD_DSP) -> Dsp {
    Dsp{dsp: dsp, can_be_deleted: true}
}

pub fn get_ffi(dsp: &Dsp) -> ffi::FMOD_DSP {
    dsp.dsp
}

pub struct Dsp {
    dsp: ffi::FMOD_DSP,
    can_be_deleted: bool
}

impl Drop for Dsp {
    fn drop(&mut self) {
        self.release();
    }
}

impl Dsp {
    pub fn release(&mut self) -> fmod::Result {
        if self.can_be_deleted && self.dsp.is_not_null() {
            match unsafe { ffi::FMOD_DSP_Release(self.dsp) } {
                fmod::Ok => {
                    self.dsp =::std::ptr::mut_null();
                    fmod::Ok
                }
                e => e
            }
        } else {
            fmod::Ok
        }
    }

    pub fn add_input(&self, target: Dsp) -> Result<dsp_connection::DspConnection, fmod::Result> {
        let mut connection = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_DSP_AddInput(self.dsp, target.dsp, &mut connection) } {
            fmod::Ok => Ok(dsp_connection::from_ptr(connection)),
            e => Err(e)
        }
    }

    pub fn disconnect_from(&self, target: Dsp) -> fmod::Result {
        unsafe { ffi::FMOD_DSP_DisconnectFrom(self.dsp, target.dsp) }
    }

    pub fn disconnect_all(&self, inputs: bool, outputs: bool) -> fmod::Result {
        let t_inputs = if inputs == true {
            1
        } else {
            0
        };
        let t_outputs = if outputs == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_DSP_DisconnectAll(self.dsp, t_inputs, t_outputs) }
    }

    pub fn remove(&self) -> fmod::Result {
        unsafe { ffi::FMOD_DSP_Remove(self.dsp) }
    }

    pub fn get_num_inputs(&self) -> Result<i32, fmod::Result> {
        let mut inputs = 0i32;

        match unsafe { ffi::FMOD_DSP_GetNumInputs(self.dsp, &mut inputs) } {
            fmod::Ok => Ok(inputs),
            e => Err(e)
        }
    }

    pub fn get_num_outputs(&self) -> Result<i32, fmod::Result> {
        let mut outputs = 0i32;

        match unsafe { ffi::FMOD_DSP_GetNumOutputs(self.dsp, &mut outputs) } {
            fmod::Ok => Ok(outputs),
            e => Err(e)
        }
    }

    pub fn get_input(&self, index: i32) -> Result<(Dsp, dsp_connection::DspConnection), fmod::Result> {
        let mut input = ::std::ptr::mut_null();
        let mut input_connection = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_DSP_GetInput(self.dsp, index, &mut input, &mut input_connection) } {
            fmod::Ok => Ok((from_ptr(input), dsp_connection::from_ptr(input_connection))),
            e => Err(e)
        }
    }

    pub fn get_output(&self, index: i32) -> Result<(Dsp, dsp_connection::DspConnection), fmod::Result> {
        let mut output = ::std::ptr::mut_null();
        let mut output_connection = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_DSP_GetOutput(self.dsp, index, &mut output, &mut output_connection) } {
            fmod::Ok => Ok((from_ptr(output), dsp_connection::from_ptr(output_connection ))),
            e => Err(e)
        }
    }

    pub fn set_active(&self, active: bool) -> fmod::Result {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_DSP_SetActive(self.dsp, t_active) }
    }

    pub fn is_active(&self) -> Result<bool, fmod::Result> {
        let mut active = 0i32;

        match unsafe { ffi::FMOD_DSP_GetActive(self.dsp, &mut active) } {
            fmod::Ok => Ok(active == 1i32),
            e => Err(e)
        }
    }

    pub fn set_bypass(&self, bypass: bool) -> fmod::Result {
        let t_bypass = if bypass == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_DSP_SetBypass(self.dsp, t_bypass) }
    }

    pub fn get_bypass(&self) -> Result<bool, fmod::Result> {
        let mut bypass = 0i32;

        match unsafe { ffi::FMOD_DSP_GetBypass(self.dsp, &mut bypass) } {
            fmod::Ok => Ok(bypass == 1i32),
            e => Err(e)
        }
    }

    pub fn set_speaker_active(&self, speaker: fmod::Speaker, active: bool) -> fmod::Result {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_DSP_SetSpeakerActive(self.dsp, speaker, t_active) }
    }

    pub fn get_speaker_active(&self, speaker: fmod::Speaker) -> Result<bool, fmod::Result> {
        let mut active = 0i32;

        match unsafe { ffi::FMOD_DSP_GetSpeakerActive(self.dsp, speaker, &mut active) } {
            fmod::Ok => Ok(active == 1i32),
            e => Err(e)
        }
    }

    pub fn reset(&self) -> fmod::Result {
        unsafe { ffi::FMOD_DSP_Reset(self.dsp) }
    }

    pub fn set_parameter(&self, index: i32, value: f32) -> fmod::Result {
        unsafe { ffi::FMOD_DSP_SetParameter(self.dsp, index, value) }
    }

    pub fn get_parameter(&self, index: i32, value_str_len: u32) -> Result<(f32, String), fmod::Result> {
        let tmp_v = String::with_capacity(value_str_len as uint);
        let mut value = 0f32;

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_DSP_GetParameter(self.dsp, index, &mut value, c_str as *mut c_char, value_str_len as i32) } {
                fmod::Ok => Ok((value, unsafe {::std::str::raw::from_c_str(c_str).clone() })),
                e => Err(e)
            }
        })
    }

    pub fn get_num_parameters(&self) -> Result<i32, fmod::Result> {
        let mut num_param = 0i32;

        match unsafe { ffi::FMOD_DSP_GetNumParameters(self.dsp, &mut num_param) } {
            fmod::Ok => Ok(num_param),
            e => Err(e)
        }
    }

    pub fn get_parameter_info(&self, index: i32, name: &String, label: &String, description_len: u32) -> Result<(String, f32, f32), fmod::Result> {
        let mut min = 0f32;
        let mut max = 0f32;
        let tmp_d = String::with_capacity(description_len as uint);
        let t_name = name.clone();
        let t_label = label.clone();

        tmp_d.with_c_str(|c_description| {
            t_name.with_c_str(|c_name| {
                t_label.with_c_str(|c_label|{
                    match unsafe { ffi::FMOD_DSP_GetParameterInfo(self.dsp, index, c_name as *mut c_char, c_label as *mut c_char,
                        c_description as *mut c_char, description_len as i32, &mut min, &mut max) } {
                        fmod::Ok => Ok((unsafe {::std::str::raw::from_c_str(c_description).clone() }, min, max)),
                        e => Err(e)
                    }
                })
            })
        })
    }

    pub fn get_info(&self, name: &String) -> Result<(u32, i32, i32, i32), fmod::Result> {
        let mut version = 0u32;
        let mut channels = 0i32;
        let mut config_width = 0i32;
        let mut config_height = 0i32;
        let tmp_n = name.clone();

        tmp_n.with_c_str(|c_name| {
            match unsafe { ffi::FMOD_DSP_GetInfo(self.dsp, c_name as *mut c_char, &mut version, &mut channels, &mut config_width, &mut config_height) } {
                fmod::Ok => Ok((version, channels, config_width, config_height)),
                e => Err(e)
            }
        })
    }

    pub fn set_defaults(&self, frequency: f32, volume: f32, pan: f32, priority: i32) -> fmod::Result {
        unsafe { ffi::FMOD_DSP_SetDefaults(self.dsp, frequency, volume, pan, priority) }
    }

    pub fn get_defaults(&self) -> Result<(f32, f32, f32, i32), fmod::Result> {
        let mut frequency = 0f32;
        let mut volume = 0f32;
        let mut pan = 0f32;
        let mut priority = 0i32;

        match unsafe { ffi::FMOD_DSP_GetDefaults(self.dsp, &mut frequency, &mut volume, &mut pan, &mut priority) } {
            fmod::Ok => Ok((frequency, volume, pan, priority)),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(FmodMemoryUsageDetails::new());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_DSP_GetMemoryInfo(self.dsp, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            fmod::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    /* to test ! */
    /*pub fn set_user_data<T>(&self, user_data: T) -> fmod::Result {
        unsafe { ffi::FMOD_DSP_SetUserData(self.dsp, transmute(user_data)) }
    }*/

    /* to test ! */
    /*pub fn get_user_data<T>(&self) -> Result<T, fmod::Result> {
        unsafe {
            let user_data =::std::ptr::null();

            match ffi::FMOD_DSP_GetUserData(self.dsp, &user_data) {
                fmod::Ok => Ok(transmute(user_data)),
                e => Err(e)
            }
        }
    }*/
}