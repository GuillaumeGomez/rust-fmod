rust-fmod
=========

This is a rust binding for FMOD, developped by FIRELIGHT TECHNOLOGIES.

FMOD website : www.fmod.org


##Installation

You must install on your computer the FMOD library which is used for the binding.

##Short example

Here is a short example, take the first program's argument and read it.

```Rust
#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::*;
use std::os;

fn main() {
   let fmod = match Fmod::new() {
       Ok(f) => f,
       Err(e) => {
       	  fail!("Error code : {}", e);
       }
   };

   let mut sound = match fmod.create_sound(StrBuf::from_str("music.mp3")) {
		      Ok(s) => s,
		      Err(err) => {fail!("Error code : {}", err);},
		   };

   match sound.play_to_the_end() {
      FMOD_OK => {println!("Ok !");}
      err => {fail!("Error code : {}", err);}
   };

   sound.release();
   fmod.release();
}


##License

    Copyright (c) 2014 Guillaume Gomez
    
    The license of this project is the same of the FMOD non-commercial use. 
    Please refer to it. Here is the website for FMOD : http://www.fmod.org/
