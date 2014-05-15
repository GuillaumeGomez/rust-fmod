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

use libc::{c_void, c_uint, c_int, c_char, c_float};

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
pub enum FMOD_OUTPUTTYPE
{
    FMOD_OUTPUTTYPE_AUTODETECT,      /* Picks the best output mode for the platform.  This is the default. */
                                     
    FMOD_OUTPUTTYPE_UNKNOWN,         /* All             - 3rd party plugin, unknown.  This is for use with System::getOutput only. */
    FMOD_OUTPUTTYPE_NOSOUND,         /* All             - All calls in this mode succeed but make no sound. */
    FMOD_OUTPUTTYPE_WAVWRITER,       /* All             - Writes output to fmodoutput.wav by default.  Use the 'extradriverdata' parameter in System::init, by simply passing the filename as a string, to set the wav filename. */
    FMOD_OUTPUTTYPE_NOSOUND_NRT,     /* All             - Non-realtime version of FMOD_OUTPUTTYPE_NOSOUND.  User can drive mixer with System::update at whatever rate they want. */
    FMOD_OUTPUTTYPE_WAVWRITER_NRT,   /* All             - Non-realtime version of FMOD_OUTPUTTYPE_WAVWRITER.  User can drive mixer with System::update at whatever rate they want. */
                                     
    FMOD_OUTPUTTYPE_DSOUND,          /* Win32/Win64     - DirectSound output.                       (Default on Windows XP and below) */
    FMOD_OUTPUTTYPE_WINMM,           /* Win32/Win64     - Windows Multimedia output. */
    FMOD_OUTPUTTYPE_WASAPI,          /* Win32           - Windows Audio Session API.                (Default on Windows Vista and above) */
    FMOD_OUTPUTTYPE_ASIO,            /* Win32           - Low latency ASIO 2.0 driver. */
    FMOD_OUTPUTTYPE_OSS,             /* Linux/Linux64   - Open Sound System output.                 (Default on Linux, third preference) */
    FMOD_OUTPUTTYPE_ALSA,            /* Linux/Linux64   - Advanced Linux Sound Architecture output. (Default on Linux, second preference if available) */
    FMOD_OUTPUTTYPE_ESD,             /* Linux/Linux64   - Enlightment Sound Daemon output. */
    FMOD_OUTPUTTYPE_PULSEAUDIO,      /* Linux/Linux64   - PulseAudio output.                        (Default on Linux, first preference if available) */
    FMOD_OUTPUTTYPE_COREAUDIO,       /* Mac             - Macintosh CoreAudio output.               (Default on Mac) */
    FMOD_OUTPUTTYPE_XBOX360,         /* Xbox 360        - Native Xbox360 output.                    (Default on Xbox 360) */
    FMOD_OUTPUTTYPE_PSP,             /* PSP             - Native PSP output.                        (Default on PSP) */
    FMOD_OUTPUTTYPE_PS3,             /* PS3             - Native PS3 output.                        (Default on PS3) */
    FMOD_OUTPUTTYPE_NGP,             /* NGP             - Native NGP output.                        (Default on NGP) */
    FMOD_OUTPUTTYPE_WII,             /* Wii             - Native Wii output.                        (Default on Wii) */
    FMOD_OUTPUTTYPE_3DS,             /* 3DS             - Native 3DS output                         (Default on 3DS) */
    FMOD_OUTPUTTYPE_AUDIOTRACK,      /* Android         - Java Audio Track output.                  (Default on Android 2.2 and below) */
    FMOD_OUTPUTTYPE_OPENSL,          /* Android         - OpenSL ES output.                         (Default on Android 2.3 and above) */   
    FMOD_OUTPUTTYPE_NACL,            /* Native Client   - Native Client output.                     (Default on Native Client) */
    FMOD_OUTPUTTYPE_WIIU,            /* Wii U           - Native Wii U output.                      (Default on Wii U) */
    FMOD_OUTPUTTYPE_ASOUND,          /* BlackBerry      - Native BlackBerry asound output.          (Default on BlackBerry) */
    FMOD_OUTPUTTYPE_AUDIOOUT,        /* Orbis           - Audio Out output.                         (Default on Orbis) */
    FMOD_OUTPUTTYPE_XAUDIO,          /* Durango         - XAudio2 output. */

    FMOD_OUTPUTTYPE_MAX,             /* Maximum number of output types supported. */
    FMOD_OUTPUTTYPE_FORCEINT = 65536 /* Makes sure this enum is signed 32bit. */
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

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_SPEAKERMODE
{
    FMOD_SPEAKERMODE_RAW,              /* There is no specific speakermode.  Sound channels are mapped in order of input to output.  Use System::setSoftwareFormat to specify speaker count. See remarks for more information. */
    FMOD_SPEAKERMODE_MONO,             /* The speakers are monaural. */
    FMOD_SPEAKERMODE_STEREO,           /* The speakers are stereo (DEFAULT). */
    FMOD_SPEAKERMODE_QUAD,             /* 4 speaker setup.  This includes front left, front right, rear left, rear right.  */
    FMOD_SPEAKERMODE_SURROUND,         /* 5 speaker setup.  This includes front left, front right, center, rear left, rear right. */
    FMOD_SPEAKERMODE_5POINT1,          /* 5.1 speaker setup.  This includes front left, front right, center, rear left, rear right and a subwoofer. */
    FMOD_SPEAKERMODE_7POINT1,          /* 7.1 speaker setup.  This includes front left, front right, center, rear left, rear right, side left, side right and a subwoofer. */
    
    FMOD_SPEAKERMODE_SRS5_1_MATRIX,    /* Stereo compatible output, embedded with surround information. SRS 5.1/Prologic/Prologic2 decoders will split the signal into a 5.1 speaker set-up or SRS virtual surround will decode into a 2-speaker/headphone setup.  See remarks about limitations.*/
    FMOD_SPEAKERMODE_DOLBY5_1_MATRIX,  /* Stereo compatible output, embedded with surround information. Dolby Pro Logic II decoders will split the signal into a 5.1 speaker set-up. */
    FMOD_SPEAKERMODE_MYEARS,           /* Stereo output, but data is encoded using personalized HRTF algorithms.  See myears.net.au */

    FMOD_SPEAKERMODE_MAX,              /* Maximum number of speaker modes supported. */
    FMOD_SPEAKERMODE_FORCEINT = 65536  /* Makes sure this enum is signed 32bit. */
}

#[deriving(Eq, Ord, Show)]
#[repr(C)]
pub enum FMOD_DSP_RESAMPLER
{
    FMOD_DSP_RESAMPLER_NOINTERP,        /* No interpolation.  High frequency aliasing hiss will be audible depending on the sample rate of the sound. */
    FMOD_DSP_RESAMPLER_LINEAR,          /* Linear interpolation (default method).  Fast and good quality, causes very slight lowpass effect on low frequency sounds. */
    FMOD_DSP_RESAMPLER_CUBIC,           /* Cubic interpolation.  Slower than linear interpolation but better quality. */
    FMOD_DSP_RESAMPLER_SPLINE,          /* 5 point spline interpolation.  Slowest resampling method but best quality. */

    FMOD_DSP_RESAMPLER_MAX,             /* Maximum number of resample methods supported. */
    FMOD_DSP_RESAMPLER_FORCEINT = 65536 /* Makes sure this enum is signed 32bit. */
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
pub static FMOD_3D_LINEARSQUAREROLLOFF  : c_uint = 0x00400000;  /* This sound will follow a linear-square rolloff model where mindistance = full volume, maxdistance = silence.  Rolloffscale is ignored. */
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