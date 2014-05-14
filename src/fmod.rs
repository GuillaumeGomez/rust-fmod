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

#![crate_id = "github.com/GuillaumeGomez/rust-fmod#rfmod:0.1"]
#![desc = "Rust binding for FMOD"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(uppercase_variables)]

extern crate libc;

use libc::{c_void, c_uint, c_int, c_char, c_float};
use std::io::timer::sleep;

#[link(name = "fmodex64")] extern{}

pub type FMOD_SYSTEM = *c_void;
pub type FMOD_SOUND = *c_void;
pub type FMOD_MODE = c_uint;
pub type FMOD_CHANNEL = *c_void;
pub type FMOD_CHANNEL_GROUP = *c_void;
pub type FMOD_BOOL = c_int;
pub type FMOD_TIMEUNIT = c_uint;
pub type FMOD_SOUNDGROUP = c_void;
pub type FMOD_INITFLAGS = c_uint;
pub type FMOD_DSP = c_void;

pub type FMOD_FILE_OPENCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_char, arg2: int, arg3: *c_uint, arg4: **c_void, arg5: **c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_CLOSECALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_READCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void, arg3: c_uint, arg4: *c_uint, arg5: *c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_SEEKCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: c_uint, arg3: *c_void) -> FMOD_RESULT>;
pub type FMOD_SOUND_NONBLOCKCALLBACK = ::std::option::Option<extern "C" fn(arg1: FMOD_SOUND, arg2: FMOD_RESULT) -> FMOD_RESULT>;
pub type FMOD_SOUND_PCMREADCALLBACK = ::std::option::Option<extern "C" fn(arg1: FMOD_SOUND, arg2: *c_void, arg3: c_uint) -> FMOD_RESULT>;
pub type FMOD_FILE_ASYNCREADCALLBACK = ::std::option::Option<extern "C" fn(arg1: *FMOD_ASYNCREADINFO, arg2: *c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_ASYNCCANCELCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void, arg3: c_uint) -> FMOD_RESULT>;
pub type FMOD_SOUND_PCMSETPOSCALLBACK = ::std::option::Option<extern "C" fn(arg1: FMOD_SOUND, arg2: c_int, arg3: c_uint, arg4: FMOD_TIMEUNIT) -> FMOD_RESULT>;

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_SPEAKERMAPTYPE
{
    FMOD_SPEAKERMAPTYPE_DEFAULT,     /* This is the default, and just means FMOD decides which speakers it puts the source channels. */
    FMOD_SPEAKERMAPTYPE_ALLMONO,     /* This means the sound is made up of all mono sounds.  All voices will be panned to the front center by default in this case.  */
    FMOD_SPEAKERMAPTYPE_ALLSTEREO,   /* This means the sound is made up of all stereo sounds.  All voices will be panned to front left and front right alternating every second channel.  */
    FMOD_SPEAKERMAPTYPE_51_PROTOOLS, /* Map a 5.1 sound to use protools L C R Ls Rs LFE mapping.  Will return an error if not a 6 channel sound. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_SOUND_FORMAT
{
    FMOD_SOUND_FORMAT_NONE,             /* Unitialized / unknown. */
    FMOD_SOUND_FORMAT_PCM8,             /* 8bit integer PCM data. */
    FMOD_SOUND_FORMAT_PCM16,            /* 16bit integer PCM data. */
    FMOD_SOUND_FORMAT_PCM24,            /* 24bit integer PCM data. */
    FMOD_SOUND_FORMAT_PCM32,            /* 32bit integer PCM data. */
    FMOD_SOUND_FORMAT_PCMFLOAT,         /* 32bit floating point PCM data. */
    FMOD_SOUND_FORMAT_GCADPCM,          /* Compressed Nintendo 3DS/Wii DSP data. */
    FMOD_SOUND_FORMAT_IMAADPCM,         /* Compressed IMA ADPCM data. */
    FMOD_SOUND_FORMAT_VAG,              /* Compressed PlayStation Portable ADPCM data. */
    FMOD_SOUND_FORMAT_HEVAG,            /* Compressed PSVita ADPCM data. */
    FMOD_SOUND_FORMAT_XMA,              /* Compressed Xbox360 XMA data. */
    FMOD_SOUND_FORMAT_MPEG,             /* Compressed MPEG layer 2 or 3 data. */
    FMOD_SOUND_FORMAT_CELT,             /* Compressed CELT data. */
    FMOD_SOUND_FORMAT_AT9,              /* Compressed PSVita ATRAC9 data. */
    FMOD_SOUND_FORMAT_XWMA,             /* Compressed Xbox360 xWMA data. */
    FMOD_SOUND_FORMAT_VORBIS,           /* Compressed Vorbis data. */

    FMOD_SOUND_FORMAT_MAX,              /* Maximum number of sound formats supported. */   
    FMOD_SOUND_FORMAT_FORCEINT = 65536, /* Makes sure this enum is signed 32bit. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_SOUND_TYPE
{
    FMOD_SOUND_TYPE_UNKNOWN,         /* 3rd party / unknown plugin format. */
    FMOD_SOUND_TYPE_AIFF,            /* AIFF. */
    FMOD_SOUND_TYPE_ASF,             /* Microsoft Advanced Systems Format (ie WMA/ASF/WMV). */
    FMOD_SOUND_TYPE_AT3,             /* Sony ATRAC 3 format */
    FMOD_SOUND_TYPE_CDDA,            /* Digital CD audio. */
    FMOD_SOUND_TYPE_DLS,             /* Sound font / downloadable sound bank. */
    FMOD_SOUND_TYPE_FLAC,            /* FLAC lossless codec. */
    FMOD_SOUND_TYPE_FSB,             /* FMOD Sample Bank. */
    FMOD_SOUND_TYPE_GCADPCM,         /* Nintendo GameCube/Wii ADPCM */
    FMOD_SOUND_TYPE_IT,              /* Impulse Tracker. */
    FMOD_SOUND_TYPE_MIDI,            /* MIDI. extracodecdata is a pointer to an FMOD_MIDI_EXTRACODECDATA structure. */
    FMOD_SOUND_TYPE_MOD,             /* Protracker / Fasttracker MOD. */
    FMOD_SOUND_TYPE_MPEG,            /* MP2/MP3 MPEG. */
    FMOD_SOUND_TYPE_OGGVORBIS,       /* Ogg vorbis. */
    FMOD_SOUND_TYPE_PLAYLIST,        /* Information only from ASX/PLS/M3U/WAX playlists */
    FMOD_SOUND_TYPE_RAW,             /* Raw PCM data. */
    FMOD_SOUND_TYPE_S3M,             /* ScreamTracker 3. */
    FMOD_SOUND_TYPE_SF2,             /* Sound font 2 format. */
    FMOD_SOUND_TYPE_USER,            /* User created sound. */
    FMOD_SOUND_TYPE_WAV,             /* Microsoft WAV. */
    FMOD_SOUND_TYPE_XM,              /* FastTracker 2 XM. */
    FMOD_SOUND_TYPE_XMA,             /* Xbox360 XMA */
    FMOD_SOUND_TYPE_VAG,             /* PlayStation Portable ADPCM VAG format. */
    FMOD_SOUND_TYPE_AUDIOQUEUE,      /* iPhone hardware decoder, supports AAC, ALAC and MP3. extracodecdata is a pointer to an FMOD_AUDIOQUEUE_EXTRACODECDATA structure. */
    FMOD_SOUND_TYPE_XWMA,            /* Xbox360 XWMA */
    FMOD_SOUND_TYPE_BCWAV,           /* 3DS BCWAV container format for DSP ADPCM and PCM */
    FMOD_SOUND_TYPE_AT9,             /* NGP ATRAC 9 format */
    FMOD_SOUND_TYPE_VORBIS,          /* Raw vorbis */
    FMOD_SOUND_TYPE_MEDIA_FOUNDATION,/* Microsoft Media Foundation wrappers, supports ASF/WMA */

    FMOD_SOUND_TYPE_MAX,             /* Maximum number of sound types supported. */
    FMOD_SOUND_TYPE_FORCEINT = 65536,/* Makes sure this enum is signed 32bit. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_RESULT
{
    FMOD_OK,                        /* No errors. */
    FMOD_ERR_ALREADYLOCKED,         /* Tried to call lock a second time before unlock was called. */
    FMOD_ERR_BADCOMMAND,            /* Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound::lock on a streaming sound). */
    FMOD_ERR_CDDA_DRIVERS,          /* Neither NTSCSI nor ASPI could be initialised. */
    FMOD_ERR_CDDA_INIT,             /* An error occurred while initialising the CDDA subsystem. */
    FMOD_ERR_CDDA_INVALID_DEVICE,   /* Couldn't find the specified device. */
    FMOD_ERR_CDDA_NOAUDIO,          /* No audio tracks on the specified disc. */
    FMOD_ERR_CDDA_NODEVICES,        /* No CD/DVD devices were found. */ 
    FMOD_ERR_CDDA_NODISC,           /* No disc present in the specified drive. */
    FMOD_ERR_CDDA_READ,             /* A CDDA read error occurred. */
    FMOD_ERR_CHANNEL_ALLOC,         /* Error trying to allocate a channel. */
    FMOD_ERR_CHANNEL_STOLEN,        /* The specified channel has been reused to play another sound. */
    FMOD_ERR_COM,                   /* A Win32 COM related error occured. COM failed to initialize or a QueryInterface failed meaning a Windows codec or driver was not installed properly. */
    FMOD_ERR_DMA,                   /* DMA Failure.  See debug output for more information. */
    FMOD_ERR_DSP_CONNECTION,        /* DSP connection error.  Connection possibly caused a cyclic dependancy.  Or tried to connect a tree too many units deep (more than 128). */
    FMOD_ERR_DSP_FORMAT,            /* DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format. */
    FMOD_ERR_DSP_NOTFOUND,          /* DSP connection error.  Couldn't find the DSP unit specified. */
    FMOD_ERR_DSP_RUNNING,           /* DSP error.  Cannot perform this operation while the network is in the middle of running.  This will most likely happen if a connection or disconnection is attempted in a DSP callback. */
    FMOD_ERR_DSP_TOOMANYCONNECTIONS,/* DSP connection error.  The unit being connected to or disconnected should only have 1 input or output. */
    FMOD_ERR_FILE_BAD,              /* Error loading file. */
    FMOD_ERR_FILE_COULDNOTSEEK,     /* Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format. */
    FMOD_ERR_FILE_DISKEJECTED,      /* Media was ejected while reading. */
    FMOD_ERR_FILE_EOF,              /* End of file unexpectedly reached while trying to read essential data (truncated data?). */
    FMOD_ERR_FILE_NOTFOUND,         /* File not found. */
    FMOD_ERR_FILE_UNWANTED,         /* Unwanted file access occured. */
    FMOD_ERR_FORMAT,                /* Unsupported file or audio format. */
    FMOD_ERR_HTTP,                  /* A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere. */
    FMOD_ERR_HTTP_ACCESS,           /* The specified resource requires authentication or is forbidden. */
    FMOD_ERR_HTTP_PROXY_AUTH,       /* Proxy authentication is required to access the specified resource. */
    FMOD_ERR_HTTP_SERVER_ERROR,     /* A HTTP server error occurred. */
    FMOD_ERR_HTTP_TIMEOUT,          /* The HTTP request timed out. */
    FMOD_ERR_INITIALIZATION,        /* FMOD was not initialized correctly to support this function. */
    FMOD_ERR_INITIALIZED,           /* Cannot call this command after System::init. */
    FMOD_ERR_INTERNAL,              /* An error occured that wasn't supposed to.  Contact support. */
    FMOD_ERR_INVALID_ADDRESS,       /* On Xbox 360, this memory address passed to FMOD must be physical, (ie allocated with XPhysicalAlloc.) */
    FMOD_ERR_INVALID_FLOAT,         /* Value passed in was a NaN, Inf or denormalized float. */
    FMOD_ERR_INVALID_HANDLE,        /* An invalid object handle was used. */
    FMOD_ERR_INVALID_PARAM,         /* An invalid parameter was passed to this function. */
    FMOD_ERR_INVALID_POSITION,      /* An invalid seek position was passed to this function. */
    FMOD_ERR_INVALID_SPEAKER,       /* An invalid speaker was passed to this function based on the current speaker mode. */
    FMOD_ERR_INVALID_SYNCPOINT,     /* The syncpoint did not come from this sound handle. */
    FMOD_ERR_INVALID_VECTOR,        /* The vectors passed in are not unit length, or perpendicular. */
    FMOD_ERR_MAXAUDIBLE,            /* Reached maximum audible playback count for this sound's soundgroup. */
    FMOD_ERR_MEMORY,                /* Not enough memory or resources. */
    FMOD_ERR_MEMORY_CANTPOINT,      /* Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used. */
    FMOD_ERR_MEMORY_SRAM,           /* Not enough memory or resources on console sound ram. */
    FMOD_ERR_NEEDS2D,               /* Tried to call a command on a 3d sound when the command was meant for 2d sound. */
    FMOD_ERR_NEEDS3D,               /* Tried to call a command on a 2d sound when the command was meant for 3d sound. */
    FMOD_ERR_NEEDSHARDWARE,         /* Tried to use a feature that requires hardware support.  (ie trying to play a GCADPCM compressed sound in software on Wii). */
    FMOD_ERR_NEEDSSOFTWARE,         /* Tried to use a feature that requires the software engine.  Software engine has either been turned off, or command was executed on a hardware channel which does not support this feature. */
    FMOD_ERR_NET_CONNECT,           /* Couldn't connect to the specified host. */
    FMOD_ERR_NET_SOCKET_ERROR,      /* A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere. */
    FMOD_ERR_NET_URL,               /* The specified URL couldn't be resolved. */
    FMOD_ERR_NET_WOULD_BLOCK,       /* Operation on a non-blocking socket could not complete immediately. */
    FMOD_ERR_NOTREADY,              /* Operation could not be performed because specified sound/DSP connection is not ready. */
    FMOD_ERR_OUTPUT_ALLOCATED,      /* Error initializing output device, but more specifically, the output device is already in use and cannot be reused. */
    FMOD_ERR_OUTPUT_CREATEBUFFER,   /* Error creating hardware sound buffer. */
    FMOD_ERR_OUTPUT_DRIVERCALL,     /* A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted. */
    FMOD_ERR_OUTPUT_ENUMERATION,    /* Error enumerating the available driver list. List may be inconsistent due to a recent device addition or removal. */
    FMOD_ERR_OUTPUT_FORMAT,         /* Soundcard does not support the minimum features needed for this soundsystem (16bit stereo output). */
    FMOD_ERR_OUTPUT_INIT,           /* Error initializing output device. */
    FMOD_ERR_OUTPUT_NOHARDWARE,     /* FMOD_HARDWARE was specified but the sound card does not have the resources necessary to play it. */
    FMOD_ERR_OUTPUT_NOSOFTWARE,     /* Attempted to create a software sound but no software channels were specified in System::init. */
    FMOD_ERR_PAN,                   /* Panning only works with mono or stereo sound sources. */
    FMOD_ERR_PLUGIN,                /* An unspecified error has been returned from a 3rd party plugin. */
    FMOD_ERR_PLUGIN_INSTANCES,      /* The number of allowed instances of a plugin has been exceeded. */
    FMOD_ERR_PLUGIN_MISSING,        /* A requested output, dsp unit type or codec was not available. */
    FMOD_ERR_PLUGIN_RESOURCE,       /* A resource that the plugin requires cannot be found. (ie the DLS file for MIDI playback or other DLLs that it needs to load) */
    FMOD_ERR_PRELOADED,             /* The specified sound is still in use by the event system, call EventSystem::unloadFSB before trying to release it. */
    FMOD_ERR_PROGRAMMERSOUND,       /* The specified sound is still in use by the event system, wait for the event which is using it finish with it. */
    FMOD_ERR_RECORD,                /* An error occured trying to initialize the recording device. */
    FMOD_ERR_REVERB_INSTANCE,       /* Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesnt exist. */
    FMOD_ERR_SUBSOUND_ALLOCATED,    /* This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first. */
    FMOD_ERR_SUBSOUND_CANTMOVE,     /* Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file. */
    FMOD_ERR_SUBSOUND_MODE,         /* The subsound's mode bits do not match with the parent sound's mode bits.  See documentation for function that it was called with. */
    FMOD_ERR_SUBSOUNDS,             /* The error occured because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound, or a parent sound was played without setting up a sentence first. */
    FMOD_ERR_TAGNOTFOUND,           /* The specified tag could not be found or there are no tags. */
    FMOD_ERR_TOOMANYCHANNELS,       /* The sound created exceeds the allowable input channel count.  This can be increased using the maxinputchannels parameter in System::setSoftwareFormat. */
    FMOD_ERR_UNIMPLEMENTED,         /* Something in FMOD hasn't been implemented when it should be! contact support! */
    FMOD_ERR_UNINITIALIZED,         /* This command failed because System::init or System::setDriver was not called. */
    FMOD_ERR_UNSUPPORTED,           /* A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified. */
    FMOD_ERR_UPDATE,                /* An error caused by System::update occured. */
    FMOD_ERR_VERSION,               /* The version number of this file format is not supported. */

    FMOD_ERR_EVENT_FAILED,          /* An Event failed to be retrieved, most likely due to 'just fail' being specified as the max playbacks behavior. */
    FMOD_ERR_EVENT_INFOONLY,        /* Can't execute this command on an EVENT_INFOONLY event. */
    FMOD_ERR_EVENT_INTERNAL,        /* An error occured that wasn't supposed to.  See debug log for reason. */
    FMOD_ERR_EVENT_MAXSTREAMS,      /* Event failed because 'Max streams' was hit when FMOD_EVENT_INIT_FAIL_ON_MAXSTREAMS was specified. */
    FMOD_ERR_EVENT_MISMATCH,        /* FSB mismatches the FEV it was compiled with, the stream/sample mode it was meant to be created with was different, or the FEV was built for a different platform. */
    FMOD_ERR_EVENT_NAMECONFLICT,    /* A category with the same name already exists. */
    FMOD_ERR_EVENT_NOTFOUND,        /* The requested event, event group, event category or event property could not be found. */
    FMOD_ERR_EVENT_NEEDSSIMPLE,     /* Tried to call a function on a complex event that's only supported by simple events. */
    FMOD_ERR_EVENT_GUIDCONFLICT,    /* An event with the same GUID already exists. */
    FMOD_ERR_EVENT_ALREADY_LOADED,  /* The specified project or bank has already been loaded. Having multiple copies of the same project loaded simultaneously is forbidden. */

    FMOD_ERR_MUSIC_UNINITIALIZED,   /* Music system is not initialized probably because no music data is loaded. */
    FMOD_ERR_MUSIC_NOTFOUND,        /* The requested music entity could not be found. */
    FMOD_ERR_MUSIC_NOCALLBACK,      /* The music callback is required, but it has not been set. */

    FMOD_RESULT_FORCEINT = 65536,   /* Makes sure this enum is signed 32bit. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_CHANNELINDEX {
    FMOD_CHANNEL_FREE  = -1,      /* For a channel index, FMOD chooses a free voice using the priority system. */
    FMOD_CHANNEL_REUSE = -2,      /* For a channel index, re-use the channel handle that was passed in. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_DSP_FFT_WINDOW
{
    FMOD_DSP_FFT_WINDOW_RECT,            /* w[n] = 1.0                                                                                            */
    FMOD_DSP_FFT_WINDOW_TRIANGLE,        /* w[n] = TRI(2n/N)                                                                                      */
    FMOD_DSP_FFT_WINDOW_HAMMING,         /* w[n] = 0.54 - (0.46 * COS(n/N) )                                                                      */
    FMOD_DSP_FFT_WINDOW_HANNING,         /* w[n] = 0.5 *  (1.0  - COS(n/N) )                                                                      */
    FMOD_DSP_FFT_WINDOW_BLACKMAN,        /* w[n] = 0.42 - (0.5  * COS(n/N) ) + (0.08 * COS(2.0 * n/N) )                                           */
    FMOD_DSP_FFT_WINDOW_BLACKMANHARRIS,  /* w[n] = 0.35875 - (0.48829 * COS(1.0 * n/N)) + (0.14128 * COS(2.0 * n/N)) - (0.01168 * COS(3.0 * n/N)) */
    
    FMOD_DSP_FFT_WINDOW_MAX,             /* Maximum number of FFT window types supported. */
    FMOD_DSP_FFT_WINDOW_FORCEINT = 65536 /* Makes sure this enum is signed 32bit. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_DELAYTYPE
{
    FMOD_DELAYTYPE_END_MS,              /* Delay at the end of the sound in milliseconds.  Use delayhi only.   Channel::isPlaying will remain true until this delay has passed even though the sound itself has stopped playing.*/
    FMOD_DELAYTYPE_DSPCLOCK_START,      /* Time the sound started if Channel::getDelay is used, or if Channel::setDelay is used, the sound will delay playing until this exact tick. */
    FMOD_DELAYTYPE_DSPCLOCK_END,        /* Time the sound should end. If this is non-zero, the channel will go silent at this exact tick. */
    FMOD_DELAYTYPE_DSPCLOCK_PAUSE,      /* Time the sound should pause. If this is non-zero, the channel will pause at this exact tick. */

    FMOD_DELAYTYPE_MAX,                 /* Maximum number of tag datatypes supported. */
    FMOD_DELAYTYPE_FORCEINT = 65536     /* Makes sure this enum is signed 32bit. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
// I need to find a solution for this enum...
pub enum FMOD_SPEAKER
{
    FMOD_SPEAKER_FRONT_LEFT,
    FMOD_SPEAKER_FRONT_RIGHT,
    FMOD_SPEAKER_FRONT_CENTER,
    FMOD_SPEAKER_LOW_FREQUENCY,
    FMOD_SPEAKER_BACK_LEFT,
    FMOD_SPEAKER_BACK_RIGHT,
    FMOD_SPEAKER_SIDE_LEFT,
    FMOD_SPEAKER_SIDE_RIGHT,
    
    FMOD_SPEAKER_MAX,                                       /* Maximum number of speaker types supported. */
    //FMOD_SPEAKER_MONO        = FMOD_SPEAKER_FRONT_LEFT,     /* For use with FMOD_SPEAKERMODE_MONO and Channel::SetSpeakerLevels.  Mapped to same value as FMOD_SPEAKER_FRONT_LEFT. */
    FMOD_SPEAKER_NULL        = 65535,                       /* A non speaker.  Use this with ASIO mapping to ignore a speaker. */
    //FMOD_SPEAKER_SBL         = FMOD_SPEAKER_SIDE_LEFT,      /* For use with FMOD_SPEAKERMODE_7POINT1 on PS3 where the extra speakers are surround back inside of side speakers. */
    //FMOD_SPEAKER_SBR         = FMOD_SPEAKER_SIDE_RIGHT,     /* For use with FMOD_SPEAKERMODE_7POINT1 on PS3 where the extra speakers are surround back inside of side speakers. */
    FMOD_SPEAKER_FORCEINT    = 65536                        /* Makes sure this enum is signed 32bit. */
}

pub struct FMOD_ASYNCREADINFO
{
	handle 		: *c_void, 		/* [r] The file handle that was filled out in the open callback. */
	offset 		: c_uint, 		/* [r] Seek position, make sure you read from this file offset. */
    sizebytes 	: c_uint, 		/* [r] how many bytes requested for read. */
    priority 	: c_int,		/* [r] 0 = low importance.  100 = extremely important (ie 'must read now or stuttering may occur') */

    buffer 		: *c_void,		/* [w] Buffer to read file data into. */
    bytesread 	: c_uint,		/* [w] Fill this in before setting result code to tell FMOD how many bytes were read. */
    result 		: FMOD_RESULT,	/* [r/w] Result code, FMOD_OK tells the system it is ready to consume the data.  Set this last!  Default value = FMOD_ERR_NOTREADY. */
    userdata 	: *c_void,		/* [r] User data pointer. */
}

pub struct FMOD_REVERB_CHANNELPROPERTIES
{                                      	/*       MIN    MAX  DEFAULT  DESCRIPTION */
    Direct 			: c_int,           	/* [r/w] -10000 1000 0        Direct path level                                        (SUPPORTED:SFX) */
    Room 			: c_int,           	/* [r/w] -10000 1000 0        Room effect level                                        (SUPPORTED:SFX) */
    Flags 			: c_uint,          	/* [r/w] FMOD_REVERB_CHANNELFLAGS - modifies the behavior of properties                (SUPPORTED:SFX) */
    ConnectionPoint : *FMOD_DSP      	/* [r/w] See remarks.         DSP network location to connect reverb for this channel. (SUPPORTED:SFX).*/
}

pub struct FMOD_CREATESOUNDEXINFO
{
	cbsize 				: c_int,             			/* [w] Size of this structure.  This is used so the structure can be expanded in the future and still work on older versions of FMOD Ex. */
    length 				: c_uint,            			/* [w] Optional. Specify 0 to ignore. Size in bytes of file to load, or sound to create (in this case only if FMOD_OPENUSER is used).  Required if loading from memory.  If 0 is specified, then it will use the size of the file (unless loading from memory then an error will be returned). */
    fileoffset 			: c_uint,        				/* [w] Optional. Specify 0 to ignore. Offset from start of the file to start loading from.  This is useful for loading files from inside big data files. */
    numchannels 		: c_int,						/* [w] Optional. Specify 0 to ignore. Number of channels in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. */
    defaultfrequency 	: c_int,						/* [w] Optional. Specify 0 to ignore. Default frequency of sound in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the frequency determined by the file format. */
    format 				: FMOD_SOUND_FORMAT,			/* [w] Optional. Specify 0 or FMOD_SOUND_FORMAT_NONE to ignore. Format of the sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the format determined by the file format.   */
    decodebuffersize 	: c_uint,						/* [w] Optional. Specify 0 to ignore. For streams.  This determines the size of the double buffer (in PCM samples) that a stream uses.  Use this for user created streams if you want to determine the size of the callback buffer passed to you.  Specify 0 to use FMOD's default size which is currently equivalent to 400ms of the sound format created/loaded. */
   	initialsubsound 	: c_int, 						/* [w] Optional. Specify 0 to ignore. In a multi-sample file format such as .FSB/.DLS/.SF2, specify the initial subsound to seek to, only if FMOD_CREATESTREAM is used. */
    numsubsounds 		: c_int,       					/* [w] Optional. Specify 0 to ignore or have no subsounds.  In a sound created with FMOD_OPENUSER, specify the number of subsounds that are accessable with Sound::getSubSound.  If not created with FMOD_OPENUSER, this will limit the number of subsounds loaded within a multi-subsound file.  If using FSB, then if FMOD_CREATESOUNDEXINFO::inclusionlist is used, this will shuffle subsounds down so that there are not any gaps.  It will mean that the indices of the sounds will be different. */
    inclusionlist 		: *c_int,     					/* [w] Optional. Specify 0 to ignore. In a multi-sample format such as .FSB/.DLS/.SF2 it may be desirable to specify only a subset of sounds to be loaded out of the whole file.  This is an array of subsound indices to load into memory when created. */
    inclusionlistnum 	: c_int,						/* [w] Optional. Specify 0 to ignore. This is the number of integers contained within the inclusionlist array. */
    pcmreadcallback 	: FMOD_SOUND_PCMREADCALLBACK,   /* [w] Optional. Specify 0 to ignore. Callback to 'piggyback' on FMOD's read functions and accept or even write PCM data while FMOD is opening the sound.  Used for user sounds created with FMOD_OPENUSER or for capturing decoded data as FMOD reads it. */
    pcmsetposcallback 	: FMOD_SOUND_PCMSETPOSCALLBACK,	/* [w] Optional. Specify 0 to ignore. Callback for when the user calls a seeking function such as Channel::setTime or Channel::setPosition within a multi-sample sound, and for when it is opened.*/
    nonblockcallback 	: FMOD_SOUND_NONBLOCKCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for successful completion, or error while loading a sound that used the FMOD_NONBLOCKING flag.  Also called duing seeking, when setPosition is called or a stream is restarted. */
    dlsname 			: *c_char,            			/* [w] Optional. Specify 0 to ignore. Filename for a DLS or SF2 sample set when loading a MIDI file. If not specified, on Windows it will attempt to open /windows/system32/drivers/gm.dls or /windows/system32/drivers/etc/gm.dls, on Mac it will attempt to load /System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls, otherwise the MIDI will fail to open. Current DLS support is for level 1 of the specification. */
    encryptionkey 		: *c_char,      				/* [w] Optional. Specify 0 to ignore. Key for encrypted FSB file.  Without this key an encrypted FSB file will not load. */
    maxpolyphony 		: c_int,       					/* [w] Optional. Specify 0 to ignore. For sequenced formats with dynamic channel allocation such as .MID and .IT, this specifies the maximum voice count allowed while playing.  .IT defaults to 64.  .MID defaults to 32. */
    userdata 			: *c_void,		           		/* [w] Optional. Specify 0 to ignore. This is user data to be attached to the sound during creation.  Access via Sound::getUserData.  Note: This is not passed to FMOD_FILE_OPENCALLBACK, that is a different userdata that is file specific. */
    suggestedsoundtype 	: FMOD_SOUND_TYPE, 				/* [w] Optional. Specify 0 or FMOD_SOUND_TYPE_UNKNOWN to ignore.  Instead of scanning all codec types, use this to speed up loading by making it jump straight to this codec. */
    useropen 			: FMOD_FILE_OPENCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for opening this file. */
    userclose 			: FMOD_FILE_CLOSECALLBACK,      /* [w] Optional. Specify 0 to ignore. Callback for closing this file. */
    userread 			: FMOD_FILE_READCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for reading from this file. */
    userseek 			: FMOD_FILE_SEEKCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    userasyncread 		: FMOD_FILE_ASYNCREADCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    userasynccancel 	: FMOD_FILE_ASYNCCANCELCALLBACK,/* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    speakermap 			: FMOD_SPEAKERMAPTYPE,			/* [w] Optional. Specify 0 to ignore. Use this to differ the way fmod maps multichannel sounds to speakers.  See FMOD_SPEAKERMAPTYPE for more. */
    initialsoundgroup 	: *FMOD_SOUNDGROUP,  			/* [w] Optional. Specify 0 to ignore. Specify a sound group if required, to put sound in as it is created. */
    initialseekposition : c_uint,						/* [w] Optional. Specify 0 to ignore. For streams. Specify an initial position to seek the stream to. */
    initialseekpostype 	: FMOD_TIMEUNIT, 				/* [w] Optional. Specify 0 to ignore. For streams. Specify the time unit for the position set in initialseekposition. */
    ignoresetfilesystem : c_int,						/* [w] Optional. Specify 0 to ignore. Set to 1 to use fmod's built in file system. Ignores setFileSystem callbacks and also FMOD_CREATESOUNEXINFO file callbacks.  Useful for specific cases where you don't want to use your own file system but want to use fmod's file system (ie net streaming). */
    cddaforceaspi 		: c_int,     					/* [w] Optional. Specify 0 to ignore. For CDDA sounds only - if non-zero use ASPI instead of NTSCSI to access the specified CD/DVD device. */
    audioqueuepolicy 	: c_uint,   					/* [w] Optional. Specify 0 or FMOD_AUDIOQUEUE_CODECPOLICY_DEFAULT to ignore. Policy used to determine whether hardware or software is used for decoding, see FMOD_AUDIOQUEUE_CODECPOLICY for options (iOS >= 3.0 required, otherwise only hardware is available) */ 
    minmidigranularity 	: c_uint, 						/* [w] Optional. Specify 0 to ignore. Allows you to set a minimum desired MIDI mixer granularity. Values smaller than 512 give greater than default accuracy at the cost of more CPU and vice versa. Specify 0 for default (512 samples). */
    nonblockthreadid 	: c_int,   						/* [w] Optional. Specify 0 to ignore. Specifies a thread index to execute non blocking load on.  Allows for up to 5 threads to be used for loading at once.  This is to avoid one load blocking another.  Maximum value = 4. */
}

pub static FMOD_DEFAULT                 : c_uint = 0x00000000;  /* Default for all modes listed below. FMOD_LOOP_OFF, FMOD_2D, FMOD_HARDWARE */
pub static FMOD_LOOP_OFF                : c_uint = 0x00000001;  /* For non looping sounds. (DEFAULT).  Overrides FMOD_LOOP_NORMAL / FMOD_LOOP_BIDI. */
pub static FMOD_LOOP_NORMAL             : c_uint = 0x00000002;  /* For forward looping sounds. */
pub static FMOD_LOOP_BIDI               : c_uint = 0x00000004;  /* For bidirectional looping sounds. (only works on software mixed static sounds). */
pub static FMOD_2D                      : c_uint = 0x00000008;  /* Ignores any 3d processing. (DEFAULT). */
pub static FMOD_3D                      : c_uint = 0x00000010;  /* Makes the sound positionable in 3D.  Overrides FMOD_2D. */
pub static FMOD_HARDWARE                : c_uint = 0x00000020;  /* Attempts to make sounds use hardware acceleration. (DEFAULT).  Note on platforms that don't support FMOD_HARDWARE (only 3DS, PS Vita, PSP, Wii and Wii U support FMOD_HARDWARE), this will be internally treated as FMOD_SOFTWARE. */
pub static FMOD_SOFTWARE                : c_uint = 0x00000040;  /* Makes the sound be mixed by the FMOD CPU based software mixer.  Overrides FMOD_HARDWARE.  Use this for FFT, DSP, compressed sample support, 2D multi-speaker support and other software related features. */
pub static FMOD_CREATESTREAM            : c_uint = 0x00000080;  /* Decompress at runtime, streaming from the source provided (ie from disk).  Overrides FMOD_CREATESAMPLE and FMOD_CREATECOMPRESSEDSAMPLE.  Note a stream can only be played once at a time due to a stream only having 1 stream buffer and file handle.  Open multiple streams to have them play concurrently. */
pub static FMOD_CREATESAMPLE            : c_uint = 0x00000100;  /* Decompress at loadtime, decompressing or decoding whole file into memory as the target sample format (ie PCM).  Fastest for FMOD_SOFTWARE based playback and most flexible.  */
pub static FMOD_CREATECOMPRESSEDSAMPLE  : c_uint = 0x00000200;  /* Load MP2/MP3/IMAADPCM/CELT/Vorbis/AT9 or XMA into memory and leave it compressed.  CELT/Vorbis/AT9 encoding only supported in the FSB file format.  During playback the FMOD software mixer will decode it in realtime as a 'compressed sample'.  Can only be used in combination with FMOD_SOFTWARE.  Overrides FMOD_CREATESAMPLE.  If the sound data is not one of the supported formats, it will behave as if it was created with FMOD_CREATESAMPLE and decode the sound into PCM. */
pub static FMOD_OPENUSER                : c_uint = 0x00000400;  /* Opens a user created static sample or stream. Use FMOD_CREATESOUNDEXINFO to specify format and/or read callbacks.  If a user created 'sample' is created with no read callback, the sample will be empty.  Use Sound::lock and Sound::unlock to place sound data into the sound if this is the case. */
pub static FMOD_OPENMEMORY              : c_uint = 0x00000800;  /* "name_or_data" will be interpreted as a pointer to memory instead of filename for creating sounds.  Use FMOD_CREATESOUNDEXINFO to specify length.  If used with FMOD_CREATESAMPLE or FMOD_CREATECOMPRESSEDSAMPLE, FMOD duplicates the memory into its own buffers.  Your own buffer can be freed after open.  If used with FMOD_CREATESTREAM, FMOD will stream out of the buffer whose pointer you passed in.  In this case, your own buffer should not be freed until you have finished with and released the stream.*/
pub static FMOD_OPENMEMORY_POINT        : c_uint = 0x10000000;  /* "name_or_data" will be interpreted as a pointer to memory instead of filename for creating sounds.  Use FMOD_CREATESOUNDEXINFO to specify length.  This differs to FMOD_OPENMEMORY in that it uses the memory as is, without duplicating the memory into its own buffers.  For Wii/PSP FMOD_HARDWARE supports this flag for the GCADPCM/VAG formats.  On other platforms FMOD_SOFTWARE must be used, as sound hardware on the other platforms (ie PC) cannot access main ram.  Cannot be freed after open, only after Sound::release.   Will not work if the data is compressed and FMOD_CREATECOMPRESSEDSAMPLE is not used. */
pub static FMOD_OPENRAW                 : c_uint = 0x00001000;  /* Will ignore file format and treat as raw pcm.  Use FMOD_CREATESOUNDEXINFO to specify format.  Requires at least defaultfrequency, numchannels and format to be specified before it will open.  Must be little endian data. */
pub static FMOD_OPENONLY                : c_uint = 0x00002000;  /* Just open the file, dont prebuffer or read.  Good for fast opens for info, or when sound::readData is to be used. */
pub static FMOD_ACCURATETIME            : c_uint = 0x00004000;  /* For System::createSound - for accurate Sound::getLength/Channel::setPosition on VBR MP3, and MOD/S3M/XM/IT/MIDI files.  Scans file first, so takes longer to open. FMOD_OPENONLY does not affect this. */
pub static FMOD_MPEGSEARCH              : c_uint = 0x00008000;  /* For corrupted / bad MP3 files.  This will search all the way through the file until it hits a valid MPEG header.  Normally only searches for 4k. */
pub static FMOD_NONBLOCKING             : c_uint = 0x00010000;  /* For opening sounds and getting streamed subsounds (seeking) asyncronously.  Use Sound::getOpenState to poll the state of the sound as it opens or retrieves the subsound in the background. */
pub static FMOD_UNIQUE                  : c_uint = 0x00020000;  /* Unique sound, can only be played one at a time */
pub static FMOD_3D_HEADRELATIVE         : c_uint = 0x00040000;  /* Make the sound's position, velocity and orientation relative to the listener. */
pub static FMOD_3D_WORLDRELATIVE        : c_uint = 0x00080000;  /* Make the sound's position, velocity and orientation absolute (relative to the world). (DEFAULT) */
pub static FMOD_3D_INVERSEROLLOFF       : c_uint = 0x00100000;  /* This sound will follow the inverse rolloff model where mindistance = full volume, maxdistance = where sound stops attenuating, and rolloff is fixed according to the global rolloff factor.  (DEFAULT) */
pub static FMOD_3D_LINEARROLLOFF        : c_uint = 0x00200000;  /* This sound will follow a linear rolloff model where mindistance = full volume, maxdistance = silence.  Rolloffscale is ignored. */
pub static FMOD_3D_LINEARSQUAREROLLOFF  : c_uint = 0x00400000; 	/* This sound will follow a linear-square rolloff model where mindistance = full volume, maxdistance = silence.  Rolloffscale is ignored. */
pub static FMOD_3D_CUSTOMROLLOFF        : c_uint = 0x04000000;  /* This sound will follow a rolloff model defined by Sound::set3DCustomRolloff / Channel::set3DCustomRolloff.  */
pub static FMOD_3D_IGNOREGEOMETRY       : c_uint = 0x40000000;  /* Is not affect by geometry occlusion.  If not specified in Sound::setMode, or Channel::setMode, the flag is cleared and it is affected by geometry again. */
pub static FMOD_UNICODE                 : c_uint = 0x01000000;  /* Filename is double-byte unicode. */
pub static FMOD_IGNORETAGS              : c_uint = 0x02000000;  /* Skips id3v2/asf/etc tag checks when opening a sound, to reduce seek/read overhead when opening files (helps with CD performance). */
pub static FMOD_LOWMEM                  : c_uint = 0x08000000;  /* Removes some features from samples to give a lower memory overhead, like Sound::getName.  See remarks. */
pub static FMOD_LOADSECONDARYRAM        : c_uint = 0x20000000;  /* Load sound into the secondary RAM of supported platform. On PS3, sounds will be loaded into RSX/VRAM. */
pub static FMOD_VIRTUAL_PLAYFROMSTART   : c_uint = 0x80000000;  /* For sounds that start virtual (due to being quiet or low importance), instead of swapping back to audible, and playing at the correct offset according to time, this flag makes the sound play from the start. */

pub static FMOD_INIT_NORMAL                     : c_uint = 0x00000000; /* All platforms - Initialize normally */
pub static FMOD_INIT_STREAM_FROM_UPDATE         : c_uint = 0x00000001; /* All platforms - No stream thread is created internally.  Streams are driven from System::update.  Mainly used with non-realtime outputs. */
pub static FMOD_INIT_3D_RIGHTHANDED             : c_uint = 0x00000002; /* All platforms - FMOD will treat +X as right, +Y as up and +Z as backwards (towards you). */
pub static FMOD_INIT_SOFTWARE_DISABLE           : c_uint = 0x00000004; /* All platforms - Disable software mixer to save memory.  Anything created with FMOD_SOFTWARE will fail and DSP will not work. */
pub static FMOD_INIT_OCCLUSION_LOWPASS          : c_uint = 0x00000008; /* All platforms - All FMOD_SOFTWARE (and FMOD_HARDWARE on 3DS and NGP) with FMOD_3D based voices will add a software lowpass filter effect into the DSP chain which is automatically used when Channel::set3DOcclusion is used or the geometry API. */
pub static FMOD_INIT_HRTF_LOWPASS               : c_uint = 0x00000010; /* All platforms - All FMOD_SOFTWARE (and FMOD_HARDWARE on 3DS and NGP) with FMOD_3D based voices will add a software lowpass filter effect into the DSP chain which causes sounds to sound duller when the sound goes behind the listener.  Use System::setAdvancedSettings to adjust cutoff frequency. */
pub static FMOD_INIT_DISTANCE_FILTERING         : c_uint = 0x00000200; /* All platforms - All FMOD_SOFTWARE with FMOD_3D based voices will add a software lowpass and highpass filter effect into the DSP chain which will act as a distance-automated bandpass filter. Use System::setAdvancedSettings to adjust the center frequency. */
pub static FMOD_INIT_REVERB_PREALLOCBUFFERS     : c_uint = 0x00000040; /* All platforms - FMOD Software reverb will preallocate enough buffers for reverb per channel, rather than allocating them and freeing them at runtime. */
pub static FMOD_INIT_ENABLE_PROFILE             : c_uint = 0x00000020; /* All platforms - Enable TCP/IP based host which allows FMOD Designer or FMOD Profiler to connect to it, and view memory, CPU and the DSP network graph in real-time. */
pub static FMOD_INIT_VOL0_BECOMES_VIRTUAL       : c_uint = 0x00000080; /* All platforms - Any sounds that are 0 volume will go virtual and not be processed except for having their positions updated virtually.  Use System::setAdvancedSettings to adjust what volume besides zero to switch to virtual at. */
pub static FMOD_INIT_WASAPI_EXCLUSIVE           : c_uint = 0x00000100; /* Win32 Vista only - for WASAPI output - Enable exclusive access to hardware, lower latency at the expense of excluding other applications from accessing the audio hardware. */
pub static FMOD_INIT_PS3_PREFERDTS              : c_uint = 0x00800000; /* PS3 only - Prefer DTS over Dolby Digital if both are supported. Note: 8 and 6 channel LPCM is always preferred over both DTS and Dolby Digital. */
pub static FMOD_INIT_PS3_FORCE2CHLPCM           : c_uint = 0x01000000; /* PS3 only - Force PS3 system output mode to 2 channel LPCM. */
pub static FMOD_INIT_DISABLEDOLBY               : c_uint = 0x00100000; /* Wii / 3DS - Disable Dolby Pro Logic surround. Speakermode will be set to STEREO even if user has selected surround in the system settings. */
pub static FMOD_INIT_SYSTEM_MUSICMUTENOTPAUSE   : c_uint = 0x00200000; /* Xbox 360 / PS3 - The "music" channelgroup which by default pauses when custom 360 dashboard / PS3 BGM music is played, can be changed to mute (therefore continues playing) instead of pausing, by using this flag. */
pub static FMOD_INIT_SYNCMIXERWITHUPDATE        : c_uint = 0x00400000; /* Win32/Wii/PS3/Xbox/Xbox 360 - FMOD Mixer thread is woken up to do a mix when System::update is called rather than waking periodically on its own timer. */
pub static FMOD_INIT_GEOMETRY_USECLOSEST        : c_uint = 0x04000000; /* All platforms - With the geometry engine, only process the closest polygon rather than accumulating all polygons the sound to listener line intersects. */
pub static FMOD_INIT_DISABLE_MYEARS_AUTODETECT  : c_uint = 0x08000000; /* Win32 - Disables automatic setting of FMOD_SPEAKERMODE_STEREO to FMOD_SPEAKERMODE_MYEARS if the MyEars profile exists on the PC.  MyEars is HRTF 7.1 downmixing through headphones. */
pub static FMOD_INIT_PS3_DISABLEDTS             : c_uint = 0x10000000; /* PS3 only - Disable DTS output mode selection */
pub static FMOD_INIT_PS3_DISABLEDOLBYDIGITAL    : c_uint = 0x20000000; /* PS3 only - Disable Dolby Digital output mode selection */
pub static FMOD_INIT_7POINT1_DOLBYMAPPING       : c_uint = 0x40000000; /* PS3/PS4 only - FMOD uses the WAVEFORMATEX Microsoft 7.1 speaker mapping where the last 2 pairs of speakers are 'rears' then 'sides', but on PS3/PS4 these are mapped to 'surrounds' and 'backs'.  Use this flag to swap fmod's last 2 pair of speakers on PS3/PS4 to avoid needing to do a special case for these platforms. */


extern "C" {
	fn FMOD_System_Create(system: *FMOD_SYSTEM) -> FMOD_RESULT;
	fn FMOD_System_Init(system: FMOD_SYSTEM, maxchannels: c_int, flags: FMOD_INITFLAGS, extradriverdata: *c_void) -> FMOD_RESULT;
	fn FMOD_System_Close(sound: FMOD_SOUND) -> FMOD_RESULT;
	fn FMOD_System_Release(system: FMOD_SYSTEM) -> FMOD_RESULT;
	fn FMOD_System_CreateSound(system: FMOD_SYSTEM, name_or_data: *c_char, mode: FMOD_MODE, exinfo: *FMOD_CREATESOUNDEXINFO,
		sound: *FMOD_SOUND) -> FMOD_RESULT;

	fn FMOD_System_PlaySound(system : FMOD_SYSTEM, channel_id : FMOD_CHANNELINDEX, sound : FMOD_SOUND, paused : FMOD_BOOL,
		channel : *FMOD_CHANNEL) -> FMOD_RESULT;
	fn FMOD_Sound_Release(sound: FMOD_SOUND) -> FMOD_RESULT;

	fn FMOD_System_GetSpectrum(system : FMOD_SYSTEM, array : *c_float, num_values : c_int, channel_offset : c_int,
		window_type : FMOD_DSP_FFT_WINDOW) -> FMOD_RESULT;
	fn FMOD_Channel_IsPlaying(channel : FMOD_CHANNEL, is_playing : *FMOD_BOOL) -> FMOD_RESULT;
	fn FMOD_Channel_SetVolume(channel : FMOD_CHANNEL, volume : c_float) -> FMOD_RESULT;
	fn FMOD_Channel_GetVolume(channel : FMOD_CHANNEL, volume : *c_float) -> FMOD_RESULT;
	fn FMOD_Channel_SetFrequency(channel : FMOD_CHANNEL, frequency : c_float) -> FMOD_RESULT;
	fn FMOD_Channel_GetFrequency(channel : FMOD_CHANNEL, frequency : *c_float) -> FMOD_RESULT;
	fn FMOD_Channel_SetPan(channel : FMOD_CHANNEL, pan : c_float) -> FMOD_RESULT;
	fn FMOD_Channel_GetPan(channel : FMOD_CHANNEL, pan : *c_float) -> FMOD_RESULT;
	fn FMOD_Channel_SetMute(channel : FMOD_CHANNEL, mute : FMOD_BOOL) -> FMOD_RESULT;
	fn FMOD_Channel_GetMute(channel : FMOD_CHANNEL, mute : *FMOD_BOOL) -> FMOD_RESULT;
	fn FMOD_Channel_SetPaused(channel : FMOD_CHANNEL, pause : FMOD_BOOL) -> FMOD_RESULT;
	fn FMOD_Channel_GetPaused(channel : FMOD_CHANNEL, pause : *FMOD_BOOL) -> FMOD_RESULT;
	fn FMOD_Channel_SetDelay(channel : FMOD_CHANNEL, delaytype : FMOD_DELAYTYPE, delayhi : c_uint, delaylo : c_uint) -> FMOD_RESULT;
	fn FMOD_Channel_GetDelay(channel : FMOD_CHANNEL, delaytype : FMOD_DELAYTYPE, delayhi : *c_uint, delaylo : *c_uint) -> FMOD_RESULT;
	fn FMOD_Channel_SetSpeakerMix(channel : FMOD_CHANNEL, frontleft : c_float, frontright : c_float, center : c_float, lfe : c_float,
								backleft : c_float, backright : c_float, sideleft : c_float, sideright : c_float) -> FMOD_RESULT;
	fn FMOD_Channel_GetSpeakerMix(channel : FMOD_CHANNEL, frontleft : *c_float, frontright : *c_float, center : *c_float, lfe : *c_float, backleft : *c_float,
								backright : *c_float, sideleft : *c_float, sideright : *c_float) -> FMOD_RESULT;
	fn FMOD_Channel_SetSpeakerLevels(channel : FMOD_CHANNEL, speaker : FMOD_SPEAKER, levels : *c_float, numlevels : c_int) -> FMOD_RESULT;
	fn FMOD_Channel_GetSpeakerLevels(channel : FMOD_CHANNEL, speaker : FMOD_SPEAKER, levels : *c_float, numlevels : c_int) -> FMOD_RESULT;
	fn FMOD_Channel_SetInputChannelMix(channel : FMOD_CHANNEL, levels : *c_float, numlevels : c_int) -> FMOD_RESULT;
	fn FMOD_Channel_GetInputChannelMix(channel : FMOD_CHANNEL, levels : *c_float, numlevels : c_int) -> FMOD_RESULT;
	fn FMOD_Channel_SetPriority(channel : FMOD_CHANNEL, priority : c_int) -> FMOD_RESULT;
	fn FMOD_Channel_GetPriority(channel : FMOD_CHANNEL, priority : *c_int) -> FMOD_RESULT;
	fn FMOD_Channel_SetPosition(channel : FMOD_CHANNEL, position : c_uint, postype : FMOD_TIMEUNIT) -> FMOD_RESULT;
	fn FMOD_Channel_GetPosition(channel : FMOD_CHANNEL, position : *c_uint, postype : FMOD_TIMEUNIT) -> FMOD_RESULT;
	fn FMOD_Channel_SetReverbProperties(channel : FMOD_CHANNEL, prop : *FMOD_REVERB_CHANNELPROPERTIES) -> FMOD_RESULT;
	fn FMOD_Channel_GetReverbProperties(channel : FMOD_CHANNEL, prop : *FMOD_REVERB_CHANNELPROPERTIES) -> FMOD_RESULT;
	fn FMOD_Channel_SetLowPassGain(channel : FMOD_CHANNEL, gain : c_float) -> FMOD_RESULT;
	fn FMOD_Channel_GetLowPassGain(channel : FMOD_CHANNEL, gain : *c_float) -> FMOD_RESULT;
	
	//fn FMOD_Channel_SetChannelGroup(channel : FMOD_CHANNEL, channelgroup : FMOD_CHANNEL_GROUP) -> FMOD_RESULT;
	//fn FMOD_Channel_GetChannelGroup(channel : FMOD_CHANNEL, channelgroup : *FMOD_CHANNEL_GROUP) -> FMOD_RESULT;
}

pub struct SpectrumOptions {
	pub window_type 	: FMOD_DSP_FFT_WINDOW,
	pub channel_offset 	: c_int
}

pub struct DelayOptions {
	pub delaytype 	: FMOD_DELAYTYPE,
	pub delayhi 	: uint,
	pub delaylo 	: uint
}

pub struct SpeakerMixOptions {
	pub front_left 	: f32,
	pub front_right : f32,
	pub center 		: f32,
	pub lfe 		: f32,
	pub back_left 	: f32,
	pub back_right 	: f32,
	pub side_left 	: f32,
	pub side_right 	: f32
}

pub struct Channel {
	channel : FMOD_CHANNEL,
}

impl Channel {
	fn new() -> Channel {
		Channel{channel : ::std::ptr::null()}
	}

	fn release(&mut self) {
		self.channel = ::std::ptr::null();
	}

	pub fn get_spectrum(&self, num_values : uint, options : Option<SpectrumOptions>) -> Result<Vec<f32>, FMOD_RESULT> {
		let ptr = Vec::from_elem(num_values, 0f32);
		let mut window_type = FMOD_DSP_FFT_WINDOW_RECT;
		let mut channel_offset = 0;

		match options {
			Some(v) => {
				window_type = v.window_type;
				channel_offset = v.channel_offset;
			}
			None => {}
		};
		match unsafe { FMOD_System_GetSpectrum(self.channel, ptr.as_ptr(), num_values as c_int, channel_offset, window_type) } {
			FMOD_OK => Ok(ptr),
			e => Err(e),
		}
	}

	pub fn is_playing(&self) -> Result<bool, FMOD_RESULT> {
		let is_playing = 0;

		match unsafe { FMOD_Channel_IsPlaying(self.channel, &is_playing) } {
			FMOD_OK => Ok(is_playing == 1),
			err => Err(err),
		}
	}

	pub fn set_volume(&self, volume : f32) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetVolume(self.channel, volume) }
	}

	pub fn get_volume(&self) -> Result<f32, FMOD_RESULT> {
		let volume = 0f32;

		match unsafe { FMOD_Channel_GetVolume(self.channel, &volume) } {
			FMOD_OK => Ok(volume),
			e => Err(e),
		}
	}

	pub fn set_frequency(&self, frequency : f32) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetFrequency(self.channel, frequency) }
	}

	pub fn get_frequency(&self) -> Result<f32, FMOD_RESULT> {
		let frequency = 0f32;

		match unsafe { FMOD_Channel_GetFrequency(self.channel, &frequency) } {
			FMOD_OK => Ok(frequency),
			e => Err(e),
		}
	}

	pub fn set_pan(&self, pan : f32) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetPan(self.channel, pan) }
	}

	pub fn get_pan(&self) -> Result<f32, FMOD_RESULT> {
		let pan = 0f32;

		match unsafe { FMOD_Channel_GetPan(self.channel, &pan) } {
			FMOD_OK => Ok(pan),
			e => Err(e),
		}
	}

	pub fn set_mute(&self, mute : bool) -> FMOD_RESULT {
		let t = match mute {
			true => 1,
			false => 0,
		};
		unsafe { FMOD_Channel_SetMute(self.channel, t) }
	}

	pub fn get_mute(&self) -> Result<bool, FMOD_RESULT> {
		let mute = 0;

		match unsafe { FMOD_Channel_GetMute(self.channel, &mute) } {
			FMOD_OK => Ok(match mute {
				1 => true,
				_ => false,
			}),
			e => Err(e),
		}
	}

	pub fn set_paused(&self, paused : bool) -> FMOD_RESULT {
		let t : FMOD_BOOL = match paused {
			true => 1,
			false => 0,
		};
		unsafe { FMOD_Channel_SetPaused(self.channel, t) }
	}

	pub fn get_paused(&self) -> Result<bool, FMOD_RESULT> {
		let t = 0;

		match unsafe { FMOD_Channel_GetPaused(self.channel, &t) } {
			FMOD_OK => Ok(match t {
				1 => true,
				_ => false,
			}),
			e => Err(e),
		}
	}

	pub fn set_delay(&self, d_o : DelayOptions) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetDelay(self.channel, d_o.delaytype, d_o.delayhi as u32, d_o.delaylo as u32) }
	}

	pub fn get_delay(&self, delaytype : FMOD_DELAYTYPE) -> Result<DelayOptions, FMOD_RESULT> {
		let delaylo = 0u32;
		let delayhi = 0u32;

		match unsafe { FMOD_Channel_GetDelay(self.channel, delaytype, &delayhi, &delaylo) } {
			FMOD_OK => Ok(DelayOptions{delaytype: delaytype, delayhi: delayhi as uint, delaylo: delaylo as uint}),
			e => Err(e),
		}
	}

	pub fn set_speaker_mix(&self, smo : SpeakerMixOptions) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetSpeakerMix(self.channel, smo.front_left, smo.front_right, smo.center, smo.lfe,
											smo.back_left, smo.back_right, smo.side_left, smo.side_right) }
	}

	pub fn get_speaker_mix(&self) -> Result<SpeakerMixOptions, FMOD_RESULT> {
		let smo = SpeakerMixOptions{front_left: 0f32, front_right: 0f32, center: 0f32, lfe: 0f32, back_left: 0f32,
									back_right: 0f32, side_left: 0f32, side_right: 0f32};

		match unsafe { FMOD_Channel_GetSpeakerMix(self.channel, &smo.front_left, &smo.front_right, &smo.center, &smo.lfe,
												&smo.back_left, &smo.back_right, &smo.side_left, &smo.side_right) } {
			FMOD_OK => Ok(smo),
			e => Err(e),
		}
	}

	pub fn set_speaker_level(&self, speaker : FMOD_SPEAKER, levels : Vec<f32>) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetSpeakerLevels(self.channel, speaker, levels.as_ptr(), levels.len() as i32) }
	}

	pub fn get_speaker_level(&self, speaker : FMOD_SPEAKER, num_levels : uint) -> Result<Vec<f32>, FMOD_RESULT> {
		let ptr = Vec::from_elem(num_levels, 0f32);

		match unsafe { FMOD_Channel_GetSpeakerLevels(self.channel, speaker, ptr.as_ptr(), num_levels as i32) } {
			FMOD_OK => Ok(ptr),
			e => Err(e),
		}
	}

	pub fn set_input_channel_mix(&self, levels : Vec<f32>) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetInputChannelMix(self.channel, levels.as_ptr(), levels.len() as i32) }
	}

	pub fn get_input_channel_mix(&self, num_levels : uint) -> Result<Vec<f32>, FMOD_RESULT> {
		let ptr = Vec::from_elem(num_levels, 0f32);

		match unsafe { FMOD_Channel_GetInputChannelMix(self.channel, ptr.as_ptr(), num_levels as i32) } {
			FMOD_OK => Ok(ptr),
			e => Err(e),
		}
	}

	pub fn set_priority(&self, priority : i32) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetPriority(self.channel, priority) }
	}

	pub fn get_priority(&self) -> Result<i32, FMOD_RESULT> {
		let t = 0i32;

		match unsafe { FMOD_Channel_GetPriority(self.channel, &t) } {
			FMOD_OK => Ok(t),
			e => Err(e),
		}
	}

	pub fn set_position(&self, position : uint, postype : FMOD_TIMEUNIT) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetPosition(self.channel, position as u32, postype) }
	}

	pub fn get_position(&self, postype : FMOD_TIMEUNIT) -> Result<uint, FMOD_RESULT> {
		let t = 0u32;

		match unsafe { FMOD_Channel_GetPosition(self.channel, &t, postype) } {
			FMOD_OK => Ok(t as uint),
			e => Err(e),
		}
	}

	pub fn set_reverb_properties(&self, prop : FMOD_REVERB_CHANNELPROPERTIES) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetReverbProperties(self.channel, &prop) }
	}

	pub fn get_reverb_properties(&self) -> Result<FMOD_REVERB_CHANNELPROPERTIES, FMOD_RESULT> {
		let t = FMOD_REVERB_CHANNELPROPERTIES{Direct : 0, Room : 0, Flags : 0, ConnectionPoint : ::std::ptr::null()};

		match unsafe { FMOD_Channel_GetReverbProperties(self.channel, &t) } {
			FMOD_OK => Ok(t),
			e => Err(e),
		}
	}

	pub fn set_low_pass_gain(&self, gain : f32) -> FMOD_RESULT {
		unsafe { FMOD_Channel_SetLowPassGain(self.channel, gain) }
	}

	pub fn get_low_pass_gain(&self) -> Result<f32, FMOD_RESULT> {
		let t = 0f32;

		match unsafe { FMOD_Channel_GetLowPassGain(self.channel, &t) } {
			FMOD_OK => Ok(t),
			e => Err(e),
		}
	}
}

pub struct Sound {
	sound 		: FMOD_SOUND,
	pub name 	: StrBuf,
	system 		: FMOD_SYSTEM,
	channel 	: Channel,
}

impl Sound {
	fn new(system : FMOD_SYSTEM, name : StrBuf, sound: FMOD_SOUND) -> Sound {
		Sound{sound: sound, channel: Channel::new(), name: name.clone(), system: system}
	}
	pub fn release(&mut self) -> FMOD_RESULT {
		self.system = ::std::ptr::null();
		self.channel.release();
		unsafe { FMOD_Sound_Release(self.sound) }
	}

	pub fn play(&self) -> FMOD_RESULT {
		if self.channel.channel == ::std::ptr::null() {
			unsafe { FMOD_System_PlaySound(self.system, FMOD_CHANNEL_FREE, self.sound, 0, &self.channel.channel) }
		} else {
			self.channel.set_paused(false)
		}
	}

	pub fn pause(&self) -> FMOD_RESULT {
		self.channel.set_paused(true)
	}

	pub fn is_playing(&self) -> Result<bool, FMOD_RESULT> {
		self.channel.is_playing()
	}

	fn play_loop(&self) -> FMOD_RESULT {
		match self.is_playing() {
			Ok(b) => {
				if b == true {
					sleep(30);
					self.play_loop()
				} else {
					FMOD_OK
				}
			},
			Err(e) => e,
		}
	}

	pub fn play_to_the_end(&self) -> FMOD_RESULT {
		match self.play() {
			FMOD_OK => self.play_loop(),
			err => err,
		}
	}

	pub fn get_channel<'a>(&'a self) -> &'a Channel {
		&self.channel
	}
}

pub struct Fmod {
	system : FMOD_SYSTEM,
}

impl Fmod {
	pub fn new() -> Result<Fmod, FMOD_RESULT> {
		let tmp = ::std::ptr::null();

		match unsafe { FMOD_System_Create(&tmp) } {
			FMOD_OK => {
				match unsafe { FMOD_System_Init(tmp, 1, FMOD_INIT_NORMAL, ::std::ptr::null()) } {
					FMOD_OK => Ok(Fmod{system : tmp}),
					err => {
						unsafe { FMOD_System_Release(tmp) };
						Err(err)
					}
				}
			},
			err => Err(err),
		}
	}

	pub fn release(&self) -> FMOD_RESULT {
		unsafe {
			FMOD_System_Close(self.system);
			FMOD_System_Release(self.system)
		}
	}

	pub fn create_sound(&self, music : StrBuf) -> Result<Sound, FMOD_RESULT> {
	   	let tmp_v = music.clone().into_owned();
	   	let sound = ::std::ptr::null();

	    tmp_v.with_c_str(|c_str|{
	        match unsafe { FMOD_System_CreateSound(self.system, c_str,
	           								FMOD_SOFTWARE | FMOD_LOOP_OFF | FMOD_2D | FMOD_CREATESTREAM, ::std::ptr::null(), &sound) } {
	           	FMOD_OK => {Ok(Sound::new(self.system, music.clone(), sound))},
	           	err => Err(err),
	        }
	    })
	}	
}