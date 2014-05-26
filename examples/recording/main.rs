#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::*;
use rfmod::types::*;
use rfmod::*;
use std::io::timer::sleep;
use std::mem;

fn get_key() -> u8 {
    let mut reader = std::io::stdio::stdin();
    
    println!("\nPress a corresponding number or ESC to quit");
    print!("> ");

    match reader.read_byte() {
        Ok(nb) => nb,
        Err(_) => 0u8
    }
    //::std::from_str<uint>(nb)
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
        match match get_key() as char {
            '1' => Some(fmod.set_output(fmod::OutputTypeOSS)),
            '2' => Some(fmod.set_output(fmod::OutputTypeALSA)),
            '3' => Some(fmod.set_output(fmod::OutputTypeESD)),
            '4' => Some(fmod.set_output(fmod::OutputTypePulseAudio)),
            c if c == 27u8 as char => return,
            _ => None
        } {
            Some(fmod::Ok) => {
                done = true
            },
            Some(e) => fail!("Output type error: {}", e),
            _ => {}
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

    let tmp_c = if num_drivers > 9 {
            9u8 + 48u8
        } else {
            num_drivers as u8 + 48u8
        };

    loop {
        match get_key() {
            27u8 => return,
            key if key < tmp_c => {
                fmod.set_driver((key - 48) as i32);
                break;
            }
            _ => print!("\nPlease press a corresponding number or ESC to quit\n> "),
        }
    }

    match fmod.init() {
        fmod::Ok => {}
        e => {
            fail!("FmodSys.init failed : {}", e);
        }
    };

    let mut exinfo = FmodCreateSoundexInfo::new();
    let secs = 5;

    //exinfo.cbsize           = mem::size_of::<FMOD_CREATESOUNDEXINFO>() as i32;
    exinfo.numchannels      = 2;
    exinfo.format           = fmod::SoundFormatPCM16;
    exinfo.defaultfrequency = 44100;
    exinfo.length           = (exinfo.defaultfrequency * mem::size_of::<i16>() as i32 * exinfo.numchannels * secs) as u32;

    let sound = match fmod.create_sound(String::new(), Some(FmodMode(FMOD_2D | FMOD_SOFTWARE | FMOD_OPENUSER)), Some(exinfo)) {
        Ok(s) => s,
        Err(e) => fail!("create sound error: {}", e)
    };

    println!("\n=========================");
    println!("=== Recording example ===");
    println!("=========================\n");
    println!("Press '0' to record a {} second segment of audio.", secs);
    println!("Press '1' to play the {} second segment of audio.", secs);

    let record_driver = 0;
    
    loop {
        match match get_key() as char {
            '0' => {
                match fmod.start_record(record_driver, &sound, false) {
                    fmod::Ok => {
                        while fmod.is_recording(record_driver).unwrap() == true {
                            print!("\rRecording : {}", fmod.get_record_position(record_driver).unwrap());
                            fmod.update();
                            sleep(15);
                        }
                        Some(fmod::Ok)
                    }
                    e => Some(e)
                }
            },
            '1' => {
                match sound.play_with_parameters(fmod::ChannelReUse) {
                    Ok(chan) => {
                        fmod.update();
                        while chan.is_playing().unwrap() == true {
                            print!("\rPlaying : {} / {}", chan.get_position(FMOD_TIMEUNIT_MS).unwrap(), sound.get_length(FMOD_TIMEUNIT_MS).unwrap());
                            fmod.update();
                            sleep(15);
                        }
                        Some(fmod::Ok)
                    }
                    Err(e) => Some(e)
                }
            }
            c if c == 27u8 as char => break,
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
}