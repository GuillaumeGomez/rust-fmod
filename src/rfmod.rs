/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FmodEx library, is provided by FIRELIGHT TECHNOLOGIES.
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

/*!
rust-fmod
=========

This is a rust binding for __FMOD__, the library developped by FIRELIGHT TECHNOLOGIES.

__FMOD__ website : www.fmod.org


##Installation

You must install on your computer the __FMOD__ library which is used for the binding.

To build it, please use :

```Shell
> make
```

This command build __rfmod__, the examples, and the documentation.

You can build them separatly too.

```Shell
> make rfmod
> make examples
> make doc
```

##Short example

Here is a short example on how to create a file and to play it :

```Rust
extern crate libc;
extern crate rfmod;

use std::os;

fn main() {
    let fmod = match rfmod::FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("Error code : {}", e);
        }
    };

    match fmod.init() {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.init failed : {}", e);
        }
    };

    let mut sound = match fmod.create_sound(StrBuf::from_str("music.mp3"), None, None) {
        Ok(s) => s,
        Err(err) => {
            panic!("Error code : {}", err);
        }
    };

    match sound.play_to_the_end() {
        rfmod::Result::Ok => {
            println!("Ok !");
        }
        err => {
            panic!("Error code : {}", err);
        }
    };
}
```

For a more complete example : https://github.com/GuillaumeGomez/rust-music-player

##License

    Copyright (c) 2014 Guillaume Gomez

    The license of this project is available in the LICENSE.TXT file. Please refer to it.
    If you want more information, here is the website for FMOD : http://www.fmod.org/

#Notes

 * Members marked with [r] mean the variable is modified by FMOD and is for reading purposes only. Do not change this value.
 * Members marked with [w] mean the variable can be written to. The user can set the value.


Here is the list of all modules :
!*/

#![crate_name = "rfmod"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(raw_pointer_derive)]

#![feature(core, libc, std_misc, unique, old_io, old_path, convert)]

extern crate libc;
extern crate c_str;
extern crate c_vec;
extern crate byteorder;

pub use channel::{Channel, FmodSpeakerMixOptions, FmodReverbChannelProperties};
pub use fmod_sys::{FmodSys, FmodGuid, FmodSoftwareFormat, FmodAdvancedSettings, FmodOutputHandle, FmodCreateSoundexInfo, FmodMemoryUsageDetails, FmodUserData};
pub use sound::{Sound, FmodTag, FmodSyncPoint};
pub use channel_group::{ChannelGroup};
pub use sound_group::SoundGroup;
pub use dsp::{Dsp, DspParameterDesc, DspDescription, DspState};
pub use dsp_connection::DspConnection;
pub use reverb::Reverb;
pub use reverb_properties::ReverbProperties;
pub use vector::FmodVector;
pub use geometry::Geometry;
pub use file::FmodFile;

pub use self::enums::result::Result;
pub use self::enums::speaker_map_type::SpeakerMapType;
pub use self::enums::sound_type::SoundType;
pub use self::enums::sound_format::SoundFormat;
pub use self::enums::tag_type::TagType;
pub use self::enums::tag_data_type::TagDataType;
pub use self::enums::channel_index::ChannelIndex;
pub use self::enums::dsp_fft_window::DspFftWindow;
pub use self::enums::delay_type::DelayType;
pub use self::enums::output_type::OutputType;
pub use self::enums::speaker::Speaker;
pub use self::enums::speaker_mode::SpeakerMode;
pub use self::enums::dsp_resampler::DspResampler;
pub use self::enums::plugin_type::PluginType;
pub use self::enums::open_state::OpenState;
pub use self::enums::system_callback_type::SystemCallbackType;
pub use self::enums::sound_group_behavior::SoundGroupBehavior;
pub use self::enums::dsp_type::DspType;
pub use self::enums::dsp_oscillator::DspOscillator;
pub use self::enums::dsp_low_pass::DspLowPass;
pub use self::enums::dsp_it_low_pass::DspITLowPass;
pub use self::enums::dsp_type_echo::DspTypeEcho;
pub use self::enums::dsp_delay::DspDelay;
pub use self::enums::dsp_flange::DspFlange;
pub use self::enums::dsp_tremolo::DspTremolo;
pub use self::enums::dsp_distortion::DspDistortion;
pub use self::enums::dsp_normalize::DspNormalize;
pub use self::enums::dsp_type_parameq::DspTypeParameq;
pub use self::enums::dsp_pitch_shift::DspPitchShift;
pub use self::enums::dsp_chorus::DspChorus;
pub use self::enums::dsp_it_echo::DspITEcho;
pub use self::enums::dsp_compressor::DspCompressor;
pub use self::enums::dsp_sfx_reverb::DspSfxReverb;
pub use self::enums::dsp_low_pass_simple::DspLowPassSimple;
pub use self::enums::dsp_high_pass_simple::DspHighPassSimple;

pub use self::types::{
    FmodMode,
    FmodTimeUnit,
    FmodCaps,
    FmodPluginHandle,
    FmodInitFlag,
    FmodMemoryBits,
    FmodEventMemoryBits,
};

mod ffi;
mod sound;
mod channel;
mod channel_group;
mod sound_group;
mod fmod_sys;
mod dsp;
mod dsp_connection;
mod geometry;
mod vector;
mod reverb;
mod reverb_properties;
mod file;
mod enums;
pub mod types;
pub mod callbacks;
pub mod error;

/// Default for all modes listed below. FMOD_LOOP_OFF, FMOD_2D, FMOD_HARDWARE
pub const FMOD_DEFAULT                : u32 = 0x00000000;
/// For non looping sounds. (DEFAULT). Overrides FMOD_LOOP_NORMAL / FMOD_LOOP_BIDI.
pub const FMOD_LOOP_OFF               : u32 = 0x00000001;
/// For forward looping sounds.
pub const FMOD_LOOP_NORMAL            : u32 = 0x00000002;
/// For bidirectional looping sounds. (only works on software mixed static sounds).
pub const FMOD_LOOP_BIDI              : u32 = 0x00000004;
/// Ignores any 3d processing. (DEFAULT).
pub const FMOD_2D                     : u32 = 0x00000008;
/// Makes the sound positionable in 3D. Overrides FMOD_2D
pub const FMOD_3D                     : u32 = 0x00000010;
/// Attempts to make sounds use hardware acceleration. (DEFAULT). Note on platforms that don't support FMOD_HARDWARE (only 3DS, PS Vita, PSP, Wii and Wii U support FMOD_HARDWARE), this will be internally treated as FMOD_SOFTWARE.
pub const FMOD_HARDWARE               : u32 = 0x00000020;
/// Makes the sound be mixed by the FMOD CPU based software mixer. Overrides FMOD_HARDWARE. Use this for FFT, DSP, compressed sample support, 2D multi-speaker support and other software related features.
pub const FMOD_SOFTWARE               : u32 = 0x00000040;
/// Decompress at runtime, streaming from the source provided (ie from disk). Overrides FMOD_CREATESAMPLE and FMOD_CREATECOMPRESSEDSAMPLE. Note a stream can only be played once at a time due to a stream only having 1 stream buffer and file handle. Open multiple streams to have them play concurrently.
pub const FMOD_CREATESTREAM           : u32 = 0x00000080;
/// Decompress at loadtime, decompressing or decoding whole file into memory as the target sample format (ie PCM). Fastest for FMOD_SOFTWARE based playback and most flexible.
pub const FMOD_CREATESAMPLE           : u32 = 0x00000100;
/// Load MP2/MP3/IMAADPCM/CELT/Vorbis/AT9 or XMA into memory and leave it compressed. CELT/Vorbis/AT9 encoding only supported in the FSB file format. During playback the FMOD software mixer will decode it in realtime as a 'compressed sample'. Can only be used in combination with FMOD_SOFTWARE. Overrides FMOD_CREATESAMPLE. If the sound data is not one of the supported formats, it will behave as if it was created with FMOD_CREATESAMPLE and decode the sound into PCM.
pub const FMOD_CREATECOMPRESSEDSAMPLE : u32 = 0x00000200;
/// Opens a user created static sample or stream. Use FMOD_CREATESOUNDEXINFO to specify format and/or read callbacks. If a user created 'sample' is created with no read callback, the sample will be empty. Use [`Sound::lock`](../struct.Sound.html#method.lock) and [`Sound::unlock`](../struct.Sound.html#method.unlock) to place sound data into the sound if this is the case.
pub const FMOD_OPENUSER               : u32 = 0x00000400;
/// "name_or_data" will be interpreted as a pointer to memory instead of filename for creating sounds. Use FMOD_CREATESOUNDEXINFO to specify length. If used with FMOD_CREATESAMPLE or FMOD_CREATECOMPRESSEDSAMPLE, FMOD duplicates the memory into its own buffers. Your own buffer can be freed after open. If used with FMOD_CREATESTREAM, FMOD will stream out of the buffer whose pointer you passed in. In this case, your own buffer should not be freed until you have finished with and released the stream.
pub const FMOD_OPENMEMORY             : u32 = 0x00000800;
/// "name_or_data" will be interpreted as a pointer to memory instead of filename for creating sounds. Use FMOD_CREATESOUNDEXINFO to specify length. This differs to FMOD_OPENMEMORY in that it uses the memory as is, without duplicating the memory into its own buffers. For Wii/PSP FMOD_HARDWARE supports this flag for the GCADPCM/VAG formats. On other platforms FMOD_SOFTWARE must be used, as sound hardware on the other platforms (ie PC) cannot access main ram. Cannot be freed after open, only after [`Sound::release`](../struct.Sound.html#method.release). Will not work if the data is compressed and FMOD_CREATECOMPRESSEDSAMPLE is not used.
pub const FMOD_OPENMEMORY_POINT       : u32 = 0x10000000;
/// Will ignore file format and treat as raw pcm. Use FMOD_CREATESOUNDEXINFO to specify format. Requires at least defaultfrequency, numchannels and format to be specified before it will open. Must be little endian data.
pub const FMOD_OPENRAW                : u32 = 0x00001000;
/// Just open the file, dont prebuffer or read. Good for fast opens for info, or when sound::readData is to be used.
pub const FMOD_OPENONLY               : u32 = 0x00002000;
/// For [`FmodSys::create_sound`](../struct.FmodSys.html#method.create_sound) - for accurate [`Sound::get_length`](../struct.Sound.html#method.get_length) / [`Channel::set_position`](../struct.Channel.html#method.set_position) on VBR MP3, and MOD/S3M/XM/IT/MIDI files. Scans file first, so takes longer to open. FMOD_OPENONLY does not affect this.
pub const FMOD_ACCURATETIME           : u32 = 0x00004000;
/// For corrupted / bad MP3 files. This will search all the way through the file until it hits a valid MPEG header. Normally only searches for 4k.
pub const FMOD_MPEGSEARCH             : u32 = 0x00008000;
/// For opening sounds and getting streamed subsounds (seeking) asyncronously. Use [`Sound::get_open_state`](../struct.Sound.html#method.get_open_state) to poll the state of the sound as it opens or retrieves the subsound in the background.
pub const FMOD_NONBLOCKING            : u32 = 0x00010000;
/// Unique sound, can only be played one at a time
pub const FMOD_UNIQUE                 : u32 = 0x00020000;
/// Make the sound's position, velocity and orientation relative to the listener.
pub const FMOD_3D_HEADRELATIVE        : u32 = 0x00040000;
/// Make the sound's position, velocity and orientation absolute (relative to the world). (DEFAULT)
pub const FMOD_3D_WORLDRELATIVE       : u32 = 0x00080000;
/// This sound will follow the inverse rolloff model where mindistance = full volume, maxdistance = where sound stops attenuating, and rolloff is fixed according to the global rolloff factor. (DEFAULT)
pub const FMOD_3D_INVERSEROLLOFF      : u32 = 0x00100000;
/// This sound will follow a linear rolloff model where mindistance = full volume, maxdistance = silence. Rolloffscale is ignored.
pub const FMOD_3D_LINEARROLLOFF       : u32 = 0x00200000;
/// This sound will follow a linear-square rolloff model where mindistance = full volume, maxdistance = silence. Rolloffscale is ignored.
pub const FMOD_3D_LINEARSQUAREROLLOFF : u32 = 0x00400000;
/// This sound will follow a rolloff model defined by [`Sound::set_3D_custom_rolloff`](../struct.Sound.html#method.set_3D_custom_rolloff) / [`Channel::set_3D_custom_rolloff`](../struct.Channel.html#method.set_3D_custom_rolloff).
pub const FMOD_3D_CUSTOMROLLOFF       : u32 = 0x04000000;
/// Is not affect by geometry occlusion. If not specified in [`Sound::set_mode`](../struct.Sound.html#method.set_mode), or [`Channel::set_mode`](../struct.Channel.html#method.set_mode), the flag is cleared and it is affected by geometry again.
pub const FMOD_3D_IGNOREGEOMETRY      : u32 = 0x40000000;
/// Filename is double-byte unicode.
pub const FMOD_UNICODE                : u32 = 0x01000000;
/// Skips id3v2/asf/etc tag checks when opening a sound, to reduce seek/read overhead when opening files (helps with CD performance).
pub const FMOD_IGNORETAGS             : u32 = 0x02000000;
/// Removes some features from samples to give a lower memory overhead, like [`Sound::get_name`](../struct.Sound.html#method.get_name). See remarks.
pub const FMOD_LOWMEM                 : u32 = 0x08000000;
/// Load sound into the secondary RAM of supported platform. On PS3, sounds will be loaded into RSX/VRAM.
pub const FMOD_LOADSECONDARYRAM       : u32 = 0x20000000;
/// For sounds that start virtual (due to being quiet or low importance), instead of swapping back to audible, and playing at the correct offset according to time, this flag makes the sound play from the start.
pub const FMOD_VIRTUAL_PLAYFROMSTART  : u32 = 0x80000000;

/// All platforms - Initialize normally
pub const FMOD_INIT_NORMAL                    : u32 = 0x00000000;
/// All platforms - No stream thread is created internally. Streams are driven from [`FmodSys::update`](../struct.FmodSys.html#method.update). Mainly used with non-realtime outputs.
pub const FMOD_INIT_STREAM_FROM_UPDATE        : u32 = 0x00000001;
/// All platforms - FMOD will treat +X as right, +Y as up and +Z as backwards (towards you).
pub const FMOD_INIT_3D_RIGHTHANDED            : u32 = 0x00000002;
/// All platforms - Disable software mixer to save memory. Anything created with FMOD_SOFTWARE will fail and DSP will not work.
pub const FMOD_INIT_SOFTWARE_DISABLE          : u32 = 0x00000004;
/// All platforms - All FMOD_SOFTWARE (and FMOD_HARDWARE on 3DS and NGP) with FMOD_3D based voices will add a software lowpass filter effect into the DSP chain which is automatically used when [`Channel::set_3D_occlusion`](../struct.Channel.html#method.set_3D_occlusion) is used or the geometry API.
pub const FMOD_INIT_OCCLUSION_LOWPASS         : u32 = 0x00000008;
/// All platforms - All FMOD_SOFTWARE (and FMOD_HARDWARE on 3DS and NGP) with FMOD_3D based voices will add a software lowpass filter effect into the DSP chain which causes sounds to sound duller when the sound goes behind the listener. Use [`FmodSys::set_advanced_settings`](../struct.FmodSys.html#method.set_advanced_settings) to adjust Cutoff frequency.
pub const FMOD_INIT_HRTF_LOWPASS              : u32 = 0x00000010;
/// All platforms - All FMOD_SOFTWARE with FMOD_3D based voices will add a software lowpass and highpass filter effect into the DSP chain which will act as a distance-automated bandpass filter. Use [`FmodSys::set_advanced_settings`](../struct.FmodSys.html#method.set_advanced_settings) to adjust the center frequency.
pub const FMOD_INIT_DISTANCE_FILTERING        : u32 = 0x00000200;
/// All platforms - FMOD Software reverb will preallocate enough buffers for reverb per channel, rather than allocating them and freeing them at runtime.
pub const FMOD_INIT_REVERB_PREALLOCBUFFERS    : u32 = 0x00000040;
/// All platforms - Enable TCP/IP based host which allows FMOD Designer or FMOD Profiler to connect to it, and view memory, CPU and the DSP network graph in real-time.
pub const FMOD_INIT_ENABLE_PROFILE            : u32 = 0x00000020;
/// All platforms - Any sounds that are 0 volume will go virtual and not be processed except for having their positions updated virtually. Use [`FmodSys::set_advanced_settings`](../struct.FmodSys.html#method.set_advanced_settings) to adjust what volume besides zero to switch to virtual at.
pub const FMOD_INIT_VOL0_BECOMES_VIRTUAL      : u32 = 0x00000080;
/// Win32 Vista only - for WASAPI output - Enable exclusive access to hardware, lower latency at the expense of excluding other applications from accessing the audio hardware.
pub const FMOD_INIT_WASAPI_EXCLUSIVE          : u32 = 0x00000100;
/// PS3 only - Prefer DTS over Dolby Digital if both are supported. Note: 8 and 6 channel LPCM is always preferred over both DTS and Dolby Digital.
pub const FMOD_INIT_PS3_PREFERDTS             : u32 = 0x00800000;
/// PS3 only - Force PS3 system output mode to 2 channel LPCM.
pub const FMOD_INIT_PS3_FORCE2CHLPCM          : u32 = 0x01000000;
/// Wii / 3DS - Disable Dolby Pro Logic surround.  will be set to STEREO even if user has selected surround in the system settings.
pub const FMOD_INIT_DISABLEDOLBY              : u32 = 0x00100000;
/// Xbox 360 / PS3 - The "music" channelgroup which by default pauses when custom 360 dashboard / PS3 BGM music is played, can be changed to mute (therefore continues playing) instead of pausing, by using this flag.
pub const FMOD_INIT_SYSTEM_MUSICMUTENOTPAUSE  : u32 = 0x00200000;
/// Win32/Wii/PS3/Xbox/Xbox 360 - FMOD Mixer thread is woken up to do a mix when [`FmodSys::update`](../struct.FmodSys.html#method.update) is called rather than waking periodically on its own timer.
pub const FMOD_INIT_SYNCMIXERWITHUPDATE       : u32 = 0x00400000;
/// All platforms - With the geometry engine, only process the closest polygon rather than accumulating all polygons the sound to listener line intersects.
pub const FMOD_INIT_GEOMETRY_USECLOSEST       : u32 = 0x04000000;
/// Win32 - Disables automatic setting of of FMOD__STEREO to FMOD__MYEARS if the MyEars profile exists on the PC. MyEars is HRTF 7.1 downmixing through headphones.
pub const FMOD_INIT_DISABLE_MYEARS_AUTODETECT : u32 = 0x08000000;
/// PS3 only - Disable DTS output mode selection
pub const FMOD_INIT_PS3_DISABLEDTS            : u32 = 0x10000000;
/// PS3 only - Disable Dolby Digital output mode selection
pub const FMOD_INIT_PS3_DISABLEDOLBYDIGITAL   : u32 = 0x20000000;
/// PS3/PS4 only - FMOD uses the WAVEFORMATEX Microsoft 7.1 speaker mapping where the last 2 pairs of speakers are 'rears' then 'sides', but on PS3/PS4 these are mapped to 'surrounds' and 'backs'. Use this flag to swap fmod's last 2 pair of speakers on PS3/PS4 to avoid needing to do a special case for these platforms.
pub const FMOD_INIT_7POINT1_DOLBYMAPPING      : u32 = 0x40000000;

/// Milliseconds.
pub const FMOD_TIMEUNIT_MS               : FmodTimeUnit = FmodTimeUnit(0x00000001);
/// PCM samples, related to milliseconds * samplerate / 1000.
pub const FMOD_TIMEUNIT_PCM              : FmodTimeUnit = FmodTimeUnit(0x00000002);
/// Bytes, related to PCM samples * channels * datawidth (ie 16bit = 2 bytes).
pub const FMOD_TIMEUNIT_PCMBYTES         : FmodTimeUnit = FmodTimeUnit(0x00000004);
/// Raw file bytes of (compressed) sound data (does not include headers). Only used by [`Sound::get_length`](../struct.Sound.html#method.get_length) and [`Channel::get_position`](../struct.Channel.html#method.get_position).
pub const FMOD_TIMEUNIT_RAWBYTES         : FmodTimeUnit = FmodTimeUnit(0x00000008);
/// Fractions of 1 PCM sample. Unsigned int range 0 to 0xFFFFFFFF. Used for sub-sample granularity for DSP purposes.
pub const FMOD_TIMEUNIT_PCMFRACTION      : FmodTimeUnit = FmodTimeUnit(0x00000010);
/// MOD/S3M/XM/IT. Order in a sequenced module format. Use [`Sound::get_format`](../struct.Sound.html#method.get_format) to determine the PCM format being decoded to.
pub const FMOD_TIMEUNIT_MODORDER         : FmodTimeUnit = FmodTimeUnit(0x00000100);
/// MOD/S3M/XM/IT. Current row in a sequenced module format. [`Sound::get_length`](../struct.Sound.html#method.get_length) will return the number of rows in the currently playing or seeked to pattern.
pub const FMOD_TIMEUNIT_MODROW           : FmodTimeUnit = FmodTimeUnit(0x00000200);
/// MOD/S3M/XM/IT. Current pattern in a sequenced module format. [`Sound::get_length`](../struct.Sound.html#method.get_length) will return the number of patterns in the song and [`Channel::get_position`](../struct.Channel.html#method.get_position) will return the currently playing pattern.
pub const FMOD_TIMEUNIT_MODPATTERN       : FmodTimeUnit = FmodTimeUnit(0x00000400);
/// Currently playing subsound in a sentence time in milliseconds.
pub const FMOD_TIMEUNIT_SENTENCE_MS      : FmodTimeUnit = FmodTimeUnit(0x00010000);
/// Currently playing subsound in a sentence time in PCM Samples, related to milliseconds * samplerate / 1000.
pub const FMOD_TIMEUNIT_SENTENCE_PCM     : FmodTimeUnit = FmodTimeUnit(0x00020000);
/// Currently playing subsound in a sentence time in bytes, related to PCM samples * channels * datawidth (ie 16bit = 2 bytes).
pub const FMOD_TIMEUNIT_SENTENCE_PCMBYTES: FmodTimeUnit = FmodTimeUnit(0x00040000);
/// Currently playing sentence index according to the channel.
pub const FMOD_TIMEUNIT_SENTENCE         : FmodTimeUnit = FmodTimeUnit(0x00080000);
/// Currently playing subsound index in a sentence.
pub const FMOD_TIMEUNIT_SENTENCE_SUBSOUND: FmodTimeUnit = FmodTimeUnit(0x00100000);
/// Time value as seen by buffered stream. This is always ahead of audible time, and is only used for processing.
pub const FMOD_TIMEUNIT_BUFFERED         : FmodTimeUnit = FmodTimeUnit(0x10000000);

/// Memory not accounted for by other types
pub const FMOD_MEMBITS_OTHER             : FmodMemoryBits = FmodMemoryBits(0x00000001);
/// String data
pub const FMOD_MEMBITS_STRING            : FmodMemoryBits = FmodMemoryBits(0x00000002);
/// [`FmodSys`](../struct.FmodSys.html) object and various internals
pub const FMOD_MEMBITS_SYSTEM            : FmodMemoryBits = FmodMemoryBits(0x00000004);
/// Plugin objects and internals
pub const FMOD_MEMBITS_PLUGINS           : FmodMemoryBits = FmodMemoryBits(0x00000008);
/// Output module object and internals
pub const FMOD_MEMBITS_OUTPUT            : FmodMemoryBits = FmodMemoryBits(0x00000010);
/// [`Channel`](../struct.Channel.html) related memory
pub const FMOD_MEMBITS_CHANNEL           : FmodMemoryBits = FmodMemoryBits(0x00000020);
/// [`ChannelGroup`](../struct.ChannelGroup.html) objects and internals
pub const FMOD_MEMBITS_CHANNELGROUP      : FmodMemoryBits = FmodMemoryBits(0x00000040);
/// Codecs allocated for streaming
pub const FMOD_MEMBITS_CODEC             : FmodMemoryBits = FmodMemoryBits(0x00000080);
/// Codecs allocated for streaming
pub const FMOD_MEMBITS_FILE              : FmodMemoryBits = FmodMemoryBits(0x00000100);
/// [`Sound`](../struct.Sound.html) objects and internals
pub const FMOD_MEMBITS_SOUND             : FmodMemoryBits = FmodMemoryBits(0x00000200);
/// Sound data stored in secondary RAM
pub const FMOD_MEMBITS_SOUND_SECONDARYRAM: FmodMemoryBits = FmodMemoryBits(0x00000400);
/// [`SoundGroup`](../struct.SoundGroup.html) objects and internals
pub const FMOD_MEMBITS_SOUNDGROUP        : FmodMemoryBits = FmodMemoryBits(0x00000800);
/// Stream buffer memory
pub const FMOD_MEMBITS_STREAMBUFFER      : FmodMemoryBits = FmodMemoryBits(0x00001000);
/// [`DspConnection`](../struct.DspConnection.html) objects and internals
pub const FMOD_MEMBITS_DSPCONNECTION     : FmodMemoryBits = FmodMemoryBits(0x00002000);
/// [`Dsp`](../struct.Dsp.html) implementation objects
pub const FMOD_MEMBITS_DSP               : FmodMemoryBits = FmodMemoryBits(0x00004000);
/// Realtime file format decoding [`Dsp`](../struct.Dsp.html) objects
pub const FMOD_MEMBITS_DSPCODEC          : FmodMemoryBits = FmodMemoryBits(0x00008000);
/// Profiler memory footprint.
pub const FMOD_MEMBITS_PROFILE           : FmodMemoryBits = FmodMemoryBits(0x00010000);
/// Buffer used to store recorded data from microphone
pub const FMOD_MEMBITS_RECORDBUFFER      : FmodMemoryBits = FmodMemoryBits(0x00020000);
/// [`Reverb`](../struct.Reverb.html) implementation objects
pub const FMOD_MEMBITS_REVERB            : FmodMemoryBits = FmodMemoryBits(0x00040000);
/// Reverb channel properties structs
pub const FMOD_MEMBITS_REVERBCHANNELPROPS: FmodMemoryBits = FmodMemoryBits(0x00080000);
/// [`Geometry`](../struct.Geometry.html) objects and internals
pub const FMOD_MEMBITS_GEOMETRY          : FmodMemoryBits = FmodMemoryBits(0x00100000);
/// Sync point memory.
pub const FMOD_MEMBITS_SYNCPOINT         : FmodMemoryBits = FmodMemoryBits(0x00200000);
/// All memory used by FMOD Ex
pub const FMOD_MEMBITS_ALL               : FmodMemoryBits = FmodMemoryBits(0xffffffff);

/// EventSystem and various internals
pub const FMOD_EVENT_MEMBITS_EVENTSYSTEM          : u32 = 0x00000001;
/// MusicSystem and various internals
pub const FMOD_EVENT_MEMBITS_MUSICSYSTEM          : u32 = 0x00000002;
/// Definition of objects contained in all loaded projects e.g. events, groups, categories
pub const FMOD_EVENT_MEMBITS_FEV                  : u32 = 0x00000004;
/// Data loaded with preloadFSB
pub const FMOD_EVENT_MEMBITS_MEMORYFSB            : u32 = 0x00000008;
/// EventProject objects and internals
pub const FMOD_EVENT_MEMBITS_EVENTPROJECT         : u32 = 0x00000010;
/// EventGroup objects and internals
pub const FMOD_EVENT_MEMBITS_EVENTGROUPI          : u32 = 0x00000020;
/// Objects used to manage wave banks
pub const FMOD_EVENT_MEMBITS_SOUNDBANKCLASS       : u32 = 0x00000040;
/// Data used to manage lists of wave bank usage
pub const FMOD_EVENT_MEMBITS_SOUNDBANKLIST        : u32 = 0x00000080;
/// Stream objects and internals
pub const FMOD_EVENT_MEMBITS_STREAMINSTANCE       : u32 = 0x00000100;
/// Sound definition objects
pub const FMOD_EVENT_MEMBITS_SOUNDDEFCLASS        : u32 = 0x00000200;
/// Sound definition static data objects
pub const FMOD_EVENT_MEMBITS_SOUNDDEFDEFCLASS     : u32 = 0x00000400;
/// Sound definition pool data
pub const FMOD_EVENT_MEMBITS_SOUNDDEFPOOL         : u32 = 0x00000800;
/// Reverb definition objects
pub const FMOD_EVENT_MEMBITS_REVERBDEF            : u32 = 0x00001000;
/// Reverb objects
pub const FMOD_EVENT_MEMBITS_EVENTREVERB          : u32 = 0x00002000;
/// User property objects
pub const FMOD_EVENT_MEMBITS_USERPROPERTY         : u32 = 0x00004000;
/// Event instance base objects
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCE        : u32 = 0x00008000;
/// Complex event instance objects
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCE_COMPLEX: u32 = 0x00010000;
/// Simple event instance objects
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCE_SIMPLE : u32 = 0x00020000;
/// Event layer instance objects
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCE_LAYER  : u32 = 0x00040000;
/// Event sound instance objects
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCE_SOUND  : u32 = 0x00080000;
/// Event envelope objects
pub const FMOD_EVENT_MEMBITS_EVENTENVELOPE        : u32 = 0x00100000;
/// Event envelope definition objects
pub const FMOD_EVENT_MEMBITS_EVENTENVELOPEDEF     : u32 = 0x00200000;
/// Event parameter objects
pub const FMOD_EVENT_MEMBITS_EVENTPARAMETER       : u32 = 0x00400000;
/// Event category objects
pub const FMOD_EVENT_MEMBITS_EVENTCATEGORY        : u32 = 0x00800000;
/// Event envelope point objects
pub const FMOD_EVENT_MEMBITS_EVENTENVELOPEPOINT   : u32 = 0x01000000;
/// Event instance pool data
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCEPOOL    : u32 = 0x02000000;
/// All memory used by FMOD Event System
pub const FMOD_EVENT_MEMBITS_ALL                  : u32 = 0xffffffff;

/// All event instance memory
pub const FMOD_EVENT_MEMBITS_EVENTINSTANCE_GROUP  : u32 = FMOD_EVENT_MEMBITS_EVENTINSTANCE | FMOD_EVENT_MEMBITS_EVENTINSTANCE_COMPLEX | FMOD_EVENT_MEMBITS_EVENTINSTANCE_SIMPLE | FMOD_EVENT_MEMBITS_EVENTINSTANCE_LAYER | FMOD_EVENT_MEMBITS_EVENTINSTANCE_SOUND;
/// All sound definition memory
pub const FMOD_EVENT_MEMBITS_SOUNDDEF_GROUP       : u32 = FMOD_EVENT_MEMBITS_SOUNDDEFCLASS | FMOD_EVENT_MEMBITS_SOUNDDEFDEFCLASS | FMOD_EVENT_MEMBITS_SOUNDDEFPOOL;

#[cfg(target_os = "linux")]
mod platform {
    #[cfg(target_arch="x86")]
    #[link(name = "fmodex")] extern{}
    #[cfg(target_arch="x86_64")]
    #[link(name = "fmodex64")] extern{}
}

#[cfg(target_os = "macos")]
mod platform {
    #[link(name = "fmodex")] extern{}
}
