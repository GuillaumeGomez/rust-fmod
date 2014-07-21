#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::*;
use rfmod::types::*;
use rfmod::*;
use std::default::Default;
use std::io::timer::sleep;

#[allow(unused_variable)]
fn pcmreadcallback(sound: &Sound, data: &mut [i16]) -> fmod::Result {
    static mut t1 : f32 = 0f32; // time
    static mut t2 : f32 = 0f32; // time
    static mut v1 : f32 = 0f32; // velocity
    static mut v2 : f32 = 0f32; // velocity
    let mut count = 0u;

    while count < data.len() {
        unsafe {
            data[count] = (t1.sin() * 32767f32) as i16; // left channel
            count += 1;
            data[count] = (t2.sin() * 32767f32) as i16; // right channel
            count += 1;

            t1 += 0.01f32 + v1;
            t2 += 0.0142f32 + v2;
            v1 += t1.sin() * 0.002f32;
            v2 += t2.sin() * 0.002f32;
        }
    }

    fmod::Ok
}


fn get_key() -> Result<int, std::io::IoError> {
    let mut reader = std::io::stdio::stdin();
    print!("> ");

    match reader.read_line() {
        Ok(mut line) => {
            let length = line.len() - 1;
            line.truncate(length);
            if line.as_slice() == "Quit" {
                Ok(-1)
            } else {
                Ok(from_str(line.as_slice()).unwrap())
            }
        }
        Err(e) => Err(e)
    }
}

fn main() {
    let channels = 2i32;
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            fail!("FmodSys.new : {}", e);
        }
    };

    match fmod.init_with_parameters(32i32, FmodInitFlag(FMOD_INIT_NORMAL)) {
        fmod::Ok => {}
        e => {
            fail!("FmodSys.init failed : {}", e);
        }
    };

    println!("============================================================================");
    println!("============== User Created Sound Example from FMOD examples ===============");
    println!("============================================================================");
    println!("Sound played here is generated in realtime.  It will either play as a stream");
    println!("which means it is continually filled as it is playing, or it will play as a");
    println!("static sample, which means it is filled once as the sound is created, then");
    println!("when played it will just play that short loop of data.");
    println!("============================================================================\n");
    println!("Enter '1' to play as a runtime decoded stream. (will carry on infinitely)");
    println!("Enter '2' to play as a static in memory sample. (loops a short block of data)");
    println!("Enter 'Quit' to quit.");
    

    let mut ret = get_key().unwrap();
    while ret != -1 && ret != 1 && ret != 2 {
        println!("Invalid entry");
        ret = get_key().unwrap();
    }
    let mut exinfo : FmodCreateSoundexInfo = Default::default();

    exinfo.decode_buffer_size = 44100;
    exinfo.length = 44100u32 * channels as u32 * std::mem::size_of::<i16>() as u32 * 5u32;
    exinfo.num_channels = channels;
    exinfo.default_frequency  = 44100;
    exinfo.format = fmod::SoundFormatPCM16;
    exinfo.pcm_read_callback = Some(pcmreadcallback);

    let sound = match match ret {
        1 => fmod.create_sound("", Some(FmodMode(FMOD_2D | FMOD_OPENUSER | FMOD_HARDWARE | FMOD_LOOP_NORMAL | FMOD_CREATESTREAM)), Some(&mut exinfo)),
        2 => fmod.create_sound("", Some(FmodMode(FMOD_2D | FMOD_OPENUSER | FMOD_HARDWARE | FMOD_LOOP_NORMAL)), Some(&mut exinfo)),
        _ => return
    } {
        Ok(s) => s,
        Err(e) => fail!("create sound error: {}", e)
    };

    let chan = match sound.play() {
        Ok(c) => c,
        Err(e) => fail!("sound.play error: {}", e)
    };

    let length = sound.get_length(FMOD_TIMEUNIT_MS).unwrap();
    while chan.is_playing().unwrap() {
        let position = chan.get_position(FMOD_TIMEUNIT_MS).unwrap();

        print!("{:02u}:{:02u} / {:02u}:{:02u}\r", position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60);
        sleep(30)
    }
}