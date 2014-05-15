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

use libc::{c_void, c_uint, c_int, c_char, c_float};
use ffi;
use types::*;
use enums::*;
use sound;
use sound::Sound;

pub struct FmodGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8, ..8] 
}

pub struct FmodSys {
    system : ffi::FMOD_SYSTEM,
}

impl FmodSys {
    pub fn new() -> Result<FmodSys, FMOD_RESULT> {
        let tmp = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_Create(&tmp) } {
            FMOD_OK => Ok(FmodSys{system : tmp}),
            err => Err(err)
        }
    }

    pub fn init(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Init(self.system, 1, FMOD_INIT_NORMAL, ::std::ptr::null()) }
    }

    pub fn release(&self) -> FMOD_RESULT {
        unsafe {
            ffi::FMOD_System_Close(self.system);
            ffi::FMOD_System_Release(self.system)
        }
    }

    pub fn create_sound(&self, music : StrBuf, options: Option<FmodMode>) -> Result<Sound, FMOD_RESULT> {
        let tmp_v = music.clone().into_owned();
        let sound = ::std::ptr::null();
        let op = match options {
            Some(FmodMode(t)) => t,
            None => FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM
        };

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateSound(self.system, c_str, op, ::std::ptr::null(), &sound) } {
                FMOD_OK => {Ok(sound::new(self.system, music.clone(), sound))},
                err => Err(err)
            }
        })
    }

    pub fn set_output(&self, output_type: FMOD_OUTPUTTYPE) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetOutput(self.system, output_type) }
    }

    pub fn get_output(&self) -> Result<FMOD_OUTPUTTYPE, FMOD_RESULT> {
        let t = FMOD_OUTPUTTYPE_AUTODETECT;
        
        match unsafe { ffi::FMOD_System_GetOutput(self.system, &t) } {
            FMOD_OK => Ok(t),
            e => Err(e)
        }
    }

    pub fn get_num_drivers(&self) -> Result<i32, FMOD_RESULT> {
        let t = 0i32;

        match unsafe { ffi::FMOD_System_GetNumDrivers(self.system, &t) } {
            FMOD_OK => Ok(t),
            e => Err(e)
        }
    }

    pub fn get_driver_info(&self, id: i32, name: StrBuf) -> Result<FmodGuid, FMOD_RESULT> {
        let tmp_v = name.clone().into_owned();
        let guid = ffi::FMOD_GUID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetDriverInfo(self.system, id, c_str, tmp_v.len() as i32, &guid) } {
                FMOD_OK => Ok(FmodGuid{data1: guid.Data1, data2: guid.Data2, data3: guid.Data3, data4: guid.Data4}),
                e => Err(e)
            }
        })
    }

    pub fn get_driver_caps(&self, id: i32) -> Result<(FmodCaps, i32, FMOD_SPEAKERMODE), FMOD_RESULT> {
        let c = 0u32;
        let s = FMOD_SPEAKERMODE_RAW;
        let cpor = 0i32;

        match unsafe { ffi::FMOD_System_GetDriverCaps(self.system, id, &c, &cpor, &s) } {
            FMOD_OK => Ok((FmodCaps(c), cpor, s)),
            e => Err(e)
        }
    }

    pub fn set_driver(&self, driver: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetDriver(self.system, driver) }
    }

    pub fn get_driver(&self) -> Result<i32, FMOD_RESULT> {
        let driver = 0i32;

        match unsafe { ffi::FMOD_System_GetDriver(self.system, &driver) } {
            FMOD_OK => Ok(driver),
            e => Err(e)
        }
    }

    pub fn set_hardware_channels(&self, num_hardware_channels: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetHardwareChannels(self.system, num_hardware_channels) }
    }

    pub fn get_hardware_channels(&self) -> Result<i32, FMOD_RESULT> {
        let t = 0i32;

        match unsafe { ffi::FMOD_System_GetHardwareChannels(self.system, &t) } {
            FMOD_OK => Ok(t),
            e => Err(e)
        }
    }

    pub fn set_software_channels(&self, num_software_channels: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSoftwareChannels(self.system, num_software_channels) }
    }

    pub fn get_software_channels(&self) -> Result<i32, FMOD_RESULT> {
        let t = 0i32;

        match unsafe { ffi::FMOD_System_GetSoftwareChannels(self.system, &t) } {
            FMOD_OK => Ok(t),
            e => Err(e)
        }
    }
}