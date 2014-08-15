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

#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::*;
use rfmod::types::*;
use rfmod::*;
use std::io::timer::sleep;
use std::mem;
use std::default::Default;
use std::time::duration::Duration;

fn get_key() -> Result<int, std::io::IoError> {
    let mut reader = std::io::stdio::stdin();
    
    println!("\nEnter a corresponding number or \"ESC\" to quit:");
    print!("> ");

    match reader.read_line() {
        Ok(mut line) => {
            let length = line.len() - 1;
            line.truncate(length);
            if line.as_slice() == "ESC" {
                Ok(-1)
            } else {
                Ok(from_str(line.as_slice()).unwrap())
            }
        }
        Err(e) => Err(e)
    }
}

fn main() {
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            fail!("FmodSys.new : {}", e);
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
                    1 => Some(fmod.set_output(fmod::OutputTypeOSS)),
                    2 => Some(fmod.set_output(fmod::OutputTypeALSA)),
                    3 => Some(fmod.set_output(fmod::OutputTypeESD)),
                    4 => Some(fmod.set_output(fmod::OutputTypePulseAudio)),
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
            Err(e) => fail!("Input type error: {}", e),
        }
    }

    let num_drivers = match fmod.get_num_drivers().unwrap() as uint {
        0 => fail!("No driver available for this output type"),
        val => val
    };
    let mut it = 0i32;

    println!("\n\n--------------------------------");
    println!("--- CHOOSE A PLAYBACK DRIVER ---");
    println!("--------------------------------");
    while it < num_drivers as i32 {
        //check this function
        let t = match fmod.get_driver_info(it, 256u) {
            Ok((_, name)) => name,
            Err(e) => fail!("get_driver_info error: {}", e)
        };
        println!("{} : {}", it, t);
        it = it + 1;
    }

    loop {
        match get_key() {
            Ok(nb) => {
                match nb {
                    -1 => return,
                    nb if nb < num_drivers as int => {
                        fmod.set_driver(nb as i32);
                        break;
                    }
                    _ => {
                        print!("\nPlease press a corresponding number or ESC to quit\n> ");
                    }
                }
            }
            Err(e) => {
                fail!("Input type error: {}", e);
            }
        }
    }

    match fmod.init() {
        fmod::Ok => {}
        e => {
            fail!("FmodSys.init failed : {}", e);
        }
    };

    let mut exinfo : FmodCreateSoundexInfo = Default::default();
    let secs = 5i32;

    exinfo.num_channels      = 2;
    exinfo.format            = fmod::SoundFormatPCM16;
    exinfo.default_frequency = 44100;
    exinfo.length            = (exinfo.default_frequency * mem::size_of::<i16>() as i32 * exinfo.num_channels * secs) as u32;

    let sound = match fmod.create_sound("", Some(FmodMode(FMOD_2D | FMOD_SOFTWARE | FMOD_OPENUSER)), Some(&mut exinfo)) {
        Ok(s) => s,
        Err(e) => fail!("create sound error: {}", e)
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
                            fmod::Ok => {
                                while fmod.is_recording(record_driver).unwrap() == true {
                                    print!("\rRecording : {}", fmod.get_record_position(record_driver).unwrap());
                                    fmod.update();
                                    sleep(Duration::milliseconds(15));
                                }
                                Some(fmod::Ok)
                            }
                            e => Some(e)
                        }
                    },
                    1 => {
                        match sound.play() {
                            Ok(chan) => {
                                fmod.update();
                                while chan.is_playing().unwrap() == true {
                                    print!("\rPlaying : {} / {}", chan.get_position(FMOD_TIMEUNIT_MS).unwrap(), sound.get_length(FMOD_TIMEUNIT_MS).unwrap());
                                    fmod.update();
                                    sleep(Duration::milliseconds(15));
                                }
                                Some(fmod::Ok)
                            }
                            Err(e) => Some(e)
                        }
                    },
                    2 => {
                        print!("Please enter the output file name : ");
                        let mut reader = std::io::stdio::stdin();

                        match reader.read_line() {
                            Ok(mut name) => {
                                name.pop_char().unwrap();
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
                    Some(r) if r == fmod::Ok => {}
                    Some(e) => {
                        println!("Error : {}", e);
                        break;
                    }
                    None => {}
                }
            }
            Err(e) => {
                fail!("Input type error: {}", e)
            }
        }
    }
}