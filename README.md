rust-fmod [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-fmod.png?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-fmod)
=========

This is a rust binding for __FMOD__, the library developped by FIRELIGHT TECHNOLOGIES.

__FMOD__ website : www.fmod.org

You can also find it on [crates.io](https://crates.io/crates/fmod) !

## Installation

You must install on your computer the __FMOD__ library which is used for the binding.

Since this project supports cargo, you can build it this way:

```Shell
> cargo build
```

This isn't a binding to the lastest version. You can find the bound version [here](http://www.guillaume-gomez.fr/fmodapi44439linux.tar.gz).

## Documentation

You can access the __rfmod__ documentation locally, just build it :

```Shell
> cargo doc
```

Then open this file with an internet browser :
file:///{rfmod_location}/doc/rfmod/index.html


You can also access the latest build of the documentation via the internet from [here](http://rust-ci.org/GuillaumeGomez/rust-fmod/doc/rfmod/).

## Short example

Here is a short example on how to create a file and to play it :

```Rust
extern crate rfmod;

fn main() {
    let fmod = match rfmod::Sys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Error code : {:?}", e);
        }
    };

    match fmod.init() {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.init failed : {:?}", e);
        }
    };

    let sound = match fmod.create_sound("music.mp3", None, None) {
        Ok(s) => s,
        Err(err) => {
            panic!("Error code : {:?}", err);
        }
    };

    match sound.play_to_the_end() {
        rfmod::Result::Ok => {
            println!("Ok !");
        }
        err => {
            panic!("Error code : {:?}", err);
        }
    };
}
```

For a more complete example : https://github.com/GuillaumeGomez/rust-music-player

## License

    Please refer to the LICENSE.txt file for more details.
    If you want more information, here is the FMOD website : http://www.fmod.org/
