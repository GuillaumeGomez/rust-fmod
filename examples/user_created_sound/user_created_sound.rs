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

#![crate_type = "bin"]

#![feature(libc)]

extern crate libc;
extern crate rfmod;

use std::default::Default;
use std::thread::sleep_ms;
use std::io::{self, BufRead, Error};

#[allow(unused_variables)]
fn pcmreadcallback(sound: &rfmod::Sound, data: &mut [i16]) -> rfmod::Result {
    static mut t1 : f32 = 0f32; // time
    static mut t2 : f32 = 0f32; // time
    static mut v1 : f32 = 0f32; // velocity
    static mut v2 : f32 = 0f32; // velocity
    let mut count = 0usize;

    while count < data.len() {
        unsafe {
            data[count] = (t1.sin() * 32767f32) as i16; // left channel
            count += 1;
            data[count] = (t2.sin() * 32767f32) as i16; // right channel
            count += 1;

            t1 += 0.01f32 + v1;
            t2 += 0.0142f32 + v2;
            v1 += t1.sin() * 0.002f32;
            v2 += t2.sin() * 0.002f32;
        }
    }

    rfmod::Result::Ok
}

fn get_key() -> Result<isize, Error> {
    let r = io::stdin();
    let mut reader = r.lock();
    let mut line = String::new();

    print!("> ");
    match reader.read_line(&mut line) {
        Ok(_) => {
            let length = line.len() - 1;

            line.truncate(length);
            if &line == "Quit" {
                Ok(-1)
            } else {
                Ok(line.parse().unwrap())
            }
        }
        Err(e) => Err(e)
    }
}

fn main() {
    let channels = 2i32;
    let fmod = match rfmod::Sys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Sys::new() : {:?}", e);
        }
    };

    match fmod.init_with_parameters(32i32, rfmod::InitFlag(rfmod::INIT_NORMAL)) {
        rfmod::Result::Ok => {}
        e => {
            panic!("Sys::init() failed : {:?}", e);
        }
    };

    println!("============================================================================");
    println!("============== User Created Sound Example from FMOD examples ===============");
    println!("============================================================================");
    println!("Sound played here is generated in realtime.  It will either play as a stream");
    println!("which means it is continually filled as it is playing, or it will play as a");
    println!("static sample, which means it is filled once as the sound is created, then");
    println!("when played it will just play that short loop of data.");
    println!("============================================================================\n");
    println!("Enter '1' to play as a runtime decoded stream. (will carry on infinitely)");
    println!("Enter '2' to play as a static in memory sample. (loops a short block of data)");
    println!("Enter 'Quit' to quit.");
    

    let mut ret = get_key().unwrap();
    while ret != -1 && ret != 1 && ret != 2 {
        println!("Invalid entry");
        ret = get_key().unwrap();
    }
    let mut exinfo : rfmod::CreateSoundexInfo = Default::default();

    exinfo.decode_buffer_size = 44100;
    exinfo.length = 44100u32 * channels as u32 * std::mem::size_of::<i16>() as u32 * 5u32;
    exinfo.num_channels = channels;
    exinfo.default_frequency  = 44100;
    exinfo.format = rfmod::SoundFormat::PCM16;
    exinfo.pcm_read_callback = Some(pcmreadcallback as fn(&_, &mut _) -> _);

    let sound = match match ret {
        1 => fmod.create_sound("",
            Some(rfmod::Mode(rfmod::_2D | rfmod::OPENUSER | rfmod::HARDWARE | rfmod::LOOP_NORMAL
            | rfmod::CREATESTREAM)), Some(&mut exinfo)),
        2 => fmod.create_sound("",
            Some(rfmod::Mode(rfmod::_2D | rfmod::OPENUSER | rfmod::HARDWARE | rfmod::LOOP_NORMAL)),
            Some(&mut exinfo)),
        _ => return
    } {
        Ok(s) => s,
        Err(e) => panic!("create sound error: {:?}", e)
    };

    let chan = match sound.play() {
        Ok(c) => c,
        Err(e) => panic!("sound.play error: {:?}", e)
    };

    let length = match sound.get_length(rfmod::TIMEUNIT_MS) {
        Ok(l) => l,
        Err(e) => panic!("sound.get_length failed: {:?}", e)
    };
    while match chan.is_playing() {
        Ok(p) => p,
        Err(e) => {
            println!("chan.is_playing failed: {:?}", e);
            false
        }
    } {
        let position = match chan.get_position(rfmod::TIMEUNIT_MS) {
            Ok(p) => p,
            Err(e) => {
                println!("channel.get_position failed: {:?}", e);
                return;
            }
        };

        print!("{:02}:{:02} / {:02}:{:02}\r", position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60);
        sleep_ms(30)
    }
}