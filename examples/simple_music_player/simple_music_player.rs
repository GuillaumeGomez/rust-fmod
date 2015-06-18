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

extern crate rfmod;

use std::thread::sleep_ms;

fn play_to_the_end(sound: rfmod::Sound, len: usize) -> rfmod::Result {
    let length = match sound.get_length(rfmod::TIMEUNIT_MS) {
        Ok(l) => l,
        Err(e) => panic!("sound.get_length error: {:?}", e)
    };
    let name = match sound.get_name(len) {
        Ok(n) => n,
        Err(e) => panic!("sound.get_name error: {:?}", e)
    };
    let mut old_position = 100usize;

    match sound.play() {
        Ok(chan) => {
            loop {
                match chan.is_playing() {
                    Ok(b) => {
                        if b == true {
                            let position = match chan.get_position(rfmod::TIMEUNIT_MS) {
                                Ok(p) => p,
                                Err(e) => {
                                    panic!("channel.get_position failed: {:?}", e)
                                }
                            };

                            if position != old_position {
                                old_position = position;
                                print!("\r{} : {:02}:{:02} / {:02}:{:02}", name, position / 1000 / 60, position / 1000 % 60,
                                    length / 1000 / 60, length / 1000 % 60);
                            }
                            sleep_ms(30)
                        } else {
                            break;
                        }
                    },
                    Err(e) => return e,
                }
            }
            rfmod::Result::Ok
        }
        Err(err) => err,
    }
}

fn main() {
    let t_args = std::env::args();
    let mut args = Vec::new();

    for tmp in t_args {
        args.push(tmp);
    }
    let tmp = args[1..].to_owned();

    if tmp.len() < 1 {
        panic!("USAGE: ./simple_music_player [music_file]");
    }
    let fmod = match rfmod::Sys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Sys::new() : {:?}", e);
        }
    };

    match fmod.init() {
        rfmod::Result::Ok => {}
        e => {
            panic!("Sys::init() failed : {:?}", e);
        }
    };

    let arg1 = tmp.get(0).unwrap();

    let sound = match fmod.create_sound(&(*arg1), None, None) {
        Ok(s) => s,
        Err(err) => {
            panic!("Sys::create_sound() failed : {:?}", err);
        },
    };

    match play_to_the_end(sound, arg1.len()) {
        rfmod::Result::Ok => {
            println!("Ok !");
        },
        err => {
            panic!("Sys::play_to_the_end() : {:?}", err);
        }
    };
}