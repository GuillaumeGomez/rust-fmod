#![feature(globs)]

extern crate rfmod;

use rfmod::enums::*;
use rfmod::callbacks::*;
use rfmod::*;
use std::os;
use std::io::timer::sleep;

fn get_key() -> u8 {
    let mut reader = std::io::stdio::stdin();
    
    println!("\nPress a corresponding number or ESC to quit");
    print!("> ");

    match reader.read_byte() {
        Ok(nb) => nb,
        Err(_) => 0u8
    }
}

extern fn my_DSP_callback(dsp_state: DspState, float *inbuffer, float *outbuffer, unsigned int length, int inchannels, int outchannels) {
    println!("I'm called from C with value {0}", a);
}

fn main() {
    let args = os::args();
    let tmp = args.tail();

    if tmp.len() < 1 {
        fail!("USAGE: ./dsp_custom [music_file]");
    }
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            fail!("FmodSys.new : {}", e);
        }
    };

    match fmod.init() {
        fmod::Ok => {}
        e => {
            fail!("FmodSys.init failed : {}", e);
        }
    };

    let arg1 = tmp.get(0).unwrap();

    let sound = match fmod.create_sound(String::from_str((*arg1).as_slice()), Some(FmodMode(FMOD_SOFTWARE | FMOD_LOOP_NORMAL)), None) {
        Ok(s) => s,
        Err(err) => {fail!("FmodSys.create_sound failed : {}", err);}
    };

    println!("============================");
    println!("======== Custom DSP ========");
    println!("============================\n");
    println!("Press 'f' to activate / deactivate user filter");
    println!("Press 'Esc' to quit");

    let channel = match sound.play() {
        Ok(c) => c,
        Err(e) => {fail!("Sound.play failed : {}", e);}
    }

    let mut description = DspDescription::new();
    description.read = Some(my_DSP_callback);

    let dsp = match fmod.create_DSP_with_description(&description) {
        Ok(dsp) => dsp,
        Err(e) => {fail!("FmodSys.create_DSP_with_description failed : {}", e);}
    };

    dsp.set_bypass(true);
    fmod.add_DSP(&dsp).unwrap();

    let mut active = false;
    loop {
        match get_key() as char {
            'f' => {
                dsp.set_bypass(active);
                active = !active;
                fmod.update();
            }
            c if c == 27u8 as char => break,
            _ => None
        }
    }
}