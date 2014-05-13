#![feature(globs)]

extern crate libc;

use fmod::*;
use std::os;

mod fmod;

fn main() {
	let args = os::args();
	let tmp = args.tail();
	let mut fmod = match Fmod::new() {
		Ok(f) => f,
		Err(e) => {
			fail!("Error code : {}", e);
		}
	};

	if tmp.len() < 1 {
		fail!("USAGE: ./fmod [music_file]");
	}

	let arg1 = tmp.get(0).unwrap();

	let sound = match fmod.create_sound(StrBuf::from_str(*arg1)) {
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