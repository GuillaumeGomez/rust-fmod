#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::{FMOD_OK, FMOD_RESULT, FMOD_TIMEUNIT_MS};
use rfmod::*;
use std::os;
use std::io::timer::sleep;

fn play_to_the_end(sound: Sound, len: uint) -> FMOD_RESULT {
	let length = sound.get_length(FMOD_TIMEUNIT_MS).unwrap();
	let name = sound.get_name(len as u32).unwrap();
	let mut old_position = 100u;

    match sound.play() {
        FMOD_OK => {
            loop {
                match sound.is_playing() {
                    Ok(b) => {
                        if b == true {
                        	let position = sound.get_channel().get_position(FMOD_TIMEUNIT_MS).unwrap();

                        	if position != old_position {
                        		old_position = position;
                        		print!("\r{} : {:02u}:{:02u} / {:02u}:{:02u}", name, position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60);
                        	}
                            sleep(30)
                        } else {
                            break;
                        }
                    },
                    Err(e) => return e,
                }
            }
            FMOD_OK
        }
        err => err,
    }
}

fn main() {
	let args = os::args();
	let tmp = args.tail();

	if tmp.len() < 1 {
		fail!("USAGE: ./fmod [music_file]");
	}
	let fmod = match FmodSys::new() {
		Ok(f) => f,
		Err(e) => {
			fail!("FmodSys.new : {}", e);
		}
	};

	match fmod.init() {
		FMOD_OK => {}
		e => {
			fmod.release();
			fail!("FmodSys.init failed : {}", e);
		}
	};

	let arg1 = tmp.get(0).unwrap();

	let mut sound = match fmod.create_sound(StrBuf::from_str(*arg1), None, None) {
		Ok(s) => s,
		Err(err) => {fail!("FmodSys.create_sound failed : {}", err);},
	};

	match play_to_the_end(sound, arg1.len()) {
		FMOD_OK => {println!("Ok !");},
		err => {fail!("FmodSys.play_to_the_end : {}", err);},
	};

	sound.release();
	fmod.release();
}