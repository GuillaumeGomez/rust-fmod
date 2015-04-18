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

use std::thread::sleep_ms;
use std::mem;
use std::default::Default;
use std::io::{self, Error, BufRead};

fn get_key() -> Result<isize, Error> {
    let r = io::stdin();
    let mut reader = r.lock();
    let mut line = String::new();

    println!("\nEnter a corresponding number or \"ESC\" to quit:");
    print!("> ");
    match reader.read_line(&mut line) {
        Ok(_) => {
            let length = line.len() - 1;
            line.truncate(length);
            if &line == "ESC" {
                Ok(-1)
            } else {
                Ok(line.parse().unwrap())
            }
        }
        Err(e) => Err(e)
    }
}

fn main() {
    let fmod = match rfmod::FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("FmodSys.new : {:?}", e);
        }
    };

    println!("--------------------------");
    println!("--- SELECT OUTPUT TYPE ---");
    println!("--------------------------");
    println!("1 :  OSS        - Open Sound System");
    println!("2 :  ALSA       - Advanced Linux Sound Architecture");
    println!("3 :  ESD        - Enlightenment Sound Daemon");
    println!("4 :  PULSEAUDIO - Pulse Audio Sound Server");
    println!("--------------------------");

    let mut done = false;

    while done == false {
        match get_key() {
            Ok(n) => {
                match match n {
                    1 => Some(fmod.set_output(rfmod::OutputType::OSS)),
                    2 => Some(fmod.set_output(rfmod::OutputType::ALSA)),
                    3 => Some(fmod.set_output(rfmod::OutputType::ESD)),
                    4 => Some(fmod.set_output(rfmod::OutputType::PulseAudio)),
                    -1 => {
                        return;
                    }
                    _ => None
                } {
                    Some(_) => {
                        done = true;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("Input type error: {}", e),
        }
    }

    let num_drivers = match match fmod.get_num_drivers() {
        Ok(n) => n as usize,
        Err(e) => panic!("rfmod.get_num_drivers failed: {:?}", e)
    } {
        0 => panic!("No driver available for this output type"),
        val => val
    };
    let mut it = 0i32;

    println!("\n\n--------------------------------");
    println!("--- CHOOSE A PLAYBACK DRIVER ---");
    println!("--------------------------------");
    while it < num_drivers as i32 {
        //check this function
        let t = match fmod.get_driver_info(it, 256usize) {
            Ok((_, name)) => name,
            Err(e) => panic!("get_driver_info error: {:?}", e)
        };
        println!("{} : {}", it, t);
        it = it + 1;
    }

    loop {
        match get_key() {
            Ok(nb) => {
                match nb {
                    -1 => return,
                    nb if nb < num_drivers as isize => {
                        fmod.set_driver(nb as i32);
                        break;
                    }
                    _ => {
                        print!("\nPlease press a corresponding number or ESC to quit\n> ");
                    }
                }
            }
            Err(e) => {
                panic!("Input type error: {:?}", e);
            }
        }
    }

    match fmod.init() {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.init failed : {:?}", e);
        }
    };

    let mut exinfo : rfmod::FmodCreateSoundexInfo = Default::default();
    let secs = 5i32;

    exinfo.num_channels      = 2;
    exinfo.format            = rfmod::SoundFormat::PCM16;
    exinfo.default_frequency = 44100;
    exinfo.length            = (exinfo.default_frequency * mem::size_of::<i16>() as i32 * exinfo.num_channels * secs) as u32;

    let sound = match fmod.create_sound("", Some(rfmod::FmodMode(rfmod::FMOD_2D | rfmod::FMOD_SOFTWARE | rfmod::FMOD_OPENUSER)),
        Some(&mut exinfo)) {
        Ok(s) => s,
        Err(e) => panic!("create sound error: {:?}", e)
    };

    println!("\n=========================");
    println!("=== Recording example ===");
    println!("=========================\n");
    println!("Press '0' to record a {} seconds segment of audio.", secs);
    println!("Press '1' to play the {} seconds segment of audio.", secs);
    println!("Press '2' to save the {} seconds segment of audio into a file.", secs);

    let record_driver = 0;
    
    loop {
        match get_key() {
            Ok(nb) => {
                match match nb {
                    0 => {
                        match fmod.start_record(record_driver, &sound, false) {
                            rfmod::Result::Ok => {
                                while match fmod.is_recording(record_driver) {
                                    Ok(r) => r,
                                    Err(e) => {
                                        println!("Error on rfmod.is_recording: {:?}", e);
                                        false
                                    }
                                } {
                                    print!("\rRecording : {}", match fmod.get_record_position(record_driver) {
                                        Ok(p) => p,
                                        Err(e) => {
                                            println!("rfmod.get_record_position failed: {:?}", e);
                                            return;
                                        }
                                    });
                                    fmod.update();
                                    sleep_ms(15);
                                }
                                Some(rfmod::Result::Ok)
                            }
                            e => Some(e)
                        }
                    },
                    1 => {
                        match sound.play() {
                            Ok(chan) => {
                                fmod.update();
                                while match chan.is_playing() {
                                    Ok(p) => p,
                                    Err(e) => {
                                        println!("channel.is_playing failed: {:?}", e);
                                        false
                                    }
                                } {
                                    print!("\rPlaying : {} / {}", match chan.get_position(rfmod::FMOD_TIMEUNIT_MS) {
                                        Ok(l) => l,
                                        Err(e) => {
                                            println!("channel.get_position failed: {:?}", e);
                                            return;
                                        }
                                    }, match sound.get_length(rfmod::FMOD_TIMEUNIT_MS) {
                                        Ok(l) => l,
                                        Err(e) => {
                                            println!("sound.get_length failed: {:?}", e);
                                            return;
                                        }
                                    });
                                    fmod.update();
                                    sleep_ms(15);
                                }
                                Some(rfmod::Result::Ok)
                            }
                            Err(e) => Some(e)
                        }
                    },
                    2 => {
                        print!("Please enter the output file name : ");
                        let r = io::stdin();
                        let mut reader = r.lock();
                        let mut name = String::new();

                        match reader.read_line(&mut name) {
                            Ok(_) => {
                                name.pop().unwrap();
                                match sound.save_to_wav(&name) {
                                    Ok(b) => if b {
                                        println!("export succeeded");
                                        None
                                    } else {
                                        println!("export failed");
                                        None
                                    },
                                    Err(e) => {
                                        println!("save_to_wav error: {}", e);
                                        None
                                    }
                                }
                            },
                            Err(_) => None
                        }
                    },
                    -1 => break,
                    _ => None
                } {
                    Some(r) if r == rfmod::Result::Ok => {}
                    Some(e) => {
                        println!("Error : {:?}", e);
                        break;
                    }
                    None => {}
                }
            }
            Err(e) => {
                panic!("Input type error: {}", e)
            }
        }
    }
}