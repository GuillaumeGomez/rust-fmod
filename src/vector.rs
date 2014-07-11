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

pub fn from_ptr(vec: ffi::FMOD_VECTOR) -> FmodVector {
    FmodVector{x: vec.x, y: vec.y, z: vec.z}
}

pub fn get_ffi(vec: &FmodVector) -> ffi::FMOD_VECTOR {
    ffi::FMOD_VECTOR{x: vec.x, y: vec.y, z: vec.z}
}

pub fn new() -> FmodVector {
    FmodVector{x: 0f32, y: 0f32, z: 0f32}
}

#[deriving(Show)]
pub struct FmodVector
{
    pub x: f32, /* X co-ordinate in 3D space. */
    pub y: f32, /* Y co-ordinate in 3D space. */
    pub z: f32  /* Z co-ordinate in 3D space. */
}

impl FmodVector {
    pub fn new() -> FmodVector {
        FmodVector{x: 0f32, y: 0f32, z: 0f32}
    }
}

impl PartialEq for FmodVector {
    fn eq(&self, other: &FmodVector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    fn ne(&self, other: &FmodVector) -> bool {
        !self.eq(other)
    }
}