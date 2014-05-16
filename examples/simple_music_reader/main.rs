#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::FMOD_OK;
use rfmod::*;
use std::os;

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

	let mut sound = match fmod.create_sound(StrBuf::from_str(*arg1), None) {
		Ok(s) => s,
		Err(err) => {fail!("FmodSys.create_sound failed : {}", err);},
	};

	match sound.play_to_the_end() {
		FMOD_OK => {println!("Ok !");},
		err => {fail!("FmodSys.play_to_the_end : {}", err);},
	};

	sound.release();
	fmod.release();
}