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

use libc::funcs::c95::stdio::{fopen, fread, fclose, fseek, ftell};
use libc::consts::os::c95::{SEEK_SET, SEEK_CUR, SEEK_END};
use std::io::{SeekStyle, SeekSet, SeekEnd, SeekCur};
use libc::types::common::c95::FILE;
use libc::types::os::arch::posix01::stat;
use std::mem::zeroed;
use libc::funcs::posix88::stat_::fstat;
use libc::funcs::posix88::stdio::fileno;
use libc::c_void;
use c_str::ToCStr;

pub fn get_ffi(file: &FmodFile) -> *mut FILE {
    file.fd
}

pub fn from_ffi(fd: *mut FILE) -> FmodFile {
    FmodFile {
        fd: fd
    }
}

/// A little struct to wrap C files. I'll try to improve this or to replace it by File
#[derive(Copy)]
pub struct FmodFile {
    fd: *mut FILE
}

impl FmodFile {
    pub fn open(file_name: &str) -> Option<FmodFile> {
        unsafe {
            let tmp = file_name.with_c_str(|c_str| {
                "rb".with_c_str(|c_str2| {
                    fopen(c_str, c_str2)
                })
            });
            if tmp.is_null() {
                None
            } else {
                Some(FmodFile{fd: tmp})
            }
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> uint {
        unsafe {
            if self.fd.is_null() {
                0u
            } else {
                fread(buffer.as_mut_ptr() as *mut c_void, buffer.len() as u64, 1u64, self.fd) as uint
            }
        }
    }

    pub fn seek(&self, pos: i64, style: SeekStyle) -> uint {
        unsafe {
            if self.fd.is_null() {
                0u
            } else {
                fseek(self.fd, pos, match style {
                    SeekSet => SEEK_SET,
                    SeekEnd => SEEK_END,
                    SeekCur => SEEK_CUR
                }) as uint
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