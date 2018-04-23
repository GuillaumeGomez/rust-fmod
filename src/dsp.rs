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
use callbacks::*;
use dsp_connection;
use fmod_sys;
use fmod_sys::{MemoryUsageDetails, Sys};
use std::mem::transmute;
use channel;
use libc::{c_char, c_void, c_uint, c_int, c_float};
use std::default::Default;
use c_vec::CVec;
use std::ffi::CString;

extern "C" fn create_callback(dsp_state: *mut ffi::FMOD_DSP_STATE) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);

                match callbacks.callbacks.create_callback {
                    Some(p) => p(&from_state_ptr(::std::ptr::read(
                        dsp_state as *const ffi::FMOD_DSP_STATE))),
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

extern "C" fn release_callback(dsp_state: *mut ffi::FMOD_DSP_STATE) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                
                match callbacks.callbacks.release_callback {
                    Some(p) => p(&from_state_ptr(::std::ptr::read(
                        dsp_state as *const ffi::FMOD_DSP_STATE))),
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

extern "C" fn reset_callback(dsp_state: *mut ffi::FMOD_DSP_STATE) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                
                match callbacks.callbacks.reset_callback {
                    Some(p) => p(&from_state_ptr(::std::ptr::read(
                        dsp_state as *const ffi::FMOD_DSP_STATE))),
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

extern "C" fn read_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, in_buffer: *mut c_float,
                            out_buffer: *mut c_float, length: c_uint, in_channels: c_int,
                            out_channels: c_int) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                match callbacks.callbacks.read_callback {
                    Some(p) => {
                        let in_size = ((length as i32 - 1i32) * in_channels) + out_channels;
                        let out_size = ((length as i32 - 1i32) * out_channels) + out_channels;
                        let mut v_in_buffer = CVec::new(in_buffer,
                                                        in_size as usize);
                        let mut v_out_buffer = CVec::new(out_buffer, out_size as usize);

                        p(&from_state_ptr(::std::ptr::read(
                            dsp_state as *const ffi::FMOD_DSP_STATE)), v_in_buffer.as_mut(),
                            v_out_buffer.as_mut(), length as u32, in_channels as i32,
                            out_channels as i32)
                    },
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

extern "C" fn set_position_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, pos: c_uint) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                
                match callbacks.callbacks.set_pos_callback {
                    Some(p) => p(&from_state_ptr(::std::ptr::read(
                        dsp_state as *const ffi::FMOD_DSP_STATE)), pos as u32),
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

extern "C" fn set_parameter_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, index: c_int,
                                     value: c_float) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                
                match callbacks.callbacks.set_param_callback {
                    Some(p) => p(&from_state_ptr(::std::ptr::read(
                        dsp_state as *const ffi::FMOD_DSP_STATE)), index as i32, value),
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

extern "C" fn get_parameter_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, index: c_int,
                                     value: *mut c_float, value_str: *mut c_char) -> ::Status {
    unsafe {
        if !dsp_state.is_null() && !(*dsp_state).instance.is_null() {
            let mut tmp = ::std::ptr::null_mut();

            ffi::FMOD_DSP_GetUserData((*dsp_state).instance, &mut tmp);
            if !tmp.is_null() {
                let callbacks : &mut UserData = transmute(tmp);
                match callbacks.callbacks.get_param_callback {
                    Some(p) => {
                        let mut t_value = *value;
                        let l = ffi::strlen(value_str);
                        let tmp = String::from_raw_parts(value_str as *mut u8, l, l);

                        let ret = p(&from_state_ptr(::std::ptr::read(
                            dsp_state as *const ffi::FMOD_DSP_STATE)), index as i32,
                            &mut t_value, &tmp);
                        *value = t_value;
                        ret
                    },
                    None => ::Status::Ok
                }
            } else {
              ::Status::Ok
            }
        } else {
           ::Status::Ok
        }
    }
}

#[allow(unused_variables)]
extern "C" fn config_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, hwnd: *mut c_void,
                              show: c_int) -> ::Status {
   ::Status::Ok
}

struct UserData {
    callbacks: DspCallbacks,
    user_data: *mut c_void,
}

impl UserData {
    fn new() -> UserData {
        UserData {
            callbacks: DspCallbacks::new(),
            user_data: ::std::ptr::null_mut(),
        }
    }
}

struct DspCallbacks {
    create_callback: DspCreateCallback,
    release_callback: DspReleaseCallback,
    reset_callback: DspResetCallback,
    read_callback: DspReadCallback,
    set_pos_callback: DspSetPositionCallback,
    set_param_callback: DspSetParamCallback,
    get_param_callback: DspGetParamCallback,
}

impl DspCallbacks {
    fn new() -> DspCallbacks {
        DspCallbacks {
            create_callback: None,
            release_callback: None,
            reset_callback: None,
            read_callback: None,
            set_pos_callback: None,
            set_param_callback: None,
            get_param_callback: None,
        }
    }
}

impl Clone for DspCallbacks {
    fn clone(&self) -> DspCallbacks {
        DspCallbacks {
            create_callback: self.create_callback,
            release_callback: self.release_callback,
            reset_callback: self.reset_callback,
            read_callback: self.read_callback,
            set_pos_callback: self.set_pos_callback,
            set_param_callback: self.set_param_callback,
            get_param_callback: self.get_param_callback,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Structure to define a parameter for a DSP unit.
pub struct DspParameterDesc {
    /// [w] Minimum value of the parameter (ie 100.0)
    pub min         : f32,
    /// [w] Maximum value of the parameter (ie 22050.0)
    pub max         : f32,
    /// [w] Default value of parameter
    pub default_val : f32,
    /// [w] Name of the parameter to be displayed (ie "Cutoff frequency")
    pub name        : String,
    /// [w] Short string to be put next to value to denote the unit type (ie "hz")
    pub label       : String,
    /// [w] Description of the parameter to be displayed as a help item / tooltip for this parameter
    pub description : String,
}

impl Default for DspParameterDesc {
    fn default() -> DspParameterDesc {
        DspParameterDesc {
            min: 0f32,
            max: 0f32,
            default_val: 0f32,
            name: String::new(),
            label: String::new(),
            description: String::new(),
        }
    }
}

pub fn from_parameter_ptr(dsp_parameter: *mut ffi::FMOD_DSP_PARAMETERDESC) -> Result<DspParameterDesc, ::RStatus> {
    if !dsp_parameter.is_null() {
        let description = unsafe {
            let l = ffi::strlen((*dsp_parameter).description);
            String::from_raw_parts((*dsp_parameter).description as *mut u8, l, l)
        };
        let mut v1 : Vec<u8> = unsafe { Vec::with_capacity((*dsp_parameter).name.len()) };
        let mut v2 : Vec<u8> = unsafe { Vec::with_capacity((*dsp_parameter).label.len()) };

        unsafe {
            for i in (*dsp_parameter).name.iter() {
                if *i == 0 {
                    break
                }
                v1.push(*i as u8);
            }
        }
        let name = from_utf8!(v1);
        unsafe {
            for i in (*dsp_parameter).label.iter() {
                if *i == 0 {
                    break
                }
                v2.push(*i as u8);
            }
        }
        let label = from_utf8!(v2);
        unsafe {
            Ok(DspParameterDesc {
                min: (*dsp_parameter).min,
                max: (*dsp_parameter).max,
                default_val: (*dsp_parameter).default_val,
                name,
                label,
                description,
            })
        }
    } else {
        Ok(Default::default())
    }
}

pub fn get_parameter_ffi(dsp_parameter: &DspParameterDesc) -> Result<ffi::FMOD_DSP_PARAMETERDESC, ::RStatus> {
    let mut tmp_name = dsp_parameter.name.as_bytes().to_vec();
    let mut tmp_label = dsp_parameter.label.as_bytes().to_vec();
    let tmp_description = match CString::new(dsp_parameter.description.clone()) {
        Ok(s) => s,
        Err(e) => return Err(::RStatus::Other(format!("Issue with dsp_parameter: {}", e))),
    };

    tmp_name.truncate(16);
    tmp_label.truncate(16);
    tmp_name.reserve_exact(16);
    tmp_label.reserve_exact(16);
    Ok(ffi::FMOD_DSP_PARAMETERDESC {
        min: dsp_parameter.min,
        max: dsp_parameter.max,
        default_val: dsp_parameter.default_val,
        name: {
            let mut slice : [i8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let mut it = 0;

            for tmp in tmp_name.iter() {
                slice[it] = *tmp as i8;
                it += 1;
            }
            slice
        },
        label: {
            let mut slice : [i8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let mut it = 0;

            for tmp in tmp_label.iter() {
                slice[it] = *tmp as i8;
                it += 1;
            }
            slice
        },
        description: tmp_description.as_ptr() as *const c_char,
    })
}

/// When creating a DSP unit, declare one of these and provide the relevant callbacks and name for
/// FMOD to use when it creates and uses a DSP unit of this type.
pub struct DspDescription {
    /// [w] Name of the unit to be displayed in the network.
    pub name                : String,
    /// [w] Plugin writer's version number.
    pub version             : u32,
    /// [w] Number of channels. Use 0 to process whatever number of channels is currently in the
    /// network. > 0 would be mostly used if the unit is a unit that only generates sound.
    pub channels            : i32,
    /// [w] Create callback. This is called when DSP unit is created. Can be null.
    pub create              : DspCreateCallback,
    /// [w] Release callback. This is called just before the unit is freed so the user can do any
    /// cleanup needed for the unit. Can be null.
    pub release             : DspReleaseCallback,
    /// [w] Reset callback. This is called by the user to reset any history buffers that may need
    /// resetting for a filter, when it is to be used or re-used for the first time to its initial
    /// clean state. Use to avoid clicks or artifacts.
    pub reset               : DspResetCallback,
    /// [w] Read callback. Processing is done here. Can be null.
    pub read                : DspReadCallback,
    /// [w] Set position callback. This is called if the unit wants to update its position info but
    /// not process data, or reset a cursor position internally if it is reading data from a certain
    /// source. Can be null.
    pub set_position        : DspSetPositionCallback,
    /// [w] Number of parameters used in this filter. The user finds this with DSP::getNumParameters
    pub num_parameters      : i32,
    /// [w] Variable number of parameter structures.
    pub param_desc          : DspParameterDesc,
    /// [w] This is called when the user calls DSP::setParameter. Can be null.
    pub set_parameter       : DspSetParamCallback,
    /// [w] This is called when the user calls DSP::getParameter. Can be null.
    pub get_parameter       : DspGetParamCallback,
    /// [w] This is called when the user calls DSP::showConfigDialog. Can be used to display a
    /// dialog to configure the filter. Can be null.
    config                  : DspDialogCallback,
    /// [w] Width of config dialog graphic if there is one. 0 otherwise.
    pub config_width        : i32,
    /// [w] Height of config dialog graphic if there is one. 0 otherwise.
    pub config_height       : i32,
    /// [w] Optional. Specify 0 to ignore. This is user data to be attached to the DSP unit during
    /// creation. Access via DSP::getUserData.
    user_data               : Box<UserData>,
}

impl Default for DspDescription {
    fn default() -> DspDescription {
        DspDescription {
            name: String::new(),
            version: 0u32,
            channels: 0i32,
            create: None,
            release: None,
            reset: None,
            read: None,
            set_position: None,
            num_parameters: 0i32,
            param_desc: Default::default(),
            set_parameter: None,
            get_parameter: None,
            config: None,
            config_width: 0i32,
            config_height: 0i32,
            user_data: Box::new(UserData::new()),
        }
    }
}

pub fn get_description_ffi(dsp_description: &mut DspDescription) -> ffi::FMOD_DSP_DESCRIPTION {
    let mut tmp_s = dsp_description.name.as_bytes().to_vec();

    tmp_s.truncate(32);
    tmp_s.reserve_exact(32);
    ffi::FMOD_DSP_DESCRIPTION {
        name: {
            let mut slice : [i8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let mut it = 0;

            for tmp in tmp_s.iter() {
                slice[it] = *tmp as i8;
                it += 1;
            }
            slice
        },
        version: dsp_description.version,
        channels: dsp_description.channels,
        create: match dsp_description.create {
            Some(_) => Some(create_callback as extern "C" fn(*mut _) -> _),
            None => None
        },
        release: match dsp_description.release {
            Some(_) => Some(release_callback as extern "C" fn(*mut _) -> _),
            None => None
        },
        reset: match dsp_description.reset {
            Some(_) => Some(reset_callback as extern "C" fn(*mut _) -> _),
            None => None
        },
        read: match dsp_description.read {
            Some(_) => Some(read_callback as extern "C" fn(*mut _, *mut _, *mut _, _, _, _) -> _),
            None => None
        },
        set_position: match dsp_description.set_position {
            Some(_) => Some(set_position_callback as extern "C" fn(*mut _, _) -> _),
            None => None
        },
        num_parameters: dsp_description.num_parameters,
        param_desc: &mut get_parameter_ffi(&dsp_description.param_desc).expect("get_parameter_ffi failed")
                    as *mut ffi::FMOD_DSP_PARAMETERDESC,
        set_parameter: match dsp_description.set_parameter {
            Some(_) => Some(set_parameter_callback as extern "C" fn(*mut _, _, _) -> _),
            None => None
        },
        get_parameter: match dsp_description.get_parameter {
            Some(_) => Some(get_parameter_callback
                            as extern "C" fn(*mut _, _, *mut _, *mut _) -> _),
            None => None
        },
        config: match dsp_description.config {
            Some(_) => Some(config_callback as extern "C" fn(*mut _, *mut _, _) -> _),
            None => None
        },
        config_height: dsp_description.config_height,
        config_width: dsp_description.config_width,
        user_data: {
            dsp_description.user_data.callbacks.create_callback = dsp_description.create;
            dsp_description.user_data.callbacks.release_callback = dsp_description.release;
            dsp_description.user_data.callbacks.reset_callback = dsp_description.reset;
            dsp_description.user_data.callbacks.read_callback = dsp_description.read;
            dsp_description.user_data.callbacks.set_pos_callback = dsp_description.set_position;
            dsp_description.user_data.callbacks.set_param_callback = dsp_description.set_parameter;
            dsp_description.user_data.callbacks.get_param_callback = dsp_description.get_parameter;
            unsafe { transmute::<&mut UserData, *mut c_void>(&mut *dsp_description.user_data) }
        },
    }
}

pub fn get_state_ffi(state: &DspState) -> ffi::FMOD_DSP_STATE {
    ffi::FMOD_DSP_STATE {
        instance: ffi::FFI::unwrap(&state.instance),
        plugin_data: state.plugin_data,
        speaker_mask: state.speaker_mask
    }
}

pub fn from_state_ptr(state: ffi::FMOD_DSP_STATE) -> DspState {
    DspState {
        instance: ffi::FFI::wrap(state.instance),
        plugin_data: state.plugin_data,
        speaker_mask: state.speaker_mask
    }
}

/// DSP plugin structure that is passed into each callback.
pub struct DspState {
    /// [r] Handle to the DSP hand the user created. Not to be modified. C++ users cast toDSP to
    /// use.
    pub instance: Dsp,
    /// [w] Plugin writer created data the output author wants to attach to this object.
    plugin_data: *mut c_void,
    /// [w] Specifies which speakers the DSP effect is active on
    pub speaker_mask: u16,
}

pub fn from_ptr_first(dsp: *mut ffi::FMOD_DSP) -> Dsp {
    Dsp {
        dsp: dsp,
        can_be_deleted: true,
        user_data: UserData {
            callbacks: DspCallbacks::new(),
            user_data: ::std::ptr::null_mut()
        }
    }
}

/// Dsp object
pub struct Dsp {
    dsp: *mut ffi::FMOD_DSP,
    can_be_deleted: bool,
    user_data: UserData
}

impl ffi::FFI<ffi::FMOD_DSP> for Dsp {
    fn wrap(dsp: *mut ffi::FMOD_DSP) -> Dsp {
        Dsp {
            dsp: dsp,
            can_be_deleted: false,
            user_data: UserData {
                callbacks: DspCallbacks::new(),
                user_data: ::std::ptr::null_mut()
            }
        }
    }

    fn unwrap(d: &Dsp) -> *mut ffi::FMOD_DSP {
        d.dsp
    }
}

impl Drop for Dsp {
    fn drop(&mut self) {
        self.release();
    }
}

impl Dsp {
    pub fn get_system_object(&self) -> Result<Sys, ::Status> {
        let mut system = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_DSP_GetSystemObject(self.dsp, &mut system) } {
            ::Status::Ok => Ok(ffi::FFI::wrap(system)),
            e => Err(e)
        }
    }

    pub fn release(&mut self) -> ::Status {
        if self.can_be_deleted && !self.dsp.is_null() {
            match unsafe { ffi::FMOD_DSP_Release(self.dsp) } {
               ::Status::Ok => {
                    self.dsp =::std::ptr::null_mut();
                   ::Status::Ok
                }
                e => e
            }
        } else {
           ::Status::Ok
        }
    }

    pub fn play(&self) -> Result<channel::Channel, ::Status> {
        let mut channel = ::std::ptr::null_mut();

        match match self.get_system_object() {
            Ok(s) => { 
                unsafe { ffi::FMOD_System_PlayDSP(ffi::FFI::unwrap(&s), ::ChannelIndex::Free,
                                                  self.dsp, 0, &mut channel) }
            }
            Err(e) => e
        } {
            ::Status::Ok => Ok(ffi::FFI::wrap(channel)),
            e => Err(e)
        }
    }

    pub fn play_with_parameters(&self, channel_id: ::ChannelIndex)
                                -> Result<channel::Channel, ::Status> {
        let mut channel = ::std::ptr::null_mut();
        
        match match self.get_system_object() {
            Ok(s) => { 
                unsafe { ffi::FMOD_System_PlayDSP(ffi::FFI::unwrap(&s), channel_id, self.dsp, 0,
                                                  &mut channel) }
            }
            Err(e) => e
        } {
            ::Status::Ok => Ok(ffi::FFI::wrap(channel)),
            e => Err(e)
        }
    }

    pub fn add_input(&self, target: Dsp) -> Result<dsp_connection::DspConnection, ::Status> {
        let mut connection = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_DSP_AddInput(self.dsp, target.dsp, &mut connection) } {
            ::Status::Ok => Ok(ffi::FFI::wrap(connection)),
            e => Err(e)
        }
    }

    pub fn disconnect_from(&self, target: Dsp) -> ::Status {
        unsafe { ffi::FMOD_DSP_DisconnectFrom(self.dsp, target.dsp) }
    }

    pub fn disconnect_all(&self, inputs: bool, outputs: bool) -> ::Status {
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

    pub fn remove(&self) -> ::Status {
        unsafe { ffi::FMOD_DSP_Remove(self.dsp) }
    }

    pub fn get_num_inputs(&self) -> Result<i32, ::Status> {
        let mut inputs = 0i32;

        match unsafe { ffi::FMOD_DSP_GetNumInputs(self.dsp, &mut inputs) } {
            ::Status::Ok => Ok(inputs),
            e => Err(e)
        }
    }

    pub fn get_num_outputs(&self) -> Result<i32, ::Status> {
        let mut outputs = 0i32;

        match unsafe { ffi::FMOD_DSP_GetNumOutputs(self.dsp, &mut outputs) } {
            ::Status::Ok => Ok(outputs),
            e => Err(e)
        }
    }

    pub fn get_input(&self, index: i32) -> Result<(Dsp, dsp_connection::DspConnection), ::Status> {
        let mut input = ::std::ptr::null_mut();
        let mut input_connection = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_DSP_GetInput(self.dsp, index, &mut input,
                                              &mut input_connection) } {
            ::Status::Ok => Ok((ffi::FFI::wrap(input), ffi::FFI::wrap(input_connection))),
            e => Err(e)
        }
    }

    pub fn get_output(&self, index: i32) -> Result<(Dsp, dsp_connection::DspConnection), ::Status> {
        let mut output = ::std::ptr::null_mut();
        let mut output_connection = ::std::ptr::null_mut();

        match unsafe { ffi::FMOD_DSP_GetOutput(self.dsp, index, &mut output,
                                               &mut output_connection) } {
            ::Status::Ok => Ok((ffi::FFI::wrap(output), ffi::FFI::wrap(output_connection ))),
            e => Err(e)
        }
    }

    pub fn set_active(&self, active: bool) -> ::Status {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_DSP_SetActive(self.dsp, t_active) }
    }

    pub fn get_active(&self) -> Result<bool, ::Status> {
        let mut active = 0i32;

        match unsafe { ffi::FMOD_DSP_GetActive(self.dsp, &mut active) } {
            ::Status::Ok => Ok(active != 0i32),
            e => Err(e)
        }
    }

    pub fn set_bypass(&self, bypass: bool) -> ::Status {
        let t_bypass = if bypass == true {
            1i32
        } else {
            0i32
        };

        unsafe { ffi::FMOD_DSP_SetBypass(self.dsp, t_bypass) }
    }

    pub fn get_bypass(&self) -> Result<bool, ::Status> {
        let mut bypass = 0i32;

        match unsafe { ffi::FMOD_DSP_GetBypass(self.dsp, &mut bypass) } {
            ::Status::Ok => Ok(bypass == 1i32),
            e => Err(e)
        }
    }

    pub fn set_speaker_active(&self, speaker: ::Speaker, active: bool) -> ::Status {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_DSP_SetSpeakerActive(self.dsp, speaker, t_active) }
    }

    pub fn get_speaker_active(&self, speaker: ::Speaker) -> Result<bool, ::Status> {
        let mut active = 0i32;

        match unsafe { ffi::FMOD_DSP_GetSpeakerActive(self.dsp, speaker, &mut active) } {
            ::Status::Ok => Ok(active == 1i32),
            e => Err(e)
        }
    }

    pub fn reset(&self) -> ::Status {
        unsafe { ffi::FMOD_DSP_Reset(self.dsp) }
    }

    /// value argument depends directly on the index argument,
    /// index argument depends on your DSP type, it is a value from one of the following enums :
    /// 
    /// * [`DspType`](enums/fmod/type.DspType.html)
    /// * [`DspOscillator`](enums/fmod/type.DspOscillator.html)
    /// * [`DspLowPass`](enums/fmod/type.DspLowPass.html)
    /// * [`DspITLowPass`](enums/fmod/type.DspITLowPass.html)
    /// * [`DspHighPass`](enums/fmod/type.DspHighPass.html)
    /// * [`DspTypeEcho`](enums/fmod/type.DspTypeEcho.html)
    /// * [`DspDelay`](enums/fmod/type.DspDelay.html)
    /// * [`DspFlange`](enums/fmod/type.DspFlange.html)
    /// * [`DspTremolo`](enums/fmod/type.DspTremolo.html)
    /// * [`DspDistortion`](enums/fmod/type.DspDistortion.html)
    /// * [`DspNormalize`](enums/fmod/type.DspNormalize.html)
    /// * [`DspTypeParameq`](enums/fmod/type.DspTypeParameq.html)
    /// * [`DspPitchShift`](enums/fmod/type.DspPitchShift.html)
    /// * [`DspChorus`](enums/fmod/type.DspChorus.html)
    /// * [`DspITEcho`](enums/fmod/type.DspITEcho.html)
    /// * [`DspCompressor`](enums/fmod/type.DspCompressor.html)
    /// * [`DspSfxReverb`](enums/fmod/type.DspSfxReverb.html)
    /// * [`DspLowPassSimple`](enums/fmod/type.DspLowPassSimple.html)
    /// * [`DspHighPassSimple`](enums/fmod/type.DspHighPassSimple.html)
    pub fn set_parameter(&self, index: i32, value: f32) -> ::Status {
        unsafe { ffi::FMOD_DSP_SetParameter(self.dsp, index, value) }
    }

    /// value result depends directly on the index argument,
    /// index argument depends on your DSP type, it is a value from one of the following enums:
    /// 
    /// * [`DspType`](enums/fmod/type.DspType.html)
    /// * [`DspOscillator`](enums/fmod/type.DspOscillator.html)
    /// * [`DspLowPass`](enums/fmod/type.DspLowPass.html)
    /// * [`DspITLowPass`](enums/fmod/type.DspITLowPass.html)
    /// * [`DspHighPass`](enums/fmod/type.DspHighPass.html)
    /// * [`DspTypeEcho`](enums/fmod/type.DspTypeEcho.html)
    /// * [`DspDelay`](enums/fmod/type.DspDelay.html)
    /// * [`DspFlange`](enums/fmod/type.DspFlange.html)
    /// * [`DspTremolo`](enums/fmod/type.DspTremolo.html)
    /// * [`DspDistortion`](enums/fmod/type.DspDistortion.html)
    /// * [`DspNormalize`](enums/fmod/type.DspNormalize.html)
    /// * [`DspTypeParameq`](enums/fmod/type.DspTypeParameq.html)
    /// * [`DspPitchShift`](enums/fmod/type.DspPitchShift.html)
    /// * [`DspChorus`](enums/fmod/type.DspChorus.html)
    /// * [`DspITEcho`](enums/fmod/type.DspITEcho.html)
    /// * [`DspCompressor`](enums/fmod/type.DspCompressor.html)
    /// * [`DspSfxReverb`](enums/fmod/type.DspSfxReverb.html)
    /// * [`DspLowPassSimple`](enums/fmod/type.DspLowPassSimple.html)
    /// * [`DspHighPassSimple`](enums/fmod/type.DspHighPassSimple.html)
    pub fn get_parameter(&self, index: i32, value_str_len: usize)
                        -> Result<(f32, String), ::RStatus> {
        let mut value = 0f32;
        let mut c = Vec::with_capacity(value_str_len + 1);

        for _ in 0..(value_str_len + 1) {
            c.push(0);
        }

        match unsafe { ffi::FMOD_DSP_GetParameter(self.dsp, index, &mut value,
                                                  c.as_mut_ptr() as *mut c_char,
                                                  value_str_len as i32) } {
           ::Status::Ok => {
                let c = from_utf8!(c);
                Ok((value, c))
            }
            e => Err(::RStatus::FMOD(e))
        }
    }

    pub fn get_num_parameters(&self) -> Result<i32, ::Status> {
        let mut num_param = 0i32;

        match unsafe { ffi::FMOD_DSP_GetNumParameters(self.dsp, &mut num_param) } {
            ::Status::Ok => Ok(num_param),
            e => Err(e)
        }
    }

    pub fn get_parameter_info(&self, index: i32, name: &str, label: &str,
                              description_len: usize) -> Result<(String, f32, f32), ::RStatus> {
        let mut min = 0f32;
        let mut max = 0f32;
        let t_name = name.clone();
        let t_label = label.clone();
        let mut description = Vec::with_capacity(description_len + 1);

        for _ in 0..(description_len + 1) {
            description.push(0);
        }

        match unsafe { ffi::FMOD_DSP_GetParameterInfo(self.dsp, index,
                                                      t_name.as_ptr() as *mut c_char,
                                                      t_label.as_ptr() as *mut c_char,
                                                      description.as_mut_ptr() as *mut c_char,
                                                      description_len as i32, &mut min,
                                                      &mut max) } {
            ::Status::Ok => Ok((from_utf8!(description), min, max)),
            e => Err(::RStatus::FMOD(e)),
        }
    }

    pub fn get_info(&self, name: &str) -> Result<(u32, i32, i32, i32), ::Status> {
        let mut version = 0u32;
        let mut channels = 0i32;
        let mut config_width = 0i32;
        let mut config_height = 0i32;
        let tmp_n = name.clone();

        match unsafe { ffi::FMOD_DSP_GetInfo(self.dsp, tmp_n.as_ptr() as *mut c_char, &mut version,
                                             &mut channels, &mut config_width,
            &mut config_height) } {
            ::Status::Ok => Ok((version, channels, config_width, config_height)),
            e => Err(e)
        }
    }

    pub fn set_defaults(&self, frequency: f32, volume: f32, pan: f32, priority: i32) -> ::Status {
        unsafe { ffi::FMOD_DSP_SetDefaults(self.dsp, frequency, volume, pan, priority) }
    }

    pub fn get_type(&self) -> Result<::DspType, ::Status> {
        let mut _type = ::DspType::Unknown;

        match unsafe { ffi::FMOD_DSP_GetType(self.dsp, &mut _type) } {
            ::Status::Ok => Ok(_type),
            e => Err(e)
        }
    }

    pub fn get_defaults(&self) -> Result<(f32, f32, f32, i32), ::Status> {
        let mut frequency = 0f32;
        let mut volume = 0f32;
        let mut pan = 0f32;
        let mut priority = 0i32;

        match unsafe { ffi::FMOD_DSP_GetDefaults(self.dsp, &mut frequency, &mut volume, &mut pan,
                                                 &mut priority) } {
            ::Status::Ok => Ok((frequency, volume, pan, priority)),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, MemoryBits(memory_bits): MemoryBits,
                           EventMemoryBits(event_memory_bits): EventMemoryBits)
                           -> Result<(u32, MemoryUsageDetails), ::Status> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_DSP_GetMemoryInfo(self.dsp, memory_bits, event_memory_bits,
                                                   &mut memory_used, &mut details) } {
            ::Status::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    pub fn set_user_data<'r, T>(&'r mut self, user_data: &'r mut T) -> ::Status {
        let mut data: *mut c_void = ::std::ptr::null_mut();

        unsafe {
            match ffi::FMOD_DSP_GetUserData(self.dsp, &mut data) {
               ::Status::Ok => {
                    if data.is_null() {
                        self.user_data.user_data = ::std::ptr::null_mut();

                        ffi::FMOD_DSP_SetUserData(self.dsp, transmute(&mut self.user_data))
                    } else {
                        let tmp : &mut UserData = transmute::<*mut c_void, &mut UserData>(data);

                        tmp.user_data = transmute::<&mut T, *mut c_void>(user_data);
                        ffi::FMOD_DSP_SetUserData(self.dsp, transmute(tmp))
                    }
                }
                _ => {
                    self.user_data.user_data = transmute::<&mut T, *mut c_void>(user_data);

                    ffi::FMOD_DSP_SetUserData(self.dsp, transmute(&mut self.user_data))
                }
            }
        }
    }

    pub fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, ::Status> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::null_mut();

            match ffi::FMOD_DSP_GetUserData(self.dsp, &mut user_data) {
               ::Status::Ok => {
                    if !user_data.is_null() {
                        let tmp: &mut UserData = transmute::<*mut c_void, &mut UserData>(user_data);
                        let tmp2: &mut T = transmute::<*mut c_void, &mut T>(tmp.user_data);

                        Ok(tmp2)
                    } else {
                        Err(::Status::Ok)
                    }
                },
                e => Err(e)
            }
        }
    }
}
