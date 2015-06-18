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

#![crate_type = "bin"]

extern crate rfmod;

use std::thread::sleep_ms;
use std::io::{self, Error, BufRead};

fn get_key() -> Result<isize, Error> {
    let r = io::stdin();
    let mut reader = r.lock();
    let mut line = String::new();

    print!("> ");
    match reader.read_line(&mut line) {
        Ok(_) => {
            let length = line.len() - 1;
            line.truncate(length);
            if &line == "quit" {
                Ok(-1)
            } else {
                match line.parse() {
                    Ok(s) => Ok(s),
                    Err(_) => Ok(9)
                }
            }
        }
        Err(e) => Err(e)
    }
}

fn switch_dsp_state(dsp: &rfmod::Dsp, fmod: &rfmod::Sys, dsp_type: isize) {
    if match dsp.get_active() {
        Ok(c) => c,
        Err(_) => return
    } {
        dsp.remove();
    } else {
        match fmod.add_DSP(dsp) { _ => {}};
        match dsp_type {
            3 => {
                dsp.set_parameter(rfmod::DspTypeEcho::Delay as i32, 50f32);
            },
            5 => {
                dsp.set_parameter(rfmod::DspDistortion::Level as i32, 0.8f32);
            },
            7 => {
                dsp.set_parameter(rfmod::DspTypeParameq::Center as i32, 5000f32);
                dsp.set_parameter(rfmod::DspTypeParameq::Gain as i32, 0f32);
            }
            _ => {}
        };
    }
}

fn main() {
    let t_args = std::env::args();
    let mut args = Vec::new();

    for tmp in t_args {
        args.push(tmp);
    }
    let tmp = args[1..].to_owned();

    if tmp.len() < 1 {
        panic!("USAGE: ./effects [music_file]");
    }
    let fmod = match rfmod::Sys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Sys::new() : {:?}", e);
        }
    };

    match fmod.init_with_parameters(32i32, rfmod::InitFlag(rfmod::INIT_NORMAL)) {
        rfmod::Result::Ok => {}
        e => {
            panic!("Sys::init() failed : {:?}", e);
        }
    };

    println!("==============================================");
    println!("===== Effects example from FMOD examples =====");
    println!("==============================================");

    let arg1 = tmp.get(0).unwrap();
    let sound = match fmod.create_sound(&(*arg1), Some(rfmod::Mode(rfmod::SOFTWARE)), None) {
        Ok(s) => s,
        Err(e) => panic!("create sound error: {:?}", e)
    };
    sound.set_mode(rfmod::Mode(rfmod::LOOP_NORMAL));

    match sound.play() {
        Ok(_) => {},
        Err(e) => panic!("sound.play error: {:?}", e)
    };
    let mut dsps = Vec::new();
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::LowPass) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type low_pass error: {:?}", e)
    });
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::HighPass) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type high_pass error: {:?}", e)
    });
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::Echo) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type echo error: {:?}", e)
    });
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::Flange) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type flange error: {:?}", e)
    });
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::Distortion) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type distortion error: {:?}", e)
    });
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::Chorus) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type chorus error: {:?}", e)
    });
    dsps.push(match fmod.create_DSP_by_type(rfmod::DspType::Parameq) {
        Ok(r) => r,
        Err(e) => panic!("fmod.create_DSP_by_type parameq error: {:?}", e)
    });

    println!("Enter '1' to toggle dsp low pass effect.");
    println!("Enter '2' to toggle dsp high pass effect.");
    println!("Enter '3' to toggle dsp echo effect.");
    println!("Enter '4' to toggle dsp flange effect.");
    println!("Enter '5' to toggle dsp distortion effect.");
    println!("Enter '6' to toggle dsp chorus effect.");
    println!("Enter '7' to toggle dsp parameq effect.");
    println!("Enter 'quit' to quit.");
    loop {
        println!("low pass[{}] high pass[{}] echo[{}] flange[{}] dist[{}] chorus[{}] parameq[{}]",
            match dsps[0].get_active() { Ok(true) => {'x'}, _ => {' '}},
            match dsps[1].get_active() { Ok(true) => {'x'}, _ => {' '}},
            match dsps[2].get_active() { Ok(true) => {'x'}, _ => {' '}},
            match dsps[3].get_active() { Ok(true) => {'x'}, _ => {' '}},
            match dsps[4].get_active() { Ok(true) => {'x'}, _ => {' '}},
            match dsps[5].get_active() { Ok(true) => {'x'}, _ => {' '}},
            match dsps[6].get_active() { Ok(true) => {'x'}, _ => {' '}});
        match get_key() {
            Ok(v) => match v {
                -1 => break,
                x if x > 0 && x < 8 => {
                    switch_dsp_state(&dsps[x as usize - 1], &fmod, x)
                },
                _ => println!("Invalid entry")
            },
            Err(e) => panic!("Entry error: {:?}", e)
        }
        fmod.update();
        sleep_ms(30); // let time to the system for update
    }
}