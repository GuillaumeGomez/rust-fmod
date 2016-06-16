/*
* Rust-FMOD - Copyright (c) 2016 Gomez Guillaume.
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
use vector;
use libc::{c_int, c_void};
use fmod_sys;
use fmod_sys::MemoryUsageDetails;
use std::mem::transmute;
use std::default::Default;

/// Geometry object
pub struct Geometry {
    geometry: *mut ffi::FMOD_GEOMETRY
}

impl ffi::FFI<ffi::FMOD_GEOMETRY> for Geometry {
    fn wrap(g: *mut ffi::FMOD_GEOMETRY) -> Geometry {
        Geometry {geometry: g}
    }

    fn unwrap(g: &Geometry) -> *mut ffi::FMOD_GEOMETRY {
        g.geometry
    }
}

impl Drop for Geometry {
    fn drop(&mut self) {
        self.release();
    }
}

impl Geometry {
    pub fn release(&mut self) -> ::Status {
        if self.geometry !=::std::ptr::null_mut() {
            match unsafe { ffi::FMOD_Geometry_Release(self.geometry) } {
                ::Status::Ok => {
                    self.geometry = ::std::ptr::null_mut();
                   ::Status::Ok
                }
                e => e,
            }
        } else {
            ::Status::Ok
        }
    }

    pub fn add_polygon(&self, direct_occlusion: f32, reverb_occlusion: f32, double_sided: bool,
                       vertices: Vec<vector::Vector>) -> Result<i32, ::Status> {
        let t_double_sided = if double_sided == true {
            1
        } else {
            0
        };
        let mut index = 0i32;
        let t_vertices = Vec::with_capacity(vertices.len());

        for tmp in vertices.iter() {
            vector::get_ffi(tmp);
        }

        match unsafe { ffi::FMOD_Geometry_AddPolygon(self.geometry, direct_occlusion,
                                                     reverb_occlusion, t_double_sided,
                                                     vertices.len() as c_int, t_vertices.as_ptr(),
                                                     &mut index) } {
            ::Status::Ok => Ok(index),
            e => Err(e),
        }
    }

    pub fn get_num_polygons(&self) -> Result<i32, ::Status> {
        let mut num = 0i32;

        match unsafe { ffi::FMOD_Geometry_GetNumPolygons(self.geometry, &mut num) } {
            ::Status::Ok => Ok(num),
            e => Err(e)
        }
    }

    pub fn get_max_polygons(&self) -> Result<(i32, i32), ::Status> {
        let mut max_polygons = 0i32;
        let mut max_vertices = 0i32;

        match unsafe { ffi::FMOD_Geometry_GetMaxPolygons(self.geometry, &mut max_polygons,
                                                         &mut max_vertices) } {
            ::Status::Ok => Ok((max_polygons, max_vertices)),
            e => Err(e),
        }
    }

    pub fn get_polygon_num_vertices(&self, index: i32) -> Result<i32, ::Status> {
        let mut num = 0i32;

        match unsafe { ffi::FMOD_Geometry_GetPolygonNumVertices(self.geometry, index, &mut num) } {
            ::Status::Ok => Ok(num),
            e => Err(e),
        }
    }

    pub fn set_polygon_vertex(&self, index: i32, vertex_index: i32,
                              vertex: vector::Vector) -> ::Status {
        let t_vertex = vector::get_ffi(&vertex);

        unsafe { ffi::FMOD_Geometry_SetPolygonVertex(self.geometry, index, vertex_index,
                                                     &t_vertex) }
    }

    pub fn get_polygon_vertex(&self, index: i32,
                              vertex_index: i32) -> Result<vector::Vector, ::Status> {
        let mut vertex = vector::get_ffi(&vector::Vector::new());

        match unsafe { ffi::FMOD_Geometry_GetPolygonVertex(self.geometry, index, vertex_index,
                                                           &mut vertex) } {
            ::Status::Ok => Ok(vector::from_ptr(vertex)),
            e => Err(e),
        }
    }

    pub fn set_polygon_attributes(&self, index: i32, direct_occlusion: f32, reverb_occlusion: f32,
                                  double_sided: bool) -> ::Status {
        let t_double_sided = if double_sided == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_Geometry_SetPolygonAttributes(self.geometry, index, direct_occlusion,
                                                         reverb_occlusion, t_double_sided) }
    }

    /// Returns:
    ///
    /// Ok(direct_occlusion, reverb_occlusion, double_sided)
    pub fn get_polygon_attributes(&self, index: i32) -> Result<(f32, f32, bool), ::Status> {
        let mut direct_occlusion = 0f32;
        let mut reverb_occlusion = 0f32;
        let mut double_sided = 0;

        match unsafe { ffi::FMOD_Geometry_GetPolygonAttributes(self.geometry, index,
                                                               &mut direct_occlusion,
                                                               &mut reverb_occlusion,
                                                               &mut double_sided) } {
            ::Status::Ok => Ok((direct_occlusion, reverb_occlusion, double_sided == 1)),
            e => Err(e),
        }
    }

    pub fn set_active(&self, active: bool) -> ::Status {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_Geometry_SetActive(self.geometry, t_active) }
    }

    pub fn get_active(&self) -> Result<bool, ::Status> {
        let mut active = 0;

        match unsafe { ffi::FMOD_Geometry_GetActive(self.geometry, &mut active) } {
            ::Status::Ok => Ok(active == 1),
            e => Err(e)
        }
    }

    pub fn set_rotation(&self, forward: vector::Vector, up: vector::Vector) -> ::Status {
        let t_forward = vector::get_ffi(&forward);
        let t_up = vector::get_ffi(&up);

        unsafe { ffi::FMOD_Geometry_SetRotation(self.geometry, &t_forward, &t_up) }
    }

    /// Returns:
    ///
    /// Ok(forward, up)
    pub fn get_rotation(&self) -> Result<(vector::Vector, vector::Vector), ::Status> {
        let mut forward = vector::get_ffi(&vector::Vector::new());
        let mut up = vector::get_ffi(&vector::Vector::new());

        match unsafe { ffi::FMOD_Geometry_GetRotation(self.geometry, &mut forward, &mut up) } {
            ::Status::Ok => Ok((vector::from_ptr(forward), vector::from_ptr(up))),
            e => Err(e)
        }
    }

    pub fn set_position(&self, position: vector::Vector) -> ::Status {
        let t_position = vector::get_ffi(&position);

        unsafe { ffi::FMOD_Geometry_SetPosition(self.geometry, &t_position) }
    }

    pub fn get_position(&self) -> Result<vector::Vector, ::Status> {
        let mut position = vector::get_ffi(&vector::Vector::new());

        match unsafe { ffi::FMOD_Geometry_GetPosition(self.geometry, &mut position) } {
            ::Status::Ok => Ok(vector::from_ptr(position)),
            e => Err(e)
        }
    }

    pub fn set_scale(&self, scale: vector::Vector) -> ::Status {
        let t_scale = vector::get_ffi(&scale);

        unsafe { ffi::FMOD_Geometry_SetScale(self.geometry, &t_scale) }
    }

    pub fn get_scale(&self) -> Result<vector::Vector, ::Status> {
        let mut scale = vector::get_ffi(&vector::Vector::new());

        match unsafe { ffi::FMOD_Geometry_GetScale(self.geometry, &mut scale) } {
            ::Status::Ok => Ok(vector::from_ptr(scale)),
            e => Err(e)
        }
    }

    /// Returns:
    ///
    /// Ok(memory_used, details)
    pub fn get_memory_info(&self, MemoryBits(memory_bits): MemoryBits,
                           EventMemoryBits(event_memory_bits): EventMemoryBits)
                           -> Result<(u32, MemoryUsageDetails), ::Status> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(Default::default());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_Geometry_GetMemoryInfo(self.geometry, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            ::Status::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    pub fn set_user_data<'r, T>(&'r self, user_data: &'r mut T) -> ::Status {
        unsafe { ffi::FMOD_Geometry_SetUserData(self.geometry, transmute(user_data)) }
    }

    pub fn get_user_data<'r, T>(&'r self) -> Result<&'r mut T, ::Status> {
        unsafe {
            let mut user_data : *mut c_void = ::std::ptr::null_mut();

            match ffi::FMOD_Geometry_GetUserData(self.geometry, &mut user_data) {
                ::Status::Ok => {
                    let tmp : &mut T = transmute::<*mut c_void, &mut T>(user_data);
                    
                    Ok(tmp)
                },
                e => Err(e)
            }
        }
    }
}
