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

use libc::{fopen, fread, fclose, fseek, ftell};
use libc::{SEEK_SET, SEEK_CUR, SEEK_END};
use libc::FILE;
use libc::stat;
use std::mem::zeroed;
use libc::fstat;
use libc::fileno;
use libc::{c_void, c_char};

pub fn get_ffi(file: &FmodFile) -> *mut FILE {
    file.fd
}

pub fn from_ffi(fd: *mut FILE) -> FmodFile {
    FmodFile {
        fd: fd
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum SeekStyle {
    /// Seek from the beginning of the stream
    SeekSet,
    /// Seek from the end of the stream
    SeekEnd,
    /// Seek from the current position
    SeekCur
}

/// A little struct to wrap C files. I'll try to improve this or to replace it by File
pub struct FmodFile {
    fd: *mut FILE
}

impl FmodFile {
    pub fn open(file_name: &str) -> Option<FmodFile> {
        unsafe {
            let tmp = fopen(file_name.as_ptr() as *const c_char, "rb".as_ptr() as *const c_char);

            if tmp.is_null() {
                None
            } else {
                Some(FmodFile{fd: tmp})
            }
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> usize {
        unsafe {
            if self.fd.is_null() {
                0usize
            } else {
                fread(buffer.as_mut_ptr() as *mut c_void, buffer.len() as usize, 1usize, self.fd) as usize
            }
        }
    }

    pub fn seek(&self, pos: i64, style: self::SeekStyle) -> usize {
        unsafe {
            if self.fd.is_null() {
                0usize
            } else {
                fseek(self.fd, pos, match style {
                    self::SeekStyle::SeekSet => SEEK_SET,
                    self::SeekStyle::SeekEnd => SEEK_END,
                    self::SeekStyle::SeekCur => SEEK_CUR
                }) as usize
            }
        }
    }

    pub fn get_file_size(&self) -> i64 {
        unsafe {
            if self.fd.is_null() {
                0i64
            } else {
                let mut tmp : stat = zeroed::<stat>();
                let id = fileno(self.fd);
                match fstat(id, &mut tmp) {
                    0 => tmp.st_size,
                    _ => 0i64
                }
            }
        }
    }

    pub fn tell(&self) -> i64 {
        unsafe {
            if self.fd.is_null() {
                0i64
            } else {
                ftell(self.fd)
            }
        }
    }

    pub fn close(&mut self) {
        unsafe {
            if !self.fd.is_null() {
                fclose(self.fd);
                self.fd = ::std::ptr::null_mut();
            }
        }
    }
}
