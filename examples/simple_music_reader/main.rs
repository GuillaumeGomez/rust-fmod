#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::FMOD_OK;
use rfmod::*;
use std::os;

fn main() {
	let args = os::args();
	let tmp = args.tail();
	let fmod = match Fmod::new() {
		Ok(f) => f,
		Err(e) => {
			fail!("Error code : {}", e);
		}
	};

	if tmp.len() < 1 {
		fail!("USAGE: ./fmod [music_file]");
	}

	let arg1 = tmp.get(0).unwrap();

	let mut sound = match fmod.create_sound(StrBuf::from_str(*arg1)) {
		Ok(s) => s,
		Err(err) => {fail!("Error code : {}", err);},
	};

	match sound.play_to_the_end() {
		FMOD_OK => {println!("Ok !");},
		err => {fail!("Error code : {}", err);},
	};

	sound.release();
	fmod.release();
}