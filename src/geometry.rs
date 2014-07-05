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
use libc::{c_int};
use fmod_sys;
use fmod_sys::FmodMemoryUsageDetails;
use std::mem::transmute;

pub fn from_ptr(geometry: *mut ffi::FMOD_GEOMETRY) -> Geometry {
    Geometry{geometry: geometry}
}

pub fn get_ffi(geometry: Geometry) -> *mut ffi::FMOD_GEOMETRY {
    geometry.geometry
}

pub struct Geometry {
    geometry: *mut ffi::FMOD_GEOMETRY
}

impl Drop for Geometry {
    fn drop(&mut self) {
        self.release();
    }
}

impl Geometry {
    pub fn release(&mut self) -> fmod::Result {
        if self.geometry !=::std::ptr::mut_null() {
            match unsafe { ffi::FMOD_Geometry_Release(self.geometry) } {
                fmod::Ok => {
                    self.geometry =::std::ptr::mut_null();
                    fmod::Ok
                }
                e => e
            }
        } else {
            fmod::Ok
        }
    }

    pub fn add_polygon(&self, direct_occlusion: f32, reverb_occlusion: f32, double_sided: bool, vertices: Vec<vector::FmodVector>) -> Result<i32, fmod::Result> {
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

        match unsafe { ffi::FMOD_Geometry_AddPolygon(self.geometry, direct_occlusion, reverb_occlusion, t_double_sided, vertices.len() as c_int,
            t_vertices.as_ptr(), &mut index) } {
            fmod::Ok => Ok(index),
            e => Err(e)
        }
    }

    pub fn get_num_polygons(&self) -> Result<i32, fmod::Result> {
        let mut num = 0i32;

        match unsafe { ffi::FMOD_Geometry_GetNumPolygons(self.geometry, &mut num) } {
            fmod::Ok => Ok(num),
            e => Err(e)
        }
    }

    pub fn get_max_polygons(&self) -> Result<(i32, i32), fmod::Result> {
        let mut max_polygons = 0i32;
        let mut max_vertices = 0i32;

        match unsafe { ffi::FMOD_Geometry_GetMaxPolygons(self.geometry, &mut max_polygons, &mut max_vertices) } {
            fmod::Ok => Ok((max_polygons, max_vertices)),
            e => Err(e)
        }
    }

    pub fn get_polygon_num_vertices(&self, index: i32) -> Result<i32, fmod::Result> {
        let mut num = 0i32;

        match unsafe { ffi::FMOD_Geometry_GetPolygonNumVertices(self.geometry, index, &mut num) } {
            fmod::Ok => Ok(num),
            e => Err(e)
        }
    }

    pub fn set_polygon_vertex(&self, index: i32, vertex_index: i32, vertex: vector::FmodVector) -> fmod::Result {
        let t_vertex = vector::get_ffi(&vertex);

        unsafe { ffi::FMOD_Geometry_SetPolygonVertex(self.geometry, index, vertex_index, &t_vertex) }
    }

    pub fn get_polygon_vertex(&self, index: i32, vertex_index: i32) -> Result<vector::FmodVector, fmod::Result> {
        let mut vertex = vector::get_ffi(&vector::new());

        match unsafe { ffi::FMOD_Geometry_GetPolygonVertex(self.geometry, index, vertex_index, &mut vertex) } {
            fmod::Ok => Ok(vector::from_ptr(vertex)),
            e => Err(e)
        }
    }

    pub fn set_polygon_attributes(&self, index: i32, direct_occlusion: f32, reverb_occlusion: f32, double_sided: bool) -> fmod::Result {
        let t_double_sided = if double_sided == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_Geometry_SetPolygonAttributes(self.geometry, index, direct_occlusion, reverb_occlusion, t_double_sided) }
    }

    pub fn get_polygon_attributes(&self, index: i32) -> Result<(f32, f32, bool), fmod::Result> {
        let mut direct_occlusion = 0f32;
        let mut reverb_occlusion = 0f32;
        let mut double_sided = 0;

        match unsafe { ffi::FMOD_Geometry_GetPolygonAttributes(self.geometry, index, &mut direct_occlusion, &mut reverb_occlusion, &mut double_sided) } {
            fmod::Ok => Ok((direct_occlusion, reverb_occlusion, double_sided == 1)),
            e => Err(e)
        }
    }

    pub fn set_active(&self, active: bool) -> fmod::Result {
        let t_active = if active == true {
            1
        } else {
            0
        };

        unsafe { ffi::FMOD_Geometry_SetActive(self.geometry, t_active) }
    }

    pub fn get_active(&self) -> Result<bool, fmod::Result> {
        let mut active = 0;

        match unsafe { ffi::FMOD_Geometry_GetActive(self.geometry, &mut active) } {
            fmod::Ok => Ok(active == 1),
            e => Err(e)
        }
    }

    pub fn set_rotation(&self, forward: vector::FmodVector, up: vector::FmodVector) -> fmod::Result {
        let t_forward = vector::get_ffi(&forward);
        let t_up = vector::get_ffi(&up);

        unsafe { ffi::FMOD_Geometry_SetRotation(self.geometry, &t_forward, &t_up) }
    }

    pub fn get_rotation(&self) -> Result<(vector::FmodVector, vector::FmodVector), fmod::Result> {
        let mut forward = vector::get_ffi(&vector::new());
        let mut up = vector::get_ffi(&vector::new());

        match unsafe { ffi::FMOD_Geometry_GetRotation(self.geometry, &mut forward, &mut up) } {
            fmod::Ok => Ok((vector::from_ptr(forward), vector::from_ptr(up))),
            e => Err(e)
        }
    }

    pub fn set_position(&self, position: vector::FmodVector) -> fmod::Result {
        let t_position = vector::get_ffi(&position);

        unsafe { ffi::FMOD_Geometry_SetPosition(self.geometry, &t_position) }
    }

    pub fn get_position(&self) -> Result<vector::FmodVector, fmod::Result> {
        let mut position = vector::get_ffi(&vector::new());

        match unsafe { ffi::FMOD_Geometry_GetPosition(self.geometry, &mut position) } {
            fmod::Ok => Ok(vector::from_ptr(position)),
            e => Err(e)
        }
    }

    pub fn set_scale(&self, scale: vector::FmodVector) -> fmod::Result {
        let t_scale = vector::get_ffi(&scale);

        unsafe { ffi::FMOD_Geometry_SetScale(self.geometry, &t_scale) }
    }

    pub fn get_scale(&self) -> Result<vector::FmodVector, fmod::Result> {
        let mut scale = vector::get_ffi(&vector::new());

        match unsafe { ffi::FMOD_Geometry_GetScale(self.geometry, &mut scale) } {
            fmod::Ok => Ok(vector::from_ptr(scale)),
            e => Err(e)
        }
    }

    pub fn get_memory_info(&self, FmodMemoryBits(memory_bits): FmodMemoryBits,
        FmodEventMemoryBits(event_memory_bits): FmodEventMemoryBits) -> Result<(u32, FmodMemoryUsageDetails), fmod::Result> {
        let mut details = fmod_sys::get_memory_usage_details_ffi(FmodMemoryUsageDetails::new());
        let mut memory_used = 0u32;

        match unsafe { ffi::FMOD_Geometry_GetMemoryInfo(self.geometry, memory_bits, event_memory_bits, &mut memory_used, &mut details) } {
            fmod::Ok => Ok((memory_used, fmod_sys::from_memory_usage_details_ptr(details))),
            e => Err(e)
        }
    }

    /* to test ! */
    /*pub fn set_user_data<T>(&self, user_data: T) -> fmod::Result {
        unsafe { ffi::FMOD_Geometry_SetUserData(self.geometry, transmute(user_data)) }
    }*/

    /* to test ! */
    /*pub fn get_user_data<T>(&self) -> Result<T, fmod::Result> {
        unsafe {
            let user_data =::std::ptr::null();

            match ffi::FMOD_Geometry_GetUserData(self.geometry, &user_data) {
                fmod::Ok => Ok(transmute(user_data)),
                e => Err(e)
            }
        }
    }*/
}