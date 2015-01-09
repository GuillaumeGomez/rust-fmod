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
#![allow(unstable)]

extern crate libc;
extern crate rfmod;

use std::default::Default;
use std::io::timer::sleep;
use std::os;
use std::time::duration::Duration;
use std::num::Float;

fn main() {
    let args = os::args();
    let tmp = args.tail();

    if tmp.len() < 1 {
        panic!("USAGE: ./3d [music_file]");
    }
    let fmod = match rfmod::FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("FmodSys.new : {:?}", e);
        }
    };

    match fmod.init_with_parameters(10i32, rfmod::FmodInitFlag(rfmod::FMOD_INIT_NORMAL)) {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.init failed : {:?}", e);
        }
    };

    println!("=========================================");
    println!("============== 3D example ===============");
    println!("=========================================");

    let arg1 = tmp.get(0).unwrap();
    let sound = match fmod.create_sound((*arg1).as_slice(), Some(rfmod::FmodMode(rfmod::FMOD_3D | rfmod::FMOD_SOFTWARE)), None) {
        Ok(s) => s,
        Err(e) => panic!("create sound error: {:?}", e)
    };
    sound.set_3D_min_max_distance(4f32, 10000f32);
    sound.set_mode(rfmod::FmodMode(rfmod::FMOD_LOOP_NORMAL));

    let chan = match sound.play() {
        Ok(c) => c,
        Err(e) => panic!("sound.play error: {:?}", e)
    };
    chan.set_3D_attributes(&rfmod::FmodVector{x: -10f32, y: 0f32, z: 0f32}, &Default::default());

    let mut last_pos = rfmod::FmodVector::new();
    let mut listener_pos = rfmod::FmodVector::new();
    let mut t = 0f32;
    let interface_update_time = 50f32;
    let compensate_time = 1000f32 / interface_update_time;

    while chan.is_playing().unwrap() {
        let forward = rfmod::FmodVector{x: 0f32, y: 0f32, z: 1f32};
        let up = rfmod::FmodVector{x: 0f32, y: 1f32, z: 0f32};
        let mut vel = rfmod::FmodVector::new();

        listener_pos.x = (t * 0.05f32).sin() * 33f32; // left right ping-pong
        vel.x = (listener_pos.x - last_pos.x) * compensate_time;
        vel.y = (listener_pos.y - last_pos.y) * compensate_time;
        vel.z = (listener_pos.z - last_pos.z) * compensate_time;
        t += 30f32 * (1f32 / interface_update_time);

        last_pos = listener_pos;
        fmod.set_3D_listener_attributes(0, &listener_pos, &vel, &forward, &up);

        let mut tmp = String::from_str("|.......................<1>......................<2>....................|\r");
        unsafe { tmp.as_mut_vec().as_mut_slice()[listener_pos.x as usize + 35us] = 'L' as u8; }
        print!("{}", tmp);
        fmod.update();
        sleep(Duration::milliseconds(interface_update_time as i64 - 1i64));
    }
}