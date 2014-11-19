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
use enums;
use vector;
use reverb_properties;
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;
use std::mem::transmute;
use libc::{c_void};
use std::default::Default;

/// Reverb object
pub struct Reverb {
    reverb: *mut ffi::FMOD_REVERB
}

impl Drop for Reverb {
    fn drop(&mut self) {
        self.release();
    }
}

impl ffi::FFI<ffi::FMOD_REVERB> for Reverb {
    fn wrap(r: *mut ffi::FMOD_REVERB) -> Reverb {
        Reverb {reverb: r}
    }

    fn unwrap(r: &Reverb) -> *mut ffi::FMOD_REVERB {
        r.reverb
    }
}

impl Reverb {
    pub fn release(&mut self) -> enums::Result {
        if self.reverb !=::std::ptr::null_mut() {
            match unsafe { ffi::FMOD_Reverb_Release(self.reverb) } {
                enums::Result::Ok => {
                    self.reverb = ::std::ptr::null_mut();
                    enums::Result::Ok
                }
                e => e
            }
        } else {
            enums::Result::Ok
        }
    }

    pub fn set_3D_attributes(&self, position: vector::FmodVector, min_distance: f32, max_distance: f32) -> enums::Result {
        let t_position = vector::get_ffi(&position);

        unsafe { ffi::FMOD_Reverb_Set3DAttributes(self.reverb, &t_position, min_distance, max_distance) }
    }

    pub fn get_3D_attributes(&self) -> Result<(vector::FmodVector, f32, f32), enums::Result> {
        let mut position = vector::get_ffi(&vector::FmodVector::new());
        let mut min_distance = 0f32;
        let mut max_distance = 0f32;

        match unsafe { ffi::FMOD_Reverb_Get3DAttributes(self.reverb, &mut position, &mut min_distance, &mut max_distance) } {
            enums::Result::Ok => Ok((vector::from_ptr(position), min_distance, max_distance)),
            e => Err(e)
        }
    }

    pub fn set_properties(&self, reverb_properties: reverb_properties::ReverbProperties) -> enums::Result {
        let t_reverb_properties = reverb_properties::get_ffi(reverb_properties);

        unsafe { ffi::FMOD_Reverb_SetProperties(self.reverb, &t_reverb_properties) }
    }

    pub fn get_properties(&self, reverb_properties: reverb_properties::ReverbProperties) -> Result<reverb_properties::ReverbProperties, enums::Result> {
        let mut t_reverb_properties = reverb_properties::get_ffi(reverb_properties);

        match unsafe { ffi::FMOD_Reverb_GetProperties(self.reverb, &mut t_reverb_properties) } {
            enums::Result::Ok => Ok(reverb_properties::from_ptr(t_reverb_properties)),
            e => Err(e)
        }
    }

    pub fn set_active(&self, active: bool) -> enums::Result {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_Reverb_SetActive(self.reverb, t_active) }
    }

    pub fn get_active(&self) -> Result<bool, enums::Result> {
        let mut active = 0i32;

        match unsafe { ffi::FMOD_Reverb_GetActive(self.reverb, &mut active) } {
            enums::Result::Ok => Ok(active == 1),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), enums::Result> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_Reverb_GetMemoryInfo(self.reverb, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            enums::Result::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    pub fn set_user_data<T>(&self, user_data: &mut T) -> enums::Result {
        unsafe { ffi::FMOD_Reverb_SetUserData(self.reverb, transmute(user_data)) }
    }

    pub fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, enums::Result> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::null_mut();

            match ffi::FMOD_Reverb_GetUserData(self.reverb, &mut user_data) {
                enums::Result::Ok => {
                    let tmp : &mut T = transmute::<*mut c_void, &mut T>(user_data);
                    
                    Ok(tmp)
                },
                e => Err(e)
            }
        }
    }
}