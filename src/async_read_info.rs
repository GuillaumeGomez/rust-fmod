/*
* Rust-FMOD - Copyright (c) 2016 Gomez Guillaume.
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

use FmodFile;
use ffi::FMOD_ASYNCREADINFO;

use libc;

pub struct AsyncReadInfo {
    /// [r] The file handle that was filled out in the open callback.
    pub handle     : FmodFile,
    /// [r] Seek position, make sure you read from this file offset.
    pub offset     : u32,
    /// [r] how many bytes requested for read.
    pub sizebytes  : u32,
    /// [r] 0 = low importance. 100 = extremely important (ie 'must read now or stuttering may
    /// occur')
    pub priority   : u32,
    /// [w] Buffer to read file data into.
    pub buffer     : Vec<u8>,
    /// [r/w] Result code, Ok tells the system it is ready to consume the data. Set this last!
    ///
    /// Default value = FMOD_ERR_NOTREADY.
    pub result     : ::Status,
    #[doc(hidden)]
    ffi: *mut ffi::FMOD_ASYNCREADINFO,
}

impl AsyncReadInfo {
    #[doc(hidden)]
    pub fn new(a: *mut ffi::FMOD_ASYNCREADINFO) -> AsyncReadInfo {
        unsafe {
            AsyncReadInfo {
                handle: file::from_ffi((*a).handle),
                offset: (*a).offset,
                sizebytes: (*a).sizebytes,
                priority: (*a).priority,
                buffer: Vec::new(),
                result: (*a).result,
            }
        }
    }

    #[doc(hidden)]
    pub fn to_ffi(self) -> *mut ffi::FMOD_ASYNCREADINFO {
        let mut out = self.ffi;

        unsafe {
            (*out).offset = self.offset;
            (*out).sizebytes = self.sizebytes;
            (*out).priority = self.priority;
            (*out).buffer = ::libc::malloc(self.buffer.len());
            for (pos, i) in self.buffer.iter().enumerate() {
                *(out.offset(pos)) = i;
            }
            (*out).bytesread = self.buffer.len();
            (*out).result = self.result;
        }
        out
    }
}
