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

use libc::{c_void, c_uint, c_int, c_char, c_float};
use ffi;
use types::*;
use enums::*;
use sound;
use sound::Sound;
use sound_group;
use std::mem;
use channel_group;
use channel;

pub struct FmodGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8, ..8] 
}

pub struct FmodSoftwareFormat {
    pub sample_rate         : i32,
    pub format              : FMOD_SOUND_FORMAT,
    pub num_output_channels : i32,
    pub max_input_channels  : i32,
    pub resample_method     : FMOD_DSP_RESAMPLER,
    pub bits                : i32
}

pub struct FmodAdvancedSettings
{
    pub max_MPEG_codecs               : i32,
    pub max_ADPCM_codecs              : i32,
    pub max_XMA_codecs                : i32,
    pub max_CELT_codecs               : i32,
    pub max_VORBIS_codecs             : i32,
    pub max_AT9_codecs                : i32,
    pub max_PCM_codecs                : i32,
    pub ASIO_num_channels             : i32,
    pub ASIO_channel_list             : Vec<StrBuf>,
    pub ASIO_speaker_list             : Vec<FMOD_SPEAKER>,
    pub max_3D_reverb_DSPs            : i32,
    pub HRTF_min_angle                : f32,
    pub HRTF_max_angle                : f32,
    pub HRTF_freq                     : f32,
    pub vol0_virtual_vol              : f32,
    pub event_queue_size              : i32,
    pub default_decode_buffer_size    : u32,
    pub debug_log_filename            : StrBuf,
    pub profile_port                  : u16,
    pub geometry_max_fade_time        : u32,
    pub max_spectrum_wave_data_buffers: u32,
    pub music_system_cache_delay      : u32,
    pub distance_filter_center_freq   : f32,
    pub stack_size_stream             : u32,
    pub stack_size_non_blocking       : u32,
    pub stack_size_mixer              : u32
}

pub struct FmodReverbProperties
{                                   /*       MIN    MAX     DEFAULT DESCRIPTION */
    pub instance         : i32,     /* [w]   0      3       0       Environment Instance.                                                 (SUPPORTED:SFX(4 instances) and Wii (3 instances)) */
    pub environment      : i32,     /* [r/w] -1     25      -1      Sets all listener properties.  -1 = OFF.                              (SUPPORTED:SFX(-1 only)/PSP) */
    pub env_diffusion    : f32,     /* [r/w] 0.0    1.0     1.0     Environment diffusion                                                 (SUPPORTED:WII) */
    pub room             : i32,     /* [r/w] -10000 0       -1000   Room effect level (at mid frequencies)                                (SUPPORTED:SFX/WII/PSP) */
    pub room_HF          : i32,     /* [r/w] -10000 0       -100    Relative room effect level at high frequencies                        (SUPPORTED:SFX) */
    pub room_LF          : i32,     /* [r/w] -10000 0       0       Relative room effect level at low frequencies                         (SUPPORTED:SFX) */
    pub decay_time       : f32,     /* [r/w] 0.1    20.0    1.49    Reverberation decay time at mid frequencies                           (SUPPORTED:SFX/WII) */
    pub decay_HF_ratio   : f32,     /* [r/w] 0.1    2.0     0.83    High-frequency to mid-frequency decay time ratio                      (SUPPORTED:SFX) */
    pub decay_LF_ratio   : f32,     /* [r/w] 0.1    2.0     1.0     Low-frequency to mid-frequency decay time ratio                       (SUPPORTED:---) */
    pub reflections      : i32,     /* [r/w] -10000 1000    -2602   Early reflections level relative to room effect                       (SUPPORTED:SFX/WII) */
    pub reflections_delay: f32,     /* [r/w] 0.0    0.3     0.007   Initial reflection delay time                                         (SUPPORTED:SFX) */
    pub reverb           : i32,     /* [r/w] -10000 2000    200     Late reverberation level relative to room effect                      (SUPPORTED:SFX) */
    pub reverb_delay     : f32,     /* [r/w] 0.0    0.1     0.011   Late reverberation delay time relative to initial reflection          (SUPPORTED:SFX/WII) */
    pub modulation_time  : f32,     /* [r/w] 0.04   4.0     0.25    Modulation time                                                       (SUPPORTED:---) */
    pub modulation_depth : f32,     /* [r/w] 0.0    1.0     0.0     Modulation depth                                                      (SUPPORTED:WII) */
    pub HF_reference     : f32,     /* [r/w] 20.0   20000.0 5000.0  Reference high frequency (hz)                                         (SUPPORTED:SFX) */
    pub LF_reference     : f32,     /* [r/w] 20.0   1000.0  250.0   Reference low frequency (hz)                                          (SUPPORTED:SFX) */
    pub diffusion        : f32,     /* [r/w] 0.0    100.0   100.0   Value that controls the echo density in the late reverberation decay. (SUPPORTED:SFX) */
    pub density          : f32,     /* [r/w] 0.0    100.0   100.0   Value that controls the modal density in the late reverberation decay (SUPPORTED:SFX) */
    pub flags            : u32      /* [r/w] FMOD_REVERB_FLAGS - modifies the behavior of above properties                                (SUPPORTED:WII) */
}

impl FmodReverbProperties {
    fn new() -> FmodReverbProperties {
        FmodReverbProperties{instance: 0i32, environment: 0i32, env_diffusion: 0f32, room: 0i32, room_HF: 0i32, room_LF: 0i32, decay_time: 0f32,
            decay_HF_ratio: 0f32, decay_LF_ratio: 0f32, reflections: 0i32, reflections_delay: 0f32, reverb: 0i32, reverb_delay: 0f32, modulation_time: 0f32,
            modulation_depth: 0f32, HF_reference: 0f32, LF_reference: 0f32, diffusion: 0f32, density: 0f32, flags: 0u32}
    }

    fn from_ptr(pointer: ffi::FMOD_REVERB_PROPERTIES) -> FmodReverbProperties {
        FmodReverbProperties{instance: pointer.Instance, environment: pointer.Environment, env_diffusion: pointer.EnvDiffusion, room: pointer.Room,
            room_HF: pointer.RoomHF, room_LF: pointer.RoomLF, decay_time: pointer.DecayTime, decay_HF_ratio: pointer.DecayHFRatio, decay_LF_ratio: pointer.DecayLFRatio,
            reflections: pointer.Reflections, reflections_delay: pointer.ReflectionsDelay, reverb: pointer.Reverb, reverb_delay: pointer.ReverbDelay,
            modulation_time: pointer.ModulationTime, modulation_depth: pointer.ModulationDepth, HF_reference: pointer.HFReference, LF_reference: pointer.LFReference,
            diffusion: pointer.Diffusion, density: pointer.Density, flags: pointer.Flags}
    }

    fn convert_to_c(&self) -> ffi::FMOD_REVERB_PROPERTIES {
        ffi::FMOD_REVERB_PROPERTIES{Instance: self.instance, Environment: self.environment, EnvDiffusion: self.env_diffusion, Room: self.room,
            RoomHF: self.room_HF, RoomLF: self.room_LF, DecayTime: self.decay_time, DecayHFRatio: self.decay_HF_ratio, DecayLFRatio: self.decay_LF_ratio,
            Reflections: self.reflections, ReflectionsDelay: self.reflections_delay, Reverb: self.reverb, ReverbDelay: self.reverb_delay,
            ModulationTime: self.modulation_time, ModulationDepth: self.modulation_depth, HFReference: self.HF_reference, LFReference: self.LF_reference,
            Diffusion: self.diffusion, Density: self.density, Flags: self.flags}
    }
}

pub struct FmodCreateSoundexInfo
{
    pub length              : u32,                       /* [w] Optional. Specify 0 to ignore. Size in bytes of file to load, or sound to create (in this case only if FMOD_OPENUSER is used).  Required if loading from memory.  If 0 is specified, then it will use the size of the file (unless loading from memory then an error will be returned). */
    pub fileoffset          : u32,                       /* [w] Optional. Specify 0 to ignore. Offset from start of the file to start loading from.  This is useful for loading files from inside big data files. */
    pub numchannels         : i32,                        /* [w] Optional. Specify 0 to ignore. Number of channels in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. */
    pub defaultfrequency    : i32,                        /* [w] Optional. Specify 0 to ignore. Default frequency of sound in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the frequency determined by the file format. */
    pub format              : FMOD_SOUND_FORMAT,            /* [w] Optional. Specify 0 or FMOD_SOUND_FORMAT_NONE to ignore. Format of the sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the format determined by the file format.   */
    pub decodebuffersize    : u32,                       /* [w] Optional. Specify 0 to ignore. For streams.  This determines the size of the double buffer (in PCM samples) that a stream uses.  Use this for user created streams if you want to determine the size of the callback buffer passed to you.  Specify 0 to use FMOD's default size which is currently equivalent to 400ms of the sound format created/loaded. */
    pub initialsubsound     : i32,                        /* [w] Optional. Specify 0 to ignore. In a multi-sample file format such as .FSB/.DLS/.SF2, specify the initial subsound to seek to, only if FMOD_CREATESTREAM is used. */
    pub numsubsounds        : i32,                        /* [w] Optional. Specify 0 to ignore or have no subsounds.  In a sound created with FMOD_OPENUSER, specify the number of subsounds that are accessable with Sound::getSubSound.  If not created with FMOD_OPENUSER, this will limit the number of subsounds loaded within a multi-subsound file.  If using FSB, then if FMOD_CREATESOUNDEXINFO::inclusionlist is used, this will shuffle subsounds down so that there are not any gaps.  It will mean that the indices of the sounds will be different. */
    pub inclusionlist       : Vec<i32>,                       /* [w] Optional. Specify 0 to ignore. In a multi-sample format such as .FSB/.DLS/.SF2 it may be desirable to specify only a subset of sounds to be loaded out of the whole file.  This is an array of subsound indices to load into memory when created. */
    pub pcmreadcallback     : ffi::FMOD_SOUND_PCMREADCALLBACK,   /* [w] Optional. Specify 0 to ignore. Callback to 'piggyback' on FMOD's read functions and accept or even write PCM data while FMOD is opening the sound.  Used for user sounds created with FMOD_OPENUSER or for capturing decoded data as FMOD reads it. */
    pub pcmsetposcallback   : ffi::FMOD_SOUND_PCMSETPOSCALLBACK, /* [w] Optional. Specify 0 to ignore. Callback for when the user calls a seeking function such as Channel::setTime or Channel::setPosition within a multi-sample sound, and for when it is opened.*/
    pub nonblockcallback    : ffi::FMOD_SOUND_NONBLOCKCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for successful completion, or error while loading a sound that used the FMOD_NONBLOCKING flag.  Also called duing seeking, when setPosition is called or a stream is restarted. */
    pub dlsname             : StrBuf,                      /* [w] Optional. Specify 0 to ignore. Filename for a DLS or SF2 sample set when loading a MIDI file. If not specified, on Windows it will attempt to open /windows/system32/drivers/gm.dls or /windows/system32/drivers/etc/gm.dls, on Mac it will attempt to load /System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls, otherwise the MIDI will fail to open. Current DLS support is for level 1 of the specification. */
    pub encryptionkey       : StrBuf,                      /* [w] Optional. Specify 0 to ignore. Key for encrypted FSB file.  Without this key an encrypted FSB file will not load. */
    pub maxpolyphony        : i32,                        /* [w] Optional. Specify 0 to ignore. For sequenced formats with dynamic channel allocation such as .MID and .IT, this specifies the maximum voice count allowed while playing.  .IT defaults to 64.  .MID defaults to 32. */
    userdata                : *c_void,                      /* [w] Optional. Specify 0 to ignore. This is user data to be attached to the sound during creation.  Access via Sound::getUserData.  Note: This is not passed to FMOD_FILE_OPENCALLBACK, that is a different userdata that is file specific. */
    pub suggestedsoundtype  : FMOD_SOUND_TYPE,              /* [w] Optional. Specify 0 or FMOD_SOUND_TYPE_UNKNOWN to ignore.  Instead of scanning all codec types, use this to speed up loading by making it jump straight to this codec. */
    pub useropen            : ffi::FMOD_FILE_OPENCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for opening this file. */
    pub userclose           : ffi::FMOD_FILE_CLOSECALLBACK,      /* [w] Optional. Specify 0 to ignore. Callback for closing this file. */
    pub userread            : ffi::FMOD_FILE_READCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for reading from this file. */
    pub userseek            : ffi::FMOD_FILE_SEEKCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub userasyncread       : ffi::FMOD_FILE_ASYNCREADCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub userasynccancel     : ffi::FMOD_FILE_ASYNCCANCELCALLBACK,/* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    pub speakermap          : FMOD_SPEAKERMAPTYPE,          /* [w] Optional. Specify 0 to ignore. Use this to differ the way fmod maps multichannel sounds to speakers.  See FMOD_SPEAKERMAPTYPE for more. */
    pub initialsoundgroup   : sound_group::SoundGroup,             /* [w] Optional. Specify 0 to ignore. Specify a sound group if required, to put sound in as it is created. */
    pub initialseekposition : u32,                       /* [w] Optional. Specify 0 to ignore. For streams. Specify an initial position to seek the stream to. */
    pub initialseekpostype  : FmodTimeUnit,                /* [w] Optional. Specify 0 to ignore. For streams. Specify the time unit for the position set in initialseekposition. */
    pub ignoresetfilesystem : i32,                        /* [w] Optional. Specify 0 to ignore. Set to 1 to use fmod's built in file system. Ignores setFileSystem callbacks and also FMOD_CREATESOUNEXINFO file callbacks.  Useful for specific cases where you don't want to use your own file system but want to use fmod's file system (ie net streaming). */
    pub cddaforceaspi       : i32,                        /* [w] Optional. Specify 0 to ignore. For CDDA sounds only - if non-zero use ASPI instead of NTSCSI to access the specified CD/DVD device. */
    pub audioqueuepolicy    : u32,                       /* [w] Optional. Specify 0 or FMOD_AUDIOQUEUE_CODECPOLICY_DEFAULT to ignore. Policy used to determine whether hardware or software is used for decoding, see FMOD_AUDIOQUEUE_CODECPOLICY for options (iOS >= 3.0 required, otherwise only hardware is available) */ 
    pub minmidigranularity  : u32,                       /* [w] Optional. Specify 0 to ignore. Allows you to set a minimum desired MIDI mixer granularity. Values smaller than 512 give greater than default accuracy at the cost of more CPU and vice versa. Specify 0 for default (512 samples). */
    pub nonblockthreadid    : i32                        /* [w] Optional. Specify 0 to ignore. Specifies a thread index to execute non blocking load on.  Allows for up to 5 threads to be used for loading at once.  This is to avoid one load blocking another.  Maximum value = 4. */
}

impl FmodCreateSoundexInfo {
    pub fn new() -> FmodCreateSoundexInfo {
        FmodCreateSoundexInfo{length: 0, fileoffset: 0, numchannels: 0, defaultfrequency: 0, format: FMOD_SOUND_FORMAT_NONE, decodebuffersize: 0,
            initialsubsound: 0, numsubsounds: 0, inclusionlist: Vec::new(), pcmreadcallback: None,
            pcmsetposcallback: None, nonblockcallback: None, dlsname: StrBuf::new(), encryptionkey: StrBuf::new(),
            maxpolyphony: 0, userdata: ::std::ptr::null(), suggestedsoundtype: FMOD_SOUND_TYPE_UNKNOWN, useropen: None,
            userclose: None, userread: None, userseek: None, userasynccancel: None,
            userasyncread: None, speakermap: FMOD_SPEAKERMAPTYPE_DEFAULT, initialsoundgroup: sound_group::from_ptr(::std::ptr::null()),
            initialseekposition: 0, initialseekpostype: FMOD_TIMEUNIT_MS, ignoresetfilesystem: 0, cddaforceaspi: 0, audioqueuepolicy: 0,
            minmidigranularity: 0, nonblockthreadid: 0}
    }

    fn convert_to_c(&self) -> ffi::FMOD_CREATESOUNDEXINFO {
        ffi::FMOD_CREATESOUNDEXINFO{cbsize: mem::size_of::<ffi::FMOD_CREATESOUNDEXINFO>() as i32, length: self.length, fileoffset: self.fileoffset,
            numchannels: self.numchannels, defaultfrequency: self.defaultfrequency, format: self.format, decodebuffersize: self.decodebuffersize,
            initialsubsound: self.initialsubsound, numsubsounds: self.numsubsounds, inclusionlist: self.inclusionlist.as_ptr(),
            inclusionlistnum: self.inclusionlist.len() as i32, pcmreadcallback: self.pcmreadcallback, pcmsetposcallback: self.pcmsetposcallback,
            nonblockcallback: self.nonblockcallback, dlsname: self.dlsname.clone().into_owned().with_c_str(|c_str|{c_str}),
            encryptionkey: self.encryptionkey.clone().into_owned().with_c_str(|c_str|{c_str}), maxpolyphony: self.maxpolyphony, userdata: self.userdata,
            suggestedsoundtype: self.suggestedsoundtype, useropen: self.useropen, userclose: self.userclose, userread: self.userread,
            userseek: self.userseek, userasynccancel: self.userasynccancel, userasyncread: self.userasyncread, speakermap: self.speakermap,
            initialsoundgroup: sound_group::get_ffi(self.initialsoundgroup), initialseekposition: self.initialseekposition,
            initialseekpostype: match self.initialseekpostype {
                FmodTimeUnit(v) => v},
            ignoresetfilesystem: self.ignoresetfilesystem, cddaforceaspi: self.cddaforceaspi, audioqueuepolicy: self.audioqueuepolicy,
            minmidigranularity: self.minmidigranularity, nonblockthreadid: self.nonblockthreadid}
    }

    fn from_ptr(ptr: ffi::FMOD_CREATESOUNDEXINFO) -> FmodCreateSoundexInfo {
        let mut inc : *mut i32 = ptr.inclusionlist as *mut i32;

        FmodCreateSoundexInfo{length: ptr.length, fileoffset: ptr.fileoffset, numchannels: ptr.numchannels, defaultfrequency: ptr.defaultfrequency,
            format: ptr.format, decodebuffersize: ptr.decodebuffersize, initialsubsound: ptr.initialsubsound, numsubsounds: ptr.numsubsounds,
            inclusionlist: unsafe { Vec::from_raw_parts(ptr.inclusionlistnum as uint, ptr.inclusionlistnum as uint, inc).clone() },
            pcmreadcallback: ptr.pcmreadcallback,
            pcmsetposcallback: ptr.pcmsetposcallback, nonblockcallback: ptr.nonblockcallback,
            dlsname: if ptr.dlsname != ::std::ptr::null() {
                    StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(ptr.dlsname) }).clone()
                } else {
                    StrBuf::new()
                },
            encryptionkey: if ptr.encryptionkey != ::std::ptr::null() {
                    StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(ptr.encryptionkey) }).clone()
                } else {
                    StrBuf::new()
                },
            maxpolyphony: ptr.maxpolyphony, userdata: ptr.userdata, suggestedsoundtype: ptr.suggestedsoundtype, useropen: ptr.useropen,
            userclose: ptr.userclose, userread: ptr.userread, userseek: ptr.userseek, userasynccancel: ptr.userasynccancel,
            userasyncread: ptr.userasyncread, speakermap: ptr.speakermap, initialsoundgroup: sound_group::from_ptr(ptr.initialsoundgroup),
            initialseekposition: ptr.initialseekposition as u32, initialseekpostype: FmodTimeUnit(ptr.initialseekpostype), ignoresetfilesystem: ptr.ignoresetfilesystem,
            cddaforceaspi: ptr.cddaforceaspi, audioqueuepolicy: ptr.audioqueuepolicy, minmidigranularity: ptr.minmidigranularity,
            nonblockthreadid: ptr.nonblockthreadid}
    }
}

pub struct FmodCodecDescription {
    pub name            : StrBuf,                             /* [in] Name of the codec. */
    pub version         : u32,                                /* [in] Plugin writer's version number. */
    pub defaultasstream : i32,                                /* [in] Tells FMOD to open the file as a stream when calling System::createSound, and not a static sample.  Should normally be 0 (FALSE), because generally the user wants to decode the file into memory when using System::createSound.   Mainly used for formats that decode for a very long time, or could use large amounts of memory when decoded.  Usually sequenced formats such as mod/s3m/xm/it/midi fall into this category.   It is mainly to stop users that don't know what they're doing from getting FMOD_ERR_MEMORY returned from createSound when they should have in fact called System::createStream or used FMOD_CREATESTREAM in System::createSound. */
    pub timeunits       : FmodTimeUnit,                       /* [in] When setposition codec is called, only these time formats will be passed to the codec. Use bitwise OR to accumulate different types. */
    pub open            : ffi::FMOD_CODEC_OPENCALLBACK,       /* [in] Open callback for the codec for when FMOD tries to open a sound using this codec. */
    pub close           : ffi::FMOD_CODEC_CLOSECALLBACK,      /* [in] Close callback for the codec for when FMOD tries to close a sound using this codec.  */
    pub read            : ffi::FMOD_CODEC_READCALLBACK,       /* [in] Read callback for the codec for when FMOD tries to read some data from the file to the destination format (specified in the open callback). */
    pub getlength       : ffi::FMOD_CODEC_GETLENGTHCALLBACK,  /* [in] Callback to return the length of the song in whatever format required when Sound::getLength is called. */
    pub setposition     : ffi::FMOD_CODEC_SETPOSITIONCALLBACK,/* [in] Seek callback for the codec for when FMOD tries to seek within the file with Channel::setPosition. */
    pub getposition     : ffi::FMOD_CODEC_GETPOSITIONCALLBACK,/* [in] Tell callback for the codec for when FMOD tries to get the current position within the with Channel::getPosition. */
    pub soundcreate     : ffi::FMOD_CODEC_SOUNDCREATECALLBACK,/* [in] Sound creation callback for the codec when FMOD finishes creating the sound.  (So the codec can set more parameters for the related created sound, ie loop points/mode or 3D attributes etc). */
    pub getwaveformat   : ffi::FMOD_CODEC_GETWAVEFORMAT       /* [in] Callback to tell FMOD about the waveformat of a particular subsound.  This is to save memory, rather than saving 1000 FMOD_CODEC_WAVEFORMAT structures in the codec, the codec might have a more optimal way of storing this information. */
}

pub struct FmodOutputHandle {
    handle: *c_void
}

pub struct FmodGeometry {
    geometry: ffi::FMOD_GEOMETRY
}

pub struct FmodVector
{
    pub x: f32, /* X co-ordinate in 3D space. */
    pub y: f32, /* Y co-ordinate in 3D space. */
    pub z: f32  /* Z co-ordinate in 3D space. */
}

pub fn from_c(vec : ffi::FMOD_VECTOR) -> FmodVector {
    FmodVector{x: vec.x, y: vec.y, z: vec.z}
}

impl FmodVector {
    pub fn new() -> FmodVector {
        FmodVector{x: 0f32, y: 0f32, z: 0f32}
    }

    pub fn convert_to_c(&self) -> ffi::FMOD_VECTOR {
        ffi::FMOD_VECTOR{x: self.x, y: self.y, z: self.z}
    }
}

pub struct FmodSys {
    system : ffi::FMOD_SYSTEM,
}

impl FmodSys {
    pub fn new() -> Result<FmodSys, FMOD_RESULT> {
        let tmp = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_Create(&tmp) } {
            FMOD_OK => Ok(FmodSys{system : tmp}),
            err => Err(err)
        }
    }

    pub fn init(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Init(self.system, 1, FMOD_INIT_NORMAL, ::std::ptr::null()) }
    }

    pub fn init_with_parameters(&self, max_channels: i32, FmodInitFlag(flag) : FmodInitFlag) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Init(self.system, max_channels, flag, ::std::ptr::null()) }
    }

    pub fn update(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Update(self.system) }
    }

    pub fn release(&self) -> FMOD_RESULT {
        unsafe {
            ffi::FMOD_System_Close(self.system);
            ffi::FMOD_System_Release(self.system)
        }
    }

    pub fn create_sound(&self, music : StrBuf, options: Option<FmodMode>, exinfo: Option<FmodCreateSoundexInfo>) -> Result<Sound, FMOD_RESULT> {
        let tmp_v = music.clone().into_owned();
        let sound = ::std::ptr::null();
        let op = match options {
            Some(FmodMode(t)) => t,
            None => FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM
        };
        let ex = match exinfo {
            Some(e) => &e.convert_to_c() as *ffi::FMOD_CREATESOUNDEXINFO,
            None => ::std::ptr::null()
        };

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateSound(self.system, if tmp_v.len() > 0 {
                    c_str
                } else {
                    ::std::ptr::null()
                }, op, ex, &sound) } {
                FMOD_OK => {Ok(sound::Sound::from_ptr(sound))},
                err => Err(err)
            }
        })
    }

    pub fn create_stream(&self, music : StrBuf, options: Option<FmodMode>) -> Result<Sound, FMOD_RESULT> {
        let tmp_v = music.clone().into_owned();
        let sound = ::std::ptr::null();
        let op = match options {
            Some(FmodMode(t)) => t,
            None => FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM
        };

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateStream(self.system, c_str, op, ::std::ptr::null(), &sound) } {
                FMOD_OK => {Ok(sound::Sound::from_ptr(sound))},
                err => Err(err)
            }
        })
    }

    pub fn create_channel_group(&self, group_name : StrBuf) -> Result<channel_group::ChannelGroup, FMOD_RESULT> {
        let t_group_name = group_name.clone().into_owned();
        let channel_group = ::std::ptr::null();

        t_group_name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateChannelGroup(self.system, c_str, &channel_group) } {
                FMOD_OK => Ok(channel_group::from_ptr(channel_group)),
                e => Err(e)
            }
        })
    }

    pub fn create_sound_group(&self, group_name : StrBuf) -> Result<sound_group::SoundGroup, FMOD_RESULT> {
        let t_group_name = group_name.clone().into_owned();
        let sound_group = ::std::ptr::null();

        t_group_name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_CreateSoundGroup(self.system, c_str, &sound_group) } {
                FMOD_OK => Ok(sound_group::from_ptr(sound_group)),
                e => Err(e)
            }
        })
    }

    pub fn set_output(&self, output_type: FMOD_OUTPUTTYPE) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetOutput(self.system, output_type) }
    }

    pub fn get_output(&self) -> Result<FMOD_OUTPUTTYPE, FMOD_RESULT> {
        let output_type = FMOD_OUTPUTTYPE_AUTODETECT;
        
        match unsafe { ffi::FMOD_System_GetOutput(self.system, &output_type) } {
            FMOD_OK => Ok(output_type),
            e => Err(e)
        }
    }

    pub fn get_num_drivers(&self) -> Result<i32, FMOD_RESULT> {
        let num_drivers = 0i32;

        match unsafe { ffi::FMOD_System_GetNumDrivers(self.system, &num_drivers) } {
            FMOD_OK => Ok(num_drivers),
            e => Err(e)
        }
    }

    pub fn get_driver_info(&self, id: i32, name_len: uint) -> Result<(FmodGuid, StrBuf), FMOD_RESULT> {
        let tmp_v = StrBuf::with_capacity(name_len).into_owned();
        let guid = ffi::FMOD_GUID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetDriverInfo(self.system, id, c_str, name_len as i32, &guid) } {
                FMOD_OK => Ok((FmodGuid{data1: guid.Data1, data2: guid.Data2, data3: guid.Data3, data4: guid.Data4},
                    StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_str) }).clone())),
                e => Err(e)
            }
        })
    }

    pub fn get_driver_caps(&self, id: i32) -> Result<(FmodCaps, i32, FMOD_SPEAKERMODE), FMOD_RESULT> {
        let fmod_caps = 0u32;
        let speaker_mode = FMOD_SPEAKERMODE_RAW;
        let control_panel_output_rate = 0i32;

        match unsafe { ffi::FMOD_System_GetDriverCaps(self.system, id, &fmod_caps, &control_panel_output_rate, &speaker_mode) } {
            FMOD_OK => Ok((FmodCaps(fmod_caps), control_panel_output_rate, speaker_mode)),
            e => Err(e)
        }
    }

    pub fn set_driver(&self, driver: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetDriver(self.system, driver) }
    }

    pub fn get_driver(&self) -> Result<i32, FMOD_RESULT> {
        let driver = 0i32;

        match unsafe { ffi::FMOD_System_GetDriver(self.system, &driver) } {
            FMOD_OK => Ok(driver),
            e => Err(e)
        }
    }

    pub fn set_hardware_channels(&self, num_hardware_channels: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetHardwareChannels(self.system, num_hardware_channels) }
    }

    pub fn get_hardware_channels(&self) -> Result<i32, FMOD_RESULT> {
        let num_hardware_channels = 0i32;

        match unsafe { ffi::FMOD_System_GetHardwareChannels(self.system, &num_hardware_channels) } {
            FMOD_OK => Ok(num_hardware_channels),
            e => Err(e)
        }
    }

    pub fn set_software_channels(&self, num_software_channels: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSoftwareChannels(self.system, num_software_channels) }
    }

    pub fn get_software_channels(&self) -> Result<i32, FMOD_RESULT> {
        let num_software_channels = 0i32;

        match unsafe { ffi::FMOD_System_GetSoftwareChannels(self.system, &num_software_channels) } {
            FMOD_OK => Ok(num_software_channels),
            e => Err(e)
        }
    }

    pub fn set_software_format(&self, sample_rate: i32, format: FMOD_SOUND_FORMAT, num_output_channels: i32,
        max_input_channels: i32, resample_method: FMOD_DSP_RESAMPLER) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSoftwareFormat(self.system, sample_rate, format, num_output_channels,
            max_input_channels, resample_method) }
    }

    pub fn get_software_format(&self) -> Result<FmodSoftwareFormat, FMOD_RESULT> {
        let t = FmodSoftwareFormat{sample_rate: 0, format: FMOD_SOUND_FORMAT_NONE, num_output_channels: 0,
            max_input_channels: 0, resample_method: FMOD_DSP_RESAMPLER_NOINTERP, bits: 0};

        match unsafe { ffi::FMOD_System_GetSoftwareFormat(self.system, &t.sample_rate, &t.format,
            &t.num_output_channels, &t.max_input_channels, &t.resample_method, &t.bits) } {
            FMOD_OK => Ok(t),
            e => Err(e)
        }
    }

    pub fn set_DSP_buffer_size(&self, buffer_length: u32, num_buffers: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetDSPBufferSize(self.system, buffer_length, num_buffers) }
    }

    pub fn get_DSP_buffer_size(&self) -> Result<(u32, i32), FMOD_RESULT> {
        let buffer_length = 0u32;
        let num_buffers = 0i32;

        match unsafe { ffi::FMOD_System_GetDSPBufferSize(self.system, &buffer_length, &num_buffers) } {
            FMOD_OK => Ok((buffer_length, num_buffers)),
            e => Err(e)
        }
    }

    pub fn set_advanced_settings(&self, settings: FmodAdvancedSettings) -> FMOD_RESULT {
        let converted_c_char = Vec::from_fn(settings.ASIO_channel_list.len(), |pos| {
            settings.ASIO_channel_list.get(pos).clone().into_owned().with_c_str(|c_str| c_str)
        });
        let deb_log_filename = settings.debug_log_filename.clone().into_owned();
        let advanced_settings = ffi::FMOD_ADVANCEDSETTINGS{cbsize: mem::size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGcodecs: settings.max_MPEG_codecs, maxADPCMcodecs: settings.max_ADPCM_codecs, maxXMAcodecs: settings.max_XMA_codecs,
            maxCELTcodecs: settings.max_CELT_codecs, maxVORBIScodecs: settings.max_VORBIS_codecs, maxAT9Codecs: settings.max_AT9_codecs,
            maxPCMcodecs: settings.max_PCM_codecs, ASIONumChannels: settings.ASIO_num_channels, ASIOChannelList: converted_c_char.as_ptr(),
            ASIOSpeakerList: settings.ASIO_speaker_list.as_ptr(), max3DReverbDSPs: settings.max_3D_reverb_DSPs, HRTFMinAngle: settings.HRTF_min_angle,
            HRTFMaxAngle: settings.HRTF_max_angle, HRTFFreq: settings.HRTF_freq, vol0virtualvol: settings.vol0_virtual_vol,
            eventqueuesize: settings.event_queue_size, defaultDecodeBufferSize: settings.default_decode_buffer_size,
            debugLogFilename: deb_log_filename.with_c_str(|c_str| c_str), profileport: settings.profile_port,
            geometryMaxFadeTime: settings.geometry_max_fade_time, maxSpectrumWaveDataBuffers: settings.max_spectrum_wave_data_buffers,
            musicSystemCacheDelay: settings.music_system_cache_delay, distanceFilterCenterFreq: settings.distance_filter_center_freq,
            stackSizeStream: settings.stack_size_stream, stackSizeNonBlocking: settings.stack_size_non_blocking, stackSizeMixer: settings.stack_size_mixer};

        unsafe { ffi::FMOD_System_SetAdvancedSettings(self.system, &advanced_settings) }
    }

    pub fn get_advanced_settings(&self) -> Result<FmodAdvancedSettings, FMOD_RESULT> {
        let advanced_settings = ffi::FMOD_ADVANCEDSETTINGS{cbsize: mem::size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGcodecs: 0, maxADPCMcodecs: 0, maxXMAcodecs: 0, maxCELTcodecs: 0, maxVORBIScodecs: 0, maxAT9Codecs: 0,
            maxPCMcodecs: 0, ASIONumChannels: 0, ASIOChannelList: ::std::ptr::null(), ASIOSpeakerList: ::std::ptr::null(),
            max3DReverbDSPs: 0, HRTFMinAngle: 0f32, HRTFMaxAngle: 0f32, HRTFFreq: 0f32, vol0virtualvol: 0f32, eventqueuesize: 0,
            defaultDecodeBufferSize: 0, debugLogFilename: ::std::ptr::null(), profileport: 0, geometryMaxFadeTime: 0,
            maxSpectrumWaveDataBuffers: 0, musicSystemCacheDelay: 0, distanceFilterCenterFreq: 0f32, stackSizeStream: 0, stackSizeNonBlocking: 0,
            stackSizeMixer: 0};

        match unsafe { ffi::FMOD_System_GetAdvancedSettings(self.system, &advanced_settings) } {
            FMOD_OK => {
                let mut converted_ASIO_channel_vec = Vec::new();
                let mut converted_ASIO_speaker_vec = Vec::new();

                if advanced_settings.ASIOChannelList != ::std::ptr::null() {
                    unsafe {::std::ptr::array_each(advanced_settings.ASIOChannelList, |c_str| {
                        converted_ASIO_channel_vec.push(StrBuf::from_owned_str(::std::str::raw::from_c_str(c_str)))
                    })};
                }
                if advanced_settings.ASIOSpeakerList != ::std::ptr::null() {
                    // TODO : I need to check if that closure can segfault !
                    unsafe {::std::ptr::position(advanced_settings.ASIOSpeakerList, |elem| {
                        if elem != &FMOD_SPEAKER_NULL {
                            converted_ASIO_speaker_vec.push(*elem);
                        }
                        elem != &FMOD_SPEAKER_NULL
                    })};
                }
                Ok(FmodAdvancedSettings{max_MPEG_codecs: advanced_settings.maxMPEGcodecs, max_ADPCM_codecs: advanced_settings.maxADPCMcodecs,
                    max_XMA_codecs: advanced_settings.maxXMAcodecs, max_CELT_codecs: advanced_settings.maxCELTcodecs, max_VORBIS_codecs: advanced_settings.maxVORBIScodecs,
                    max_AT9_codecs: advanced_settings.maxAT9Codecs, max_PCM_codecs: advanced_settings.maxPCMcodecs, ASIO_num_channels: advanced_settings.ASIONumChannels,
                    ASIO_channel_list: converted_ASIO_channel_vec.clone(), ASIO_speaker_list: converted_ASIO_speaker_vec, max_3D_reverb_DSPs: advanced_settings.max3DReverbDSPs,
                    HRTF_min_angle: advanced_settings.HRTFMinAngle, HRTF_max_angle: advanced_settings.HRTFMaxAngle, HRTF_freq: advanced_settings.HRTFFreq,
                    vol0_virtual_vol: advanced_settings.vol0virtualvol, event_queue_size: advanced_settings.eventqueuesize, default_decode_buffer_size: advanced_settings.defaultDecodeBufferSize,
                    debug_log_filename: {
                        if advanced_settings.debugLogFilename != ::std::ptr::null() {
                            StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(advanced_settings.debugLogFilename) }).clone()
                        } else {
                            StrBuf::new()
                        }
                    },
                    profile_port: advanced_settings.profileport, geometry_max_fade_time: advanced_settings.geometryMaxFadeTime, max_spectrum_wave_data_buffers: advanced_settings.maxSpectrumWaveDataBuffers,
                    music_system_cache_delay: advanced_settings.musicSystemCacheDelay, distance_filter_center_freq: advanced_settings.distanceFilterCenterFreq,
                    stack_size_stream: advanced_settings.stackSizeStream, stack_size_non_blocking: advanced_settings.stackSizeNonBlocking, stack_size_mixer: advanced_settings.stackSizeMixer})
            }
            e => Err(e)
        }
    }

    pub fn set_speaker_mode(&self, speaker_mode: FMOD_SPEAKERMODE) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetSpeakerMode(self.system, &speaker_mode) }
    }

    pub fn get_speaker_mode(&self) -> Result<FMOD_SPEAKERMODE, FMOD_RESULT> {
        let speaker_mode = FMOD_SPEAKERMODE_RAW;

        match unsafe { ffi::FMOD_System_GetSpeakerMode(self.system, &speaker_mode) } {
            FMOD_OK => Ok(speaker_mode),
            e => Err(e)
        }
    }

    pub fn set_plugin_path(&self, path: StrBuf) -> FMOD_RESULT {
        let tmp_v = path.clone().into_owned();

        tmp_v.with_c_str(|c_str|{
            unsafe { ffi::FMOD_System_SetPluginPath(self.system, c_str) }
        })
    }

    pub fn load_plugin(&self, filename: StrBuf, priority: u32) -> Result<FmodPluginHandle, FMOD_RESULT> {
        let tmp_v = filename.clone().into_owned();
        let handle = 0u32;

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_LoadPlugin(self.system, c_str, &handle, priority) } {
                FMOD_OK => Ok(FmodPluginHandle(handle)),
                e => Err(e)
            }
        })
    }

    pub fn unload_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_UnloadPlugin(self.system, handle) }
    }

    pub fn get_num_plugins(&self, plugin_type: FMOD_PLUGINTYPE) -> Result<i32, FMOD_RESULT> {
        let num_plugins = 0i32;

        match unsafe { ffi::FMOD_System_GetNumPlugins(self.system, plugin_type, &num_plugins) } {
            FMOD_OK => Ok(num_plugins),
            e => Err(e)
        }
    }

    pub fn get_plugin_handle(&self, plugin_type: FMOD_PLUGINTYPE, index: i32) ->Result<FmodPluginHandle, FMOD_RESULT> {
        let handle = 0u32;

        match unsafe { ffi::FMOD_System_GetPluginHandle(self.system, plugin_type, index, &handle) } {
            FMOD_OK => Ok(FmodPluginHandle(handle)),
            e => Err(e)
        }
    }

    pub fn get_plugin_info(&self, FmodPluginHandle(handle): FmodPluginHandle, name_len: u32) -> Result<(StrBuf, FMOD_PLUGINTYPE, u32), FMOD_RESULT> {
        let name = StrBuf::with_capacity(name_len as uint).into_owned();
        let plugin_type = FMOD_PLUGINTYPE_OUTPUT;
        let version = 0u32;

        name.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetPluginInfo(self.system, handle, &plugin_type, c_str, name_len as i32, &version) } {
                FMOD_OK => Ok((StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_str) }).clone(), plugin_type, version)),
                e => Err(e)
            }
        })
    }

    pub fn set_output_by_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetOutputByPlugin(self.system, handle) }
    }

    pub fn get_output_by_plugin(&self) -> Result<FmodPluginHandle, FMOD_RESULT> {
        let handle = 0u32;

        match unsafe { ffi::FMOD_System_GetOutputByPlugin(self.system, &handle) } {
            FMOD_OK => Ok(FmodPluginHandle(handle)),
            e => Err(e)
        }
    }

    pub fn create_DSP_by_plugin(&self, FmodPluginHandle(handle): FmodPluginHandle) -> Result<ffi::FmodDSP, FMOD_RESULT> {
        let dsp = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_CreateDSPByPlugin(self.system, handle, &dsp) } {
            FMOD_OK => Ok(ffi::FmodDSP::from_ptr(dsp)),
            e => Err(e)
        }
    }

    pub fn set_3D_num_listeners(&self, num_listeners : i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Set3DNumListeners(self.system, num_listeners) }
    }

    pub fn get_3D_num_listeners(&self) -> Result<i32, FMOD_RESULT> {
        let num_listeners = 0i32;

        match unsafe { ffi::FMOD_System_Get3DNumListeners(self.system, &num_listeners) } {
            FMOD_OK => Ok(num_listeners),
            e => Err(e)
        }
    }

    pub fn set_3D_listener_attributes(&self, listener: i32, pos: FmodVector, vel: FmodVector, forward: FmodVector,
        up: FmodVector) -> FMOD_RESULT {
        let c_p = pos.convert_to_c();
        let c_v = vel.convert_to_c();
        let c_f = forward.convert_to_c();
        let c_u = up.convert_to_c();

        unsafe { ffi::FMOD_System_Set3DListenerAttributes(self.system, listener, &c_p, &c_v, &c_f, &c_u) }
    }

    pub fn get_3D_listener_attributes(&self, listener: i32) -> Result<(FmodVector, FmodVector, FmodVector, FmodVector), FMOD_RESULT> {
        let pos = FmodVector::new().convert_to_c();
        let vel = FmodVector::new().convert_to_c();
        let forward = FmodVector::new().convert_to_c();
        let up = FmodVector::new().convert_to_c();

        match unsafe { ffi::FMOD_System_Get3DListenerAttributes(self.system, listener, &pos, &vel, &forward, &up) } {
            FMOD_OK => Ok((from_c(pos), from_c(vel), from_c(forward), from_c(up))),
            e => Err(e)
        }
    }

    pub fn set_3D_speaker_position(&self, speaker: FMOD_SPEAKER, x: f32, y: f32, active: bool) -> FMOD_RESULT {
        let t_active = match active {
            true => 1i32,
            false => 0i32
        };
        unsafe { ffi::FMOD_System_Set3DSpeakerPosition(self.system, speaker, x, y, t_active) }
    }

    pub fn get_3D_speaker_position(&self, speaker: FMOD_SPEAKER) -> Result<(f32, f32, bool), FMOD_RESULT> {
        let x = 0f32;
        let y = 0f32;
        let active = 0i32;

        match unsafe { ffi::FMOD_System_Get3DSpeakerPosition(self.system, speaker, &x, &y, &active) } {
            FMOD_OK => Ok((x, y, match active {
                1 => true,
                _ => false
            })),
            e => Err(e)
        }
    }

    pub fn set_3D_settings(&self, doppler_scale: f32, distance_factor: f32, roll_off_scale: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_Set3DSettings(self.system, doppler_scale, distance_factor, roll_off_scale) }
    }

    pub fn get_3D_settings(&self) -> Result<(f32, f32, f32), FMOD_RESULT> {
        let doppler_scale = 0f32;
        let distance_factor = 0f32;
        let roll_off_scale = 0f32;

        match unsafe { ffi::FMOD_System_Get3DSettings(self.system, &doppler_scale, &distance_factor, &roll_off_scale) } {
            FMOD_OK => Ok((doppler_scale, distance_factor, roll_off_scale)),
            e => Err(e)
        }
    }

    pub fn set_stream_buffer_size(&self, file_buffer_size: u32, FmodTimeUnit(file_buffer_size_type): FmodTimeUnit) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetStreamBufferSize(self.system, file_buffer_size, file_buffer_size_type) }
    }

    pub fn get_stream_buffer_size(&self) -> Result<(u32, FmodTimeUnit), FMOD_RESULT> {
        let file_buffer_size = 0u32;
        let file_buffer_size_type = 0u32;

        match unsafe { ffi::FMOD_System_GetStreamBufferSize(self.system, &file_buffer_size, &file_buffer_size_type) } {
            FMOD_OK => Ok((file_buffer_size, FmodTimeUnit(file_buffer_size_type))),
            e => Err(e)
        }
    }

    pub fn get_version(&self) -> Result<u32, FMOD_RESULT> {
        let version = 0u32;

        match unsafe { ffi::FMOD_System_GetVersion(self.system, &version) } {
            FMOD_OK => Ok(version),
            e => Err(e)
        }
    }

    pub fn get_output_handle(&self) -> Result<FmodOutputHandle, FMOD_RESULT> {
        let output_h = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_GetOutputHandle(self.system, &output_h) } {
            FMOD_OK => Ok(FmodOutputHandle{handle: output_h}),
            e => Err(e)
        }
    }

    pub fn get_channels_playing(&self) -> Result<i32, FMOD_RESULT> {
        let playing_chans = 0i32;

        match unsafe { ffi::FMOD_System_GetChannelsPlaying(self.system, &playing_chans) } {
            FMOD_OK => Ok(playing_chans),
            e => Err(e)
        }
    }

    pub fn get_CPU_usage(&self) -> Result<(f32, f32, f32, f32, f32), FMOD_RESULT> {
        let dsp = 0f32;
        let stream = 0f32;
        let geometry = 0f32;
        let update = 0f32;
        let total = 0f32;

        match unsafe { ffi::FMOD_System_GetCPUUsage(self.system, &dsp, &stream, &geometry, &update, &total) } {
            FMOD_OK => Ok((dsp, stream, geometry, update, total)),
            e => Err(e)
        }
    }

    pub fn get_sound_RAM(&self) -> Result<(i32, i32, i32), FMOD_RESULT> {
        let current_alloced = 0i32;
        let max_alloced = 0i32;
        let total = 0i32;

        match unsafe { ffi::FMOD_System_GetSoundRAM(self.system, &current_alloced, &max_alloced, &total) } {
            FMOD_OK => Ok((current_alloced, max_alloced, total)),
            e => Err(e)
        }
    }

    pub fn get_num_CDROM_drives(&self) -> Result<i32, FMOD_RESULT> {
        let num_drives = 0i32;

        match unsafe { ffi::FMOD_System_GetNumCDROMDrives(self.system, &num_drives) } {
            FMOD_OK => Ok(num_drives),
            e => Err(e)
        }
    }

    pub fn get_CDROM_drive_name(&self, drive: i32, drive_name_len: u32, scsi_name_len: u32, device_name_len: u32) -> Result<(StrBuf, StrBuf, StrBuf), FMOD_RESULT> {
        let drive_name = StrBuf::with_capacity(drive_name_len as uint).into_owned();
        let scsi_name = StrBuf::with_capacity(scsi_name_len as uint).into_owned();
        let device_name = StrBuf::with_capacity(device_name_len as uint).into_owned();

        drive_name.with_c_str(|c_drive_name|{
            scsi_name.with_c_str(|c_scsi_name|{
                device_name.with_c_str(|c_device_name|{
                    match unsafe { ffi::FMOD_System_GetCDROMDriveName(self.system, drive, c_drive_name, drive_name_len as i32, c_scsi_name, scsi_name_len as i32,
                        c_device_name, device_name_len as i32) } {
                        FMOD_OK => Ok((StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_drive_name) }).clone(),
                                        StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_scsi_name) }).clone(),
                                        StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_device_name) }).clone())),
                        e => Err(e)
                    }
                })
            })
        })
    }

    pub fn get_spectrum(&self, spectrum_size : uint, options : Option<channel::FmodSpectrumOptions>) -> Result<Vec<f32>, FMOD_RESULT> {
        let ptr = Vec::from_elem(spectrum_size, 0f32);
        let mut window_type = FMOD_DSP_FFT_WINDOW_RECT;
        let mut channel_offset = 0;

        match options {
            Some(v) => {
                window_type = v.window_type;
                channel_offset = v.channel_offset;
            }
            None => {}
        };
        match unsafe { ffi::FMOD_System_GetSpectrum(self.system, ptr.as_ptr(), spectrum_size as c_int, channel_offset, window_type) } {
            FMOD_OK => Ok(ptr),
            e => Err(e),
        }
    }

    pub fn get_wave_data(&self, wave_size : uint, channel_offset : i32) -> Result<Vec<f32>, FMOD_RESULT> {
        let ptr = Vec::from_elem(wave_size, 0f32);

        match unsafe { ffi::FMOD_System_GetWaveData(self.system, ptr.as_ptr(), wave_size as c_int, channel_offset) } {
            FMOD_OK => Ok(ptr),
            e => Err(e)
        }
    }
    
    pub fn get_channel(&self, channel_id: i32) -> Result<channel::Channel, FMOD_RESULT> {
        let channel = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_GetChannel(self.system, channel_id, &channel) } {
            FMOD_OK => Ok(channel::from_ptr(channel)),
            e => Err(e)
        }
    }

    pub fn get_master_channel_group(&self) -> Result<channel_group::ChannelGroup, FMOD_RESULT> {
        let channel_group = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_GetMasterChannelGroup(self.system, &channel_group) } {
            FMOD_OK => Ok(channel_group::from_ptr(channel_group)),
            e => Err(e)
        }
    }

    pub fn get_master_sound_group(&self) -> Result<sound_group::SoundGroup, FMOD_RESULT> {
        let sound_group = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_GetMasterSoundGroup(self.system, &sound_group) } {
            FMOD_OK => Ok(sound_group::from_ptr(sound_group)),
            e => Err(e)
        }
    }

    pub fn set_reverb_properties(&self, properties: FmodReverbProperties) -> FMOD_RESULT {
        let t_properties = properties.convert_to_c();

        unsafe { ffi::FMOD_System_SetReverbProperties(self.system, &t_properties) }
    }

    pub fn get_reverb_properties(&self) -> Result<FmodReverbProperties, FMOD_RESULT> {
        let properties = FmodReverbProperties::new().convert_to_c();

        match unsafe { ffi::FMOD_System_GetReverbProperties(self.system, &properties) } {
            FMOD_OK => Ok(FmodReverbProperties::from_ptr(properties)),
            e => Err(e)
        }
    }

    pub fn set_reverb_ambient_properties(&self, properties: FmodReverbProperties) -> FMOD_RESULT {
        let t_properties = properties.convert_to_c();

        unsafe { ffi::FMOD_System_SetReverbAmbientProperties(self.system, &t_properties) }
    }

    pub fn get_reverb_ambient_properties(&self) -> Result<FmodReverbProperties, FMOD_RESULT> {
        let properties = FmodReverbProperties::new().convert_to_c();

        match unsafe { ffi::FMOD_System_GetReverbAmbientProperties(self.system, &properties) } {
            FMOD_OK => Ok(FmodReverbProperties::from_ptr(properties)),
            e => Err(e)
        }
    }

    pub fn get_DSP_head(&self) -> Result<ffi::FmodDSP, FMOD_RESULT> {
        let head = ::std::ptr::null();

        match unsafe { ffi::FMOD_System_GetDSPHead(self.system, &head) } {
            FMOD_OK => Ok(ffi::FmodDSP::from_ptr(head)),
            e => Err(e)
        }
    }

    pub fn lock_DSP(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_LockDSP(self.system) }
    }

    pub fn unlock_DSP(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_UnlockDSP(self.system) }
    }

    pub fn get_DSP_clock(&self) -> Result<(u32, u32), FMOD_RESULT> {
        let hi = 0u32;
        let lo = 0u32;

        match unsafe { ffi::FMOD_System_GetDSPClock(self.system, &hi, &lo) } {
            FMOD_OK => Ok((hi, lo)),
            e => Err(e)
        }
    }

    pub fn get_record_num_drivers(&self) -> Result<i32, FMOD_RESULT> {
        let num_drivers = 0i32;

        match unsafe { ffi::FMOD_System_GetRecordNumDrivers(self.system, &num_drivers) } {
            FMOD_OK => Ok(num_drivers),
            e => Err(e)
        }
    }

    pub fn get_record_driver_info(&self, id: i32, name_len: uint) -> Result<(FmodGuid, StrBuf), FMOD_RESULT> {
        let tmp_v = StrBuf::with_capacity(name_len).into_owned();
        let guid = ffi::FMOD_GUID{Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0]};

        tmp_v.with_c_str(|c_str|{
            match unsafe { ffi::FMOD_System_GetRecordDriverInfo(self.system, id, c_str, name_len as i32, &guid) } {
                FMOD_OK => Ok((FmodGuid{data1: guid.Data1, data2: guid.Data2, data3: guid.Data3, data4: guid.Data4},
                    StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_str) }).clone())),
                e => Err(e)
            }
        })
    }

    pub fn get_record_driver_caps(&self, id: i32) -> Result<(FmodCaps, i32, i32), FMOD_RESULT> {
        let fmod_caps = 0u32;
        let min_frequency = 0i32;
        let max_frequency = 0i32;

        match unsafe { ffi::FMOD_System_GetRecordDriverCaps(self.system, id, &fmod_caps, &min_frequency, &max_frequency) } {
            FMOD_OK => Ok((FmodCaps(fmod_caps), min_frequency, max_frequency)),
            e => Err(e)
        }
    }

    pub fn get_record_position(&self, id: i32) -> Result<u32, FMOD_RESULT> {
        let position = 0u32;

        match unsafe { ffi::FMOD_System_GetRecordPosition(self.system, id, &position) } {
            FMOD_OK => Ok(position),
            e => Err(e)
        }
    }

    pub fn start_record(&self, id: i32, sound: sound::Sound, _loop: bool) -> FMOD_RESULT {
        let t_loop = match _loop {
                true => 1,
                _ => 0
            };

        unsafe { ffi::FMOD_System_RecordStart(self.system, id, sound::get_ffi(sound), t_loop) }
    }

    pub fn stop_record(&self, id: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_RecordStop(self.system, id) }
    }

    pub fn is_recording(&self, id: i32) -> Result<bool, FMOD_RESULT> {
        let is_recording = 0i32;
        
        match unsafe { ffi::FMOD_System_IsRecording(self.system, id, &is_recording) } {
            FMOD_OK => Ok(is_recording == 1),
            e => Err(e)
        }
    }

    pub fn create_geometry(&self, max_polygons: i32, max_vertices: i32) -> Result<FmodGeometry, FMOD_RESULT> {
        let geometry = ::std::ptr::null();
        match unsafe { ffi::FMOD_System_CreateGeometry(self.system, max_polygons, max_vertices, &geometry) } {
            FMOD_OK => Ok(FmodGeometry{geometry: geometry}),
            e => Err(e)
        }
    }

    pub fn set_geometry_settings(&self, max_world_size: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_System_SetGeometrySettings(self.system, max_world_size) }
    }

    pub fn get_geometry_settings(&self) -> Result<f32, FMOD_RESULT> {
        let max_world_size = 0f32;

        match unsafe { ffi::FMOD_System_GetGeometrySettings(self.system, &max_world_size) } {
            FMOD_OK => Ok(max_world_size),
            e => Err(e)
        }
    }

    pub fn get_geometry_occlusion(&self) -> Result<(FmodVector, FmodVector, f32, f32), FMOD_RESULT> {
        let listener = FmodVector::new().convert_to_c();
        let source = FmodVector::new().convert_to_c();
        let direct = 0f32;
        let reverb = 0f32;

        match unsafe { ffi::FMOD_System_GetGeometryOcclusion(self.system, &listener, &source, &direct, &reverb) } {
            FMOD_OK => Ok((from_c(listener), from_c(source), direct, reverb)),
            e => Err(e)
        }
    }
}