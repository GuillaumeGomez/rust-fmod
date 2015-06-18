/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FmodEx library, is provided by FIRELIGHT TECHNOLOGIES.
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
use std::default::Default;

pub fn from_ptr(vec: ffi::FMOD_VECTOR) -> Vector {
    Vector {x: vec.x, y: vec.y, z: vec.z}
}

pub fn get_ffi(vec: &Vector) -> ffi::FMOD_VECTOR {
    ffi::FMOD_VECTOR {x: vec.x, y: vec.y, z: vec.z}
}

#[derive(Debug, Clone, Copy)]
/// Structure describing a point in 3D space.
pub struct Vector
{
    /// X co-ordinate in 3D space.
    pub x: f32,
    /// Y co-ordinate in 3D space.
    pub y: f32,
    /// Z co-ordinate in 3D space.
    pub z: f32
}

impl Default for Vector {
    fn default() -> Vector {
        Vector::new()
    }
}

impl Vector {
    pub fn new() -> Vector {
        Vector{x: 0f32, y: 0f32, z: 0f32}
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &Vector) -> bool {
        !self.eq(other)
    }
}