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
use vector;
use reverb_properties;
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;
use std::mem::transmute;
use libc::{c_void};

pub fn from_ptr(reverb: *mut ffi::FMOD_REVERB) -> Reverb {
    Reverb{reverb: reverb}
}

pub fn get_ffi(reverb: Reverb) -> *mut ffi::FMOD_REVERB {
    reverb.reverb
}

/// Reverb object
pub struct Reverb {
    reverb: *mut ffi::FMOD_REVERB
}

impl Drop for Reverb {
    fn drop(&mut self) {
        self.release();
    }
}

impl Reverb {
    pub fn release(&mut self) -> fmod::Result {
        if self.reverb !=::std::ptr::mut_null() {
            match unsafe { ffi::FMOD_Reverb_Release(self.reverb) } {
                fmod::Ok => {
                    self.reverb = ::std::ptr::mut_null();
                    fmod::Ok
                }
                e => e
            }
        } else {
            fmod::Ok
        }
    }

    pub fn set_3D_attributes(&self, position: vector::FmodVector, min_distance: f32, max_distance: f32) -> fmod::Result {
        let t_position = vector::get_ffi(&position);

        unsafe { ffi::FMOD_Reverb_Set3DAttributes(self.reverb, &t_position, min_distance, max_distance) }
    }

    pub fn get_3D_attributes(&self) -> Result<(vector::FmodVector, f32, f32), fmod::Result> {
        let mut position = vector::get_ffi(&vector::new());
        let mut min_distance = 0f32;
        let mut max_distance = 0f32;

        match unsafe { ffi::FMOD_Reverb_Get3DAttributes(self.reverb, &mut position, &mut min_distance, &mut max_distance) } {
            fmod::Ok => Ok((vector::from_ptr(position), min_distance, max_distance)),
            e => Err(e)
        }
    }

    pub fn set_properties(&self, reverb_properties: reverb_properties::ReverbProperties) -> fmod::Result {
        let t_reverb_properties = reverb_properties::get_ffi(reverb_properties);

        unsafe { ffi::FMOD_Reverb_SetProperties(self.reverb, &t_reverb_properties) }
    }

    pub fn get_properties(&self, reverb_properties: reverb_properties::ReverbProperties) -> Result<reverb_properties::ReverbProperties, fmod::Result> {
        let mut t_reverb_properties = reverb_properties::get_ffi(reverb_properties);

        match unsafe { ffi::FMOD_Reverb_GetProperties(self.reverb, &mut t_reverb_properties) } {
            fmod::Ok => Ok(reverb_properties::from_ptr(t_reverb_properties)),
            e => Err(e)
        }
    }

    pub fn set_active(&self, active: bool) -> fmod::Result {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_Reverb_SetActive(self.reverb, t_active) }
    }

    pub fn get_active(&self) -> Result<bool, fmod::Result> {
        let mut active = 0i32;

        match unsafe { ffi::FMOD_Reverb_GetActive(self.reverb, &mut active) } {
            fmod::Ok => Ok(active == 1),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(FmodMemoryUsageDetails::new());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_Reverb_GetMemoryInfo(self.reverb, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            fmod::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    pub fn set_user_data<T>(&self, user_data: &mut T) -> fmod::Result {
        unsafe { ffi::FMOD_Reverb_SetUserData(self.reverb, transmute(user_data)) }
    }

    fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, fmod::Result> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::mut_null();

            match ffi::FMOD_Reverb_GetUserData(self.reverb, &mut user_data) {
                fmod::Ok => {
                    let tmp : &mut T = transmute::<*mut c_void, &mut T>(user_data);
                    Ok(tmp)
                },
                e => Err(e)
            }
        }
    }
}