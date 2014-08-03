rust-fmod [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-fmod.png?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-fmod)
=========

This is a rust binding for __FMOD__, the library developped by FIRELIGHT TECHNOLOGIES.

__FMOD__ website : www.fmod.org


##Installation

You must install on your computer the __FMOD__ library which is used for the binding.

To build it, please use :

```Shell
> make
```

This command build __rfmod__, the examples and the documentation.

You can build them separatly too.

```Shell
> make rfmod
> make examples
> make doc
```

Since this project supports cargo, you can also build it like this :

```Shell
> cargo build
```

##Documentation

You can access the __rfmod__ documentation locally, just build it :

```Shell
> make doc
```

Then open this file with an internet browser :
file:///{rfmod_location}/doc/rfmod/index.html


You can also access the latest build of the documentation via the internet :
http://rust-ci.org/GuillaumeGomez/rust-fmod/doc/rfmod/

##Short example

Here is a short example on how to create a file and to play it :

```Rust
#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::*;
use rfmod::*;
use std::os;

fn main() {
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            fail!("Error code : {}", e);
        }
    };

    match fmod.init() {
        fmod::Ok => {}
        e => {
            fmod.release();
            fail!("FmodSys.init failed : {}", e);
        }
    };

    let mut sound = match fmod.create_sound(StrBuf::from_str("music.mp3"), None, None) {
                      Ok(s) => s,
                      Err(err) => {fail!("Error code : {}", err);},
                    };

    match sound.play_to_the_end() {
        fmod::Ok => {println!("Ok !");}
        err => {fail!("Error code : {}", err);}
    };
}
```

For a more complete example : https://github.com/GuillaumeGomez/rust-music-player

##License

    Copyright (c) 2014 Guillaume Gomez
    
    The license of this project is the same of the FMOD non-commercial use. 
    Please refer to it. Here is the website for FMOD : http://www.fmod.org/
