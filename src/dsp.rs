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
use callbacks::*;
use dsp_connection;
use fmod_sys;
use fmod_sys::{FmodMemoryUsageDetails, FmodSys};
use std::mem::transmute;
use channel;
use libc::{c_char, c_void, c_uint, c_int, c_float, c_ushort};
use std;
use std::collections::hashmap::HashMap;
use std::c_str::CString;

fn get_callbacks() -> HashMap<*mut ffi::FMOD_DSP, DspCallbacks> {
    local_data_key!(key: HashMap<*mut ffi::FMOD_DSP, DspCallbacks>);
    if key.get().is_none() {
        let map : HashMap<*mut ffi::FMOD_DSP, DspCallbacks> = HashMap::new();

        key.replace(Some(map.clone()));
        map.clone()
    } else {
        match key.get() {
            Some(p) => p.clone(),
            None => {
                let map : HashMap<*mut ffi::FMOD_DSP, DspCallbacks> = HashMap::new();

                map
            }
        }
    }
}

fn get_callback_by_id(id: *mut ffi::FMOD_DSP) -> Option<DspCallbacks> {
    local_data_key!(key: HashMap<*mut ffi::FMOD_DSP, DspCallbacks>);
    if key.get().is_none() {
        None
    } else {
        match (*key.get().unwrap()).find(&id) {
            Some(calls) => {
                Some(*calls)
            },
            None => None
        }
    }
}

fn remove_callback(map: HashMap<*mut ffi::FMOD_DSP, DspCallbacks>) {
    local_data_key!(key: HashMap<*mut ffi::FMOD_DSP, DspCallbacks>);
    key.replace(Some(map));
}

fn register_callbacks(map: &mut HashMap<*mut ffi::FMOD_DSP, DspCallbacks>, ptr: *mut ffi::FMOD_DSP, callbacks: DspCallbacks) {
    println!("register");
    match get_callback_by_id(ptr) {
        Some(_) => {}
        None => {
            local_data_key!(key: HashMap<*mut ffi::FMOD_DSP, DspCallbacks>);
            if key.get().is_none() {
                let mut t_map = HashMap::new();

                t_map.insert(ptr, callbacks);
                key.replace(Some(t_map));
            } else {
                map.insert(ptr, callbacks);
                key.replace(Some(map.clone()));
            }
        }
    }
}

extern "C" fn create_callback(dsp_state: *mut ffi::FMOD_DSP_STATE) -> fmod::Result {
    unsafe {
        match get_callback_by_id((*dsp_state).instance) {
            Some(c) => match c.create_callback {
                Some(p) => p(&from_state_ptr(*dsp_state)),
                None => fmod::Ok
            },
            None => fmod::Ok
        }
    }
}

extern "C" fn release_callback(dsp_state: *mut ffi::FMOD_DSP_STATE) -> fmod::Result {
    unsafe {
        match get_callback_by_id((*dsp_state).instance) {
            Some(c) => match c.release_callback {
                Some(p) => p(&from_state_ptr(*dsp_state)),
                None => fmod::Ok
            },
            None => fmod::Ok
        }
    }
}

extern "C" fn reset_callback(dsp_state: *mut ffi::FMOD_DSP_STATE) -> fmod::Result {
    unsafe {
        match get_callback_by_id((*dsp_state).instance) {
            Some(c) => match c.reset_callback { 
                Some(p) => p(&from_state_ptr(*dsp_state)),
                None => fmod::Ok
            },
            None => fmod::Ok
        }
    }
}

extern "C" fn read_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, in_buffer: *mut c_float, out_buffer: *mut c_float, length: c_uint,
    in_channels: c_int, out_channels: c_int) -> fmod::Result {
    if dsp_state.is_not_null() {
        unsafe {
            println!("go");
            match get_callback_by_id((*dsp_state).instance) {
                Some(c) => {
                    println!("go !");
                    let mut v_in_buffer = Vec::new();
                    let mut v_out_buffer = Vec::new();

                    for count in range(0i32, length as i32) {
                        for count2 in range(0i32, out_channels) {
                            v_in_buffer.push(*in_buffer.offset((count * in_channels) as int + count2 as int));
                            v_out_buffer.push(*out_buffer.offset((count * out_channels) as int + count2 as int));
                        }
                    }
                    //let mut v_in_buffer = Vec::from_slice(std::c_vec::CVec::new(in_buffer, length as uint * in_channels as uint + out_channels as uint).as_slice());
                    //let mut v_out_buffer = Vec::from_slice(std::c_vec::CVec::new(out_buffer, length as uint * out_channels as uint + out_channels as uint).as_slice());
                    let ret = match c.read_callback {
                        Some(p) => p(&from_state_ptr(*dsp_state), &mut v_in_buffer, &mut v_out_buffer, length as u32, in_channels as i32, out_channels as i32),
                        None => fmod::Ok
                    };
                    let mut it = 0;
                    for count in range(0i32, length as i32) {
                        for count2 in range(0i32, out_channels) {
                            *in_buffer.offset((count * in_channels) as int + count2 as int) = *v_in_buffer.get(it);
                            *out_buffer.offset((count * out_channels) as int + count2 as int) = *v_out_buffer.get(it);
                            it += 1;
                        }
                    }
                    ret
                }
                None => fmod::Ok
            }
        }
    } else {
        fmod::Ok
    }
}

extern "C" fn set_position_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, pos: c_uint) -> fmod::Result {
    unsafe {
        match get_callback_by_id((*dsp_state).instance) {
            Some(c) => match c.set_pos_callback {
                Some(p) => p(&from_state_ptr(*dsp_state), pos as u32),
                None => fmod::Ok
            },
            None => fmod::Ok
        }
    }
}

extern "C" fn set_parameter_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, index: c_int, value: c_float) -> fmod::Result {
    unsafe {
        match get_callback_by_id((*dsp_state).instance) {
            Some(c) => match c.set_param_callback {
                Some(p) => p(&from_state_ptr(*dsp_state), index as i32, value),
                None => fmod::Ok
            },
            None => fmod::Ok
        }
    }
}

extern "C" fn get_parameter_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, index: c_int, value: *mut c_float, value_str: *mut c_char) -> fmod::Result {
    unsafe {
        match get_callback_by_id((*dsp_state).instance) {
            Some(c) => match c.get_param_callback {
                Some(p) => {
                    let mut t_value = *value;

                    let ret = p(&from_state_ptr(*dsp_state),
                        index as i32,
                        &mut t_value,
                        ::std::str::raw::from_c_str(value_str as *const c_char).as_slice());
                    *value = t_value;
                    ret
                },
                None => fmod::Ok
            },
            None => fmod::Ok
        }
    }
}

#[allow(unused_variable)]
extern "C" fn config_callback(dsp_state: *mut ffi::FMOD_DSP_STATE, hwnd: *mut c_void, show: c_int) -> fmod::Result {
    fmod::Ok
}

struct DspCallbacks {
    create_callback: DspCreateCallback,
    release_callback: DspReleaseCallback,
    reset_callback: DspResetCallback,
    read_callback: DspReadCallback,
    set_pos_callback: DspSetPositionCallback,
    set_param_callback: DspSetParamCallback,
    get_param_callback: DspGetParamCallback
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
            get_param_callback: None
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
            get_param_callback: self.get_param_callback
        }
    }
}

pub struct DspParameterDesc
{
    pub min         : f32,          /* [w] Minimum value of the parameter (ie 100.0). */
    pub max         : f32,          /* [w] Maximum value of the parameter (ie 22050.0). */
    pub default_val : f32,          /* [w] Default value of parameter. */
    pub name        : String,       /* [w] Name of the parameter to be displayed (ie "Cutoff frequency"). */
    pub label       : String,       /* [w] Short string to be put next to value to denote the unit type (ie "hz"). */
    pub description : String        /* [w] Description of the parameter to be displayed as a help item / tooltip for this parameter. */
}

pub fn from_parameter_ptr(dsp_parameter: *mut ffi::FMOD_DSP_PARAMETERDESC) -> DspParameterDesc {
    if dsp_parameter.is_not_null() {
        unsafe {
            DspParameterDesc {
                min: (*dsp_parameter).min,
                max: (*dsp_parameter).max,
                default_val: (*dsp_parameter).default_val,
                name: match CString::new((*dsp_parameter).name.as_ptr(), true).as_str() {
                    Some(s) => s.to_string(),
                    None => "".to_string()
                },
                label: match CString::new((*dsp_parameter).label.as_ptr(), true).as_str() {
                    Some(s) => s.to_string(),
                    None => "".to_string()
                },
                description: ::std::str::raw::from_c_str((*dsp_parameter).description.clone())
            }
        }
    } else {
        new_parameter()
    }
}

pub fn get_parameter_ffi(dsp_parameter: &DspParameterDesc) -> ffi::FMOD_DSP_PARAMETERDESC {
    let mut tmp_name = Vec::from_slice(dsp_parameter.name.as_bytes());
    let mut tmp_label = Vec::from_slice(dsp_parameter.label.as_bytes());

    tmp_name.truncate(16);
    tmp_label.truncate(16);
    tmp_name.reserve_exact(16);
    tmp_label.reserve_exact(16);
    dsp_parameter.description.as_slice().with_c_str(|c_description| {
        ffi::FMOD_DSP_PARAMETERDESC {
            min: dsp_parameter.min,
            max: dsp_parameter.max,
            default_val: dsp_parameter.default_val,
            name: {
                let mut slice : [i8, ..16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                let mut it = 0;

                for tmp in tmp_name.iter() {
                    slice[it] = *tmp as i8;
                    it += 1;
                }
                slice
            },
            label: {
                let mut slice : [i8, ..16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                let mut it = 0;

                for tmp in tmp_label.iter() {
                    slice[it] = *tmp as i8;
                    it += 1;
                }
                slice
            },
            description: c_description
        }
    })
}

pub fn new_parameter() -> DspParameterDesc {
    DspParameterDesc {
        min: 0f32,
        max: 0f32,
        default_val: 0f32,
        name: String::new(),
        label: String::new(),
        description: String::new()
    }
}

pub struct DspDescription
{
    pub name                : String,                      /* [w] Name of the unit to be displayed in the network. */
    pub version             : u32,                         /* [w] Plugin writer's version number. */
    pub channels            : i32,                         /* [w] Number of channels.  Use 0 to process whatever number of channels is currently in the network.  >0 would be mostly used if the unit is a unit that only generates sound. */
    pub create              : DspCreateCallback,      /* [w] Create callback.  This is called when DSP unit is created.  Can be null. */
    pub release             : DspReleaseCallback,     /* [w] Release callback.  This is called just before the unit is freed so the user can do any cleanup needed for the unit.  Can be null. */
    pub reset               : DspResetCallback,       /* [w] Reset callback.  This is called by the user to reset any history buffers that may need resetting for a filter, when it is to be used or re-used for the first time to its initial clean state.  Use to avoid clicks or artifacts. */
    pub read                : DspReadCallback,        /* [w] Read callback.  Processing is done here.  Can be null. */
    pub set_position        : DspSetPositionCallback, /* [w] Set position callback.  This is called if the unit wants to update its position info but not process data, or reset a cursor position internally if it is reading data from a certain source.  Can be null. */
    pub num_parameters      : i32,                         /* [w] Number of parameters used in this filter.  The user finds this with DSP::getNumParameters */
    pub param_desc          : DspParameterDesc,            /* [w] Variable number of parameter structures. */
    pub set_parameter       : DspSetParamCallback,    /* [w] This is called when the user calls DSP::setParameter.  Can be null. */
    pub get_parameter       : DspGetParamCallback,    /* [w] This is called when the user calls DSP::getParameter.  Can be null. */
    config                  : DspDialogCallback,      /* [w] This is called when the user calls DSP::showConfigDialog.  Can be used to display a dialog to configure the filter.  Can be null. */
    pub config_width        : i32,                         /* [w] Width of config dialog graphic if there is one.  0 otherwise.*/
    pub config_height       : i32,                         /* [w] Height of config dialog graphic if there is one.  0 otherwise.*/
    user_data               : *mut c_void             /* [w] Optional. Specify 0 to ignore. This is user data to be attached to the DSP unit during creation.  Access via DSP::getUserData. */
}

impl DspDescription {
    pub fn new() -> DspDescription {
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
            param_desc: new_parameter(),
            set_parameter: None,
            get_parameter: None,
            config: None,
            config_width: 0i32,
            config_height: 0i32,
            user_data: std::ptr::mut_null()
        }
    }
}

pub fn from_description_ptr(dsp_description: &ffi::FMOD_DSP_DESCRIPTION) -> DspDescription {
    DspDescription {
        name: unsafe {
            match CString::new(dsp_description.name.as_ptr(), true).as_str() {
                Some(s) => s.to_string(),
                None => "".to_string()
            }
        },
        version: dsp_description.version,
        channels: dsp_description.channels,
        create: None,
        release: None,
        reset: None,
        read: None,
        set_position: None,
        num_parameters: dsp_description.num_parameters,
        param_desc: from_parameter_ptr(dsp_description.param_desc),
        set_parameter: None,
        get_parameter: None,
        config: None,
        config_height: dsp_description.config_height,
        config_width: dsp_description.config_width,
        user_data: dsp_description.user_data
    }
}

pub fn get_description_ffi(dsp_description: &DspDescription) -> ffi::FMOD_DSP_DESCRIPTION {
    let mut tmp_s = Vec::from_slice(dsp_description.name.as_bytes());

    tmp_s.truncate(32);
    tmp_s.reserve_exact(32);
    ffi::FMOD_DSP_DESCRIPTION {
        name: {
            let mut slice : [i8, ..32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
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
            Some(_) => Some(create_callback),
            None => None
        },
        release: match dsp_description.release {
            Some(_) => Some(release_callback),
            None => None
        },
        reset: match dsp_description.reset {
            Some(_) => Some(reset_callback),
            None => None
        },
        read: match dsp_description.read {
            Some(_) => Some(read_callback),
            None => None
        },
        set_position: match dsp_description.set_position {
            Some(_) => Some(set_position_callback),
            None => None
        },
        num_parameters: dsp_description.num_parameters,
        param_desc: &mut get_parameter_ffi(&dsp_description.param_desc) as *mut ffi::FMOD_DSP_PARAMETERDESC,
        set_parameter: match dsp_description.set_parameter {
            Some(_) => Some(set_parameter_callback),
            None => None
        },
        get_parameter: match dsp_description.get_parameter {
            Some(_) => Some(get_parameter_callback),
            None => None
        },
        config: match dsp_description.config {
            Some(_) => Some(config_callback),
            None => None
        },
        config_height: dsp_description.config_height,
        config_width: dsp_description.config_width,
        user_data: dsp_description.user_data
    }
}

pub fn get_state_ffi(state: &DspState) -> ffi::FMOD_DSP_STATE {
    ffi::FMOD_DSP_STATE {
        instance: get_ffi(&state.instance),
        plugin_data: state.plugin_data,
        speaker_mask: state.speaker_mask
    }
}

pub fn from_state_ptr(state: ffi::FMOD_DSP_STATE) -> DspState {
    DspState {
        instance: from_ptr(state.instance),
        plugin_data: state.plugin_data,
        speaker_mask: state.speaker_mask
    }
}

pub struct DspState
{
    pub instance: Dsp,          /* [r] Handle to the DSP hand the user created.  Not to be modified.  C++ users cast to FMOD::DSP to use.  */
    plugin_data: *mut c_void,   /* [w] Plugin writer created data the output author wants to attach to this object. */
    pub speaker_mask: u16       /* [w] Specifies which speakers the DSP effect is active on */
}

pub fn from_ptr(dsp: *mut ffi::FMOD_DSP) -> Dsp {
    let mut map = get_callbacks();
    register_callbacks(&mut map, dsp, DspCallbacks::new());
    Dsp {
        dsp: dsp,
        can_be_deleted: false
    }
}

pub fn from_ptr_first(dsp: *mut ffi::FMOD_DSP) -> Dsp {
    let mut map = get_callbacks();
    register_callbacks(&mut map, dsp, DspCallbacks::new());
    Dsp {
        dsp: dsp,
        can_be_deleted: true
    }
}

pub fn from_ptr_with_description(dsp: *mut ffi::FMOD_DSP, description: &DspDescription, is_first: bool) -> Dsp {
    let mut map = get_callbacks();
    register_callbacks(&mut map, dsp, DspCallbacks {
        create_callback: description.create,
        release_callback: description.release,
        reset_callback: description.reset,
        read_callback: description.read,
        set_pos_callback: description.set_position,
        set_param_callback: description.set_parameter,
        get_param_callback: description.get_parameter
    });
    Dsp {
        dsp: dsp,
        can_be_deleted: is_first
    }
}

pub fn get_ffi(dsp: &Dsp) -> *mut ffi::FMOD_DSP {
    dsp.dsp
}

pub struct Dsp {
    dsp: *mut ffi::FMOD_DSP,
    can_be_deleted: bool
}

impl Drop for Dsp {
    fn drop(&mut self) {
        self.release();
    }
}

impl Dsp {
    pub fn get_system_object(&self) -> Result<FmodSys, fmod::Result> {
        let mut system = ::std::ptr::mut_null();

        match unsafe { ffi::FMOD_DSP_GetSystemObject(self.dsp, &mut system) } {
            fmod::Ok => Ok(fmod_sys::from_ptr(system)),
            e => Err(e)
        }
    }

    pub fn release(&mut self) -> fmod::Result {
        if self.can_be_deleted && self.dsp.is_not_null() {
            match unsafe { ffi::FMOD_DSP_Release(self.dsp) } {
                fmod::Ok => {
                    let mut map = get_callbacks();

                    map.pop_equiv(&(self.dsp as *const ffi::FMOD_DSP));
                    remove_callback(map);
                    self.dsp =::std::ptr::mut_null();
                    fmod::Ok
                }
                e => e
            }
        } else {
            fmod::Ok
        }
    }

    pub fn play(&self) -> Result<channel::Channel, fmod::Result> {
        let mut channel = ::std::ptr::mut_null();

        match match self.get_system_object() {
            Ok(s) => { 
                unsafe { ffi::FMOD_System_PlayDSP(fmod_sys::get_ffi(&s), fmod::ChannelFree, self.dsp, 0, &mut channel) }
            }
            Err(e) => e
        } {
            fmod::Ok => Ok(channel::from_ptr(channel)),
            e => Err(e)
        }
    }

    pub fn play_with_parameters(&self, channel_id: fmod::ChannelIndex) -> Result<channel::Channel, fmod::Result> {
        let mut channel = ::std::ptr::mut_null();
        
        match match self.get_system_object() {
            Ok(s) => { 
                unsafe { ffi::FMOD_System_PlayDSP(fmod_sys::get_ffi(&s), channel_id, self.dsp, 0, &mut channel) }
            }
            Err(e) => e
        } {
            fmod::Ok => Ok(channel::from_ptr(channel)),
            e => Err(e)
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
            1i32
        } else {
            0i32
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

    pub fn get_type(&self) -> Result<fmod::DspType, fmod::Result> {
        let mut _type = fmod::DspTypeUnknown;

        match unsafe { ffi::FMOD_DSP_GetType(self.dsp, &mut _type) } {
            fmod::Ok => Ok(_type),
            e => Err(e)
        }
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