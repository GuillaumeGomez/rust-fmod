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

extern crate libc;

use enums::*;
use libc::{c_void, c_uint, c_int, c_char, c_float, c_ushort, c_uchar, c_short};

mod enums;

pub type FMOD_FILE_OPENCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_char, arg2: int, arg3: *c_uint, arg4: **c_void, arg5: **c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_CLOSECALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_READCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void, arg3: c_uint, arg4: *c_uint, arg5: *c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_SEEKCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: c_uint, arg3: *c_void) -> FMOD_RESULT>;
pub type FMOD_SOUND_NONBLOCKCALLBACK = ::std::option::Option<extern "C" fn(arg1: FMOD_SOUND, arg2: FMOD_RESULT) -> FMOD_RESULT>;
pub type FMOD_SOUND_PCMREADCALLBACK = ::std::option::Option<extern "C" fn(arg1: FMOD_SOUND, arg2: *c_void, arg3: c_uint) -> FMOD_RESULT>;
pub type FMOD_FILE_ASYNCREADCALLBACK = ::std::option::Option<extern "C" fn(arg1: *FMOD_ASYNCREADINFO, arg2: *c_void) -> FMOD_RESULT>;
pub type FMOD_FILE_ASYNCCANCELCALLBACK = ::std::option::Option<extern "C" fn(arg1: *c_void, arg2: *c_void, arg3: c_uint) -> FMOD_RESULT>;
pub type FMOD_SOUND_PCMSETPOSCALLBACK = ::std::option::Option<extern "C" fn(arg1: FMOD_SOUND, arg2: c_int, arg3: c_uint, arg4: FMOD_TIMEUNIT) -> FMOD_RESULT>;

pub type FMOD_SYSTEM = *c_void;
pub type FMOD_SOUND = *c_void;
pub type FMOD_CHANNEL = *c_void;
pub type FMOD_CHANNEL_GROUP = *c_void;
pub type FMOD_BOOL = c_int;
pub type FMOD_MODE = c_uint;
pub type FMOD_TIMEUNIT = c_uint;
pub type FMOD_SOUNDGROUP = c_void;
pub type FMOD_INITFLAGS = c_uint;
pub type FMOD_DSP = c_void;
pub type FMOD_CAPS = c_uint;

extern "C" {
    /* pre-init functions */
    pub fn FMOD_System_Create(system: *FMOD_SYSTEM) -> FMOD_RESULT;
    pub fn FMOD_System_SetOutput(system: FMOD_SYSTEM, output_type: FMOD_OUTPUTTYPE) -> FMOD_RESULT;
    pub fn FMOD_System_GetOutput(system: FMOD_SYSTEM, output_type: *FMOD_OUTPUTTYPE) -> FMOD_RESULT;
    pub fn FMOD_System_GetNumDrivers(system: FMOD_SYSTEM, num_drivers: *c_int) -> FMOD_RESULT;
    pub fn FMOD_System_GetDriverInfo(system: FMOD_SYSTEM, id: c_int, name: *c_char, name_len: c_int, guid: *FMOD_GUID) -> FMOD_RESULT;
    pub fn FMOD_System_GetDriverInfoW(system: FMOD_SYSTEM, id: c_int, name: *c_short, name_len: c_int, guid: *FMOD_GUID) -> FMOD_RESULT;
    pub fn FMOD_System_GetDriverCaps(system: FMOD_SYSTEM, id: c_int, caps: *FMOD_CAPS, control_panel_output_rate: *c_int,
        controlpanelspeakermode: *FMOD_SPEAKERMODE) -> FMOD_RESULT;
    pub fn FMOD_System_SetDriver(system: FMOD_SYSTEM, driver: c_int) -> FMOD_RESULT;
    pub fn FMOD_System_GetDriver(system: FMOD_SYSTEM, driver: *c_int) -> FMOD_RESULT;
    pub fn FMOD_System_SetHardwareChannels(system: FMOD_SYSTEM, num_hardware_channels: c_int) -> FMOD_RESULT;
    pub fn FMOD_System_GetHardwareChannels(system: FMOD_SYSTEM, num_hardware_channels: *c_int) -> FMOD_RESULT;
    pub fn FMOD_System_SetSoftwareChannels(system: FMOD_SYSTEM, num_software_channels: c_int) -> FMOD_RESULT;
    pub fn FMOD_System_GetSoftwareChannels(system: FMOD_SYSTEM, num_software_channels: *c_int) -> FMOD_RESULT;
    pub fn FMOD_System_SetSoftwareFormat(system: FMOD_SYSTEM, sample_rate: c_int, format: FMOD_SOUND_FORMAT, num_output_channels: c_int,
        max_input_channels: c_int, resample_method: FMOD_DSP_RESAMPLER) -> FMOD_RESULT;
    pub fn FMOD_System_GetSoftwareFormat(system: FMOD_SYSTEM, sample_rate: *c_int, format: *FMOD_SOUND_FORMAT,
        num_output_channels: *c_int, max_input_channels: *c_int, resample_method: *FMOD_DSP_RESAMPLER, bits: *c_int) -> FMOD_RESULT;
    pub fn FMOD_System_SetDSPBufferSize(system: FMOD_SYSTEM, buffer_length: c_uint, num_buffers: c_int) -> FMOD_RESULT;
    pub fn FMOD_System_GetDSPBufferSize(system: FMOD_SYSTEM, buffer_length: *c_uint, num_buffers: *c_int) -> FMOD_RESULT;
    //I'll bind it a little later
    pub fn FMOD_System_SetFileSystem(system: FMOD_SYSTEM, user_open: FMOD_FILE_OPENCALLBACK, user_close: FMOD_FILE_CLOSECALLBACK,
        user_read: FMOD_FILE_READCALLBACK, user_seek: FMOD_FILE_SEEKCALLBACK, user_async_read: FMOD_FILE_ASYNCREADCALLBACK,
        user_async_cancel: FMOD_FILE_ASYNCCANCELCALLBACK, block_align: c_int) -> FMOD_RESULT;
    //I'll bind it a little later
    pub fn FMOD_System_AttachFileSystem(system: FMOD_SYSTEM, user_open: FMOD_FILE_OPENCALLBACK, user_close: FMOD_FILE_CLOSECALLBACK,
        user_read: FMOD_FILE_READCALLBACK, user_seek: FMOD_FILE_SEEKCALLBACK) -> FMOD_RESULT;
    pub fn FMOD_System_SetAdvancedSettings(system: FMOD_SYSTEM, settings: *FMOD_ADVANCEDSETTINGS) -> FMOD_RESULT;
    pub fn FMOD_System_GetAdvancedSettings(system: FMOD_SYSTEM, settings: *FMOD_ADVANCEDSETTINGS) -> FMOD_RESULT;


    pub fn FMOD_System_Init(system: FMOD_SYSTEM, max_channels: c_int, flags: FMOD_INITFLAGS, extra_driver_data: *c_void) -> FMOD_RESULT;
    pub fn FMOD_System_Close(sound: FMOD_SOUND) -> FMOD_RESULT;
    pub fn FMOD_System_Release(system: FMOD_SYSTEM) -> FMOD_RESULT;
    pub fn FMOD_System_CreateSound(system: FMOD_SYSTEM, name_or_data: *c_char, mode: FMOD_MODE, exinfo: *FMOD_CREATESOUNDEXINFO,
        sound: *FMOD_SOUND) -> FMOD_RESULT;


    /* sound functions */
    pub fn FMOD_System_PlaySound(system : FMOD_SYSTEM, channel_id : FMOD_CHANNELINDEX, sound : FMOD_SOUND, paused : FMOD_BOOL,
        channel : *FMOD_CHANNEL) -> FMOD_RESULT;
    pub fn FMOD_Sound_Release(sound: FMOD_SOUND) -> FMOD_RESULT;


    /* channel functions */
    pub fn FMOD_System_GetSpectrum(system : FMOD_SYSTEM, array : *c_float, num_values : c_int, channel_offset : c_int,
        window_type : FMOD_DSP_FFT_WINDOW) -> FMOD_RESULT;
    pub fn FMOD_Channel_IsPlaying(channel : FMOD_CHANNEL, is_playing : *FMOD_BOOL) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetVolume(channel : FMOD_CHANNEL, volume : c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetVolume(channel : FMOD_CHANNEL, volume : *c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetFrequency(channel : FMOD_CHANNEL, frequency : c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetFrequency(channel : FMOD_CHANNEL, frequency : *c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetPan(channel : FMOD_CHANNEL, pan : c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetPan(channel : FMOD_CHANNEL, pan : *c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetMute(channel : FMOD_CHANNEL, mute : FMOD_BOOL) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetMute(channel : FMOD_CHANNEL, mute : *FMOD_BOOL) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetPaused(channel : FMOD_CHANNEL, pause : FMOD_BOOL) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetPaused(channel : FMOD_CHANNEL, pause : *FMOD_BOOL) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetDelay(channel : FMOD_CHANNEL, delay_type : FMOD_DELAYTYPE, delayhi : c_uint, delaylo : c_uint) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetDelay(channel : FMOD_CHANNEL, delay_type : FMOD_DELAYTYPE, delayhi : *c_uint, delaylo : *c_uint) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetSpeakerMix(channel : FMOD_CHANNEL, front_left : c_float, front_right : c_float, center : c_float, lfe : c_float,
        back_left : c_float, back_right : c_float, side_left : c_float, side_right : c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetSpeakerMix(channel : FMOD_CHANNEL, front_left : *c_float, front_right : *c_float, center : *c_float, lfe : *c_float,
        back_left : *c_float, back_right : *c_float, side_left : *c_float, side_right : *c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetSpeakerLevels(channel : FMOD_CHANNEL, speaker : FMOD_SPEAKER, levels : *c_float, num_levels : c_int) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetSpeakerLevels(channel : FMOD_CHANNEL, speaker : FMOD_SPEAKER, levels : *c_float, num_levels : c_int) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetInputChannelMix(channel : FMOD_CHANNEL, levels : *c_float, num_levels : c_int) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetInputChannelMix(channel : FMOD_CHANNEL, levels : *c_float, num_levels : c_int) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetPriority(channel : FMOD_CHANNEL, priority : c_int) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetPriority(channel : FMOD_CHANNEL, priority : *c_int) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetPosition(channel : FMOD_CHANNEL, position : c_uint, postype : FMOD_TIMEUNIT) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetPosition(channel : FMOD_CHANNEL, position : *c_uint, postype : FMOD_TIMEUNIT) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetReverbProperties(channel : FMOD_CHANNEL, prop : *FMOD_REVERB_CHANNELPROPERTIES) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetReverbProperties(channel : FMOD_CHANNEL, prop : *FMOD_REVERB_CHANNELPROPERTIES) -> FMOD_RESULT;
    pub fn FMOD_Channel_SetLowPassGain(channel : FMOD_CHANNEL, gain : c_float) -> FMOD_RESULT;
    pub fn FMOD_Channel_GetLowPassGain(channel : FMOD_CHANNEL, gain : *c_float) -> FMOD_RESULT;
    
    //pub fn FMOD_Channel_SetChannelGroup(channel : FMOD_CHANNEL, channelgroup : FMOD_CHANNEL_GROUP) -> FMOD_RESULT;
    //pub fn FMOD_Channel_GetChannelGroup(channel : FMOD_CHANNEL, channelgroup : *FMOD_CHANNEL_GROUP) -> FMOD_RESULT;
}

pub struct FMOD_ASYNCREADINFO
{
    handle      : *c_void,      /* [r] The file handle that was filled out in the open callback. */
    offset      : c_uint,       /* [r] Seek position, make sure you read from this file offset. */
    sizebytes   : c_uint,       /* [r] how many bytes requested for read. */
    priority    : c_int,        /* [r] 0 = low importance.  100 = extremely important (ie 'must read now or stuttering may occur') */

    buffer      : *c_void,      /* [w] Buffer to read file data into. */
    bytesread   : c_uint,       /* [w] Fill this in before setting result code to tell FMOD how many bytes were read. */
    result      : FMOD_RESULT,  /* [r/w] Result code, FMOD_OK tells the system it is ready to consume the data.  Set this last!  Default value = FMOD_ERR_NOTREADY. */
    userdata    : *c_void,      /* [r] User data pointer. */
}

pub struct FMOD_CREATESOUNDEXINFO
{
    cbsize              : c_int,                        /* [w] Size of this structure.  This is used so the structure can be expanded in the future and still work on older versions of FMOD Ex. */
    length              : c_uint,                       /* [w] Optional. Specify 0 to ignore. Size in bytes of file to load, or sound to create (in this case only if FMOD_OPENUSER is used).  Required if loading from memory.  If 0 is specified, then it will use the size of the file (unless loading from memory then an error will be returned). */
    fileoffset          : c_uint,                       /* [w] Optional. Specify 0 to ignore. Offset from start of the file to start loading from.  This is useful for loading files from inside big data files. */
    numchannels         : c_int,                        /* [w] Optional. Specify 0 to ignore. Number of channels in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used. */
    defaultfrequency    : c_int,                        /* [w] Optional. Specify 0 to ignore. Default frequency of sound in a sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the frequency determined by the file format. */
    format              : FMOD_SOUND_FORMAT,            /* [w] Optional. Specify 0 or FMOD_SOUND_FORMAT_NONE to ignore. Format of the sound mandatory if FMOD_OPENUSER or FMOD_OPENRAW is used.  Other formats use the format determined by the file format.   */
    decodebuffersize    : c_uint,                       /* [w] Optional. Specify 0 to ignore. For streams.  This determines the size of the double buffer (in PCM samples) that a stream uses.  Use this for user created streams if you want to determine the size of the callback buffer passed to you.  Specify 0 to use FMOD's default size which is currently equivalent to 400ms of the sound format created/loaded. */
    initialsubsound     : c_int,                        /* [w] Optional. Specify 0 to ignore. In a multi-sample file format such as .FSB/.DLS/.SF2, specify the initial subsound to seek to, only if FMOD_CREATESTREAM is used. */
    numsubsounds        : c_int,                        /* [w] Optional. Specify 0 to ignore or have no subsounds.  In a sound created with FMOD_OPENUSER, specify the number of subsounds that are accessable with Sound::getSubSound.  If not created with FMOD_OPENUSER, this will limit the number of subsounds loaded within a multi-subsound file.  If using FSB, then if FMOD_CREATESOUNDEXINFO::inclusionlist is used, this will shuffle subsounds down so that there are not any gaps.  It will mean that the indices of the sounds will be different. */
    inclusionlist       : *c_int,                       /* [w] Optional. Specify 0 to ignore. In a multi-sample format such as .FSB/.DLS/.SF2 it may be desirable to specify only a subset of sounds to be loaded out of the whole file.  This is an array of subsound indices to load into memory when created. */
    inclusionlistnum    : c_int,                        /* [w] Optional. Specify 0 to ignore. This is the number of integers contained within the inclusionlist array. */
    pcmreadcallback     : FMOD_SOUND_PCMREADCALLBACK,   /* [w] Optional. Specify 0 to ignore. Callback to 'piggyback' on FMOD's read functions and accept or even write PCM data while FMOD is opening the sound.  Used for user sounds created with FMOD_OPENUSER or for capturing decoded data as FMOD reads it. */
    pcmsetposcallback   : FMOD_SOUND_PCMSETPOSCALLBACK, /* [w] Optional. Specify 0 to ignore. Callback for when the user calls a seeking function such as Channel::setTime or Channel::setPosition within a multi-sample sound, and for when it is opened.*/
    nonblockcallback    : FMOD_SOUND_NONBLOCKCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for successful completion, or error while loading a sound that used the FMOD_NONBLOCKING flag.  Also called duing seeking, when setPosition is called or a stream is restarted. */
    dlsname             : *c_char,                      /* [w] Optional. Specify 0 to ignore. Filename for a DLS or SF2 sample set when loading a MIDI file. If not specified, on Windows it will attempt to open /windows/system32/drivers/gm.dls or /windows/system32/drivers/etc/gm.dls, on Mac it will attempt to load /System/Library/Components/CoreAudio.component/Contents/Resources/gs_instruments.dls, otherwise the MIDI will fail to open. Current DLS support is for level 1 of the specification. */
    encryptionkey       : *c_char,                      /* [w] Optional. Specify 0 to ignore. Key for encrypted FSB file.  Without this key an encrypted FSB file will not load. */
    maxpolyphony        : c_int,                        /* [w] Optional. Specify 0 to ignore. For sequenced formats with dynamic channel allocation such as .MID and .IT, this specifies the maximum voice count allowed while playing.  .IT defaults to 64.  .MID defaults to 32. */
    userdata            : *c_void,                      /* [w] Optional. Specify 0 to ignore. This is user data to be attached to the sound during creation.  Access via Sound::getUserData.  Note: This is not passed to FMOD_FILE_OPENCALLBACK, that is a different userdata that is file specific. */
    suggestedsoundtype  : FMOD_SOUND_TYPE,              /* [w] Optional. Specify 0 or FMOD_SOUND_TYPE_UNKNOWN to ignore.  Instead of scanning all codec types, use this to speed up loading by making it jump straight to this codec. */
    useropen            : FMOD_FILE_OPENCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for opening this file. */
    userclose           : FMOD_FILE_CLOSECALLBACK,      /* [w] Optional. Specify 0 to ignore. Callback for closing this file. */
    userread            : FMOD_FILE_READCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for reading from this file. */
    userseek            : FMOD_FILE_SEEKCALLBACK,       /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    userasyncread       : FMOD_FILE_ASYNCREADCALLBACK,  /* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    userasynccancel     : FMOD_FILE_ASYNCCANCELCALLBACK,/* [w] Optional. Specify 0 to ignore. Callback for seeking within this file. */
    speakermap          : FMOD_SPEAKERMAPTYPE,          /* [w] Optional. Specify 0 to ignore. Use this to differ the way fmod maps multichannel sounds to speakers.  See FMOD_SPEAKERMAPTYPE for more. */
    initialsoundgroup   : *FMOD_SOUNDGROUP,             /* [w] Optional. Specify 0 to ignore. Specify a sound group if required, to put sound in as it is created. */
    initialseekposition : c_uint,                       /* [w] Optional. Specify 0 to ignore. For streams. Specify an initial position to seek the stream to. */
    initialseekpostype  : FMOD_TIMEUNIT,                /* [w] Optional. Specify 0 to ignore. For streams. Specify the time unit for the position set in initialseekposition. */
    ignoresetfilesystem : c_int,                        /* [w] Optional. Specify 0 to ignore. Set to 1 to use fmod's built in file system. Ignores setFileSystem callbacks and also FMOD_CREATESOUNEXINFO file callbacks.  Useful for specific cases where you don't want to use your own file system but want to use fmod's file system (ie net streaming). */
    cddaforceaspi       : c_int,                        /* [w] Optional. Specify 0 to ignore. For CDDA sounds only - if non-zero use ASPI instead of NTSCSI to access the specified CD/DVD device. */
    audioqueuepolicy    : c_uint,                       /* [w] Optional. Specify 0 or FMOD_AUDIOQUEUE_CODECPOLICY_DEFAULT to ignore. Policy used to determine whether hardware or software is used for decoding, see FMOD_AUDIOQUEUE_CODECPOLICY for options (iOS >= 3.0 required, otherwise only hardware is available) */ 
    minmidigranularity  : c_uint,                       /* [w] Optional. Specify 0 to ignore. Allows you to set a minimum desired MIDI mixer granularity. Values smaller than 512 give greater than default accuracy at the cost of more CPU and vice versa. Specify 0 for default (512 samples). */
    nonblockthreadid    : c_int,                        /* [w] Optional. Specify 0 to ignore. Specifies a thread index to execute non blocking load on.  Allows for up to 5 threads to be used for loading at once.  This is to avoid one load blocking another.  Maximum value = 4. */
}

pub struct FMOD_REVERB_CHANNELPROPERTIES
{                                           /*       MIN    MAX  DEFAULT  DESCRIPTION */
    pub Direct          : c_int,            /* [r/w] -10000 1000 0        Direct path level                                        (SUPPORTED:SFX) */
    pub Room            : c_int,            /* [r/w] -10000 1000 0        Room effect level                                        (SUPPORTED:SFX) */
    pub Flags           : c_uint,           /* [r/w] FMOD_REVERB_CHANNELFLAGS - modifies the behavior of properties                (SUPPORTED:SFX) */
    pub ConnectionPoint : *FMOD_DSP         /* [r/w] See remarks.         DSP network location to connect reverb for this channel. (SUPPORTED:SFX).*/
}

pub struct FMOD_GUID
{
    pub Data1: c_uint,              /* Specifies the first 8 hexadecimal digits of the GUID */
    pub Data2: c_ushort,            /* Specifies the first group of 4 hexadecimal digits.   */
    pub Data3: c_ushort,            /* Specifies the second group of 4 hexadecimal digits.  */
    pub Data4: [c_uchar, ..8]       /* Array of 8 bytes. The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits. */
}

pub struct FMOD_ADVANCEDSETTINGS
{
    pub cbsize                      : c_int,        /* [w]   Size of this structure.  Use sizeof(FMOD_ADVANCEDSETTINGS)  NOTE: This must be set before calling System::getAdvancedSettings! */
    pub maxMPEGcodecs               : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  Mpeg  codecs consume 21,684 bytes per instance and this number will determine how many mpeg channels can be played simultaneously.   Default = 32. */
    pub maxADPCMcodecs              : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  ADPCM codecs consume  2,136 bytes per instance and this number will determine how many ADPCM channels can be played simultaneously.  Default = 32. */
    pub maxXMAcodecs                : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  XMA   codecs consume 14,836 bytes per instance and this number will determine how many XMA channels can be played simultaneously.    Default = 32. */
    pub maxCELTcodecs               : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  CELT  codecs consume 11,500 bytes per instance and this number will determine how many CELT channels can be played simultaneously.   Default = 32. */    
    pub maxVORBIScodecs             : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  Vorbis codecs consume 12,000 bytes per instance and this number will determine how many Vorbis channels can be played simultaneously. Default = 32. */    
    pub maxAT9Codecs                : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_CREATECOMPRESSEDSAMPLE only.  AT9 codecs consume  8,720 bytes per instance and this number will determine how many AT9 channels can be played simultaneously. Default = 32. */    
    pub maxPCMcodecs                : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with PS3 only.                          PCM   codecs consume 12,672 bytes per instance and this number will determine how many streams and PCM voices can be played simultaneously. Default = 16. */
    pub ASIONumChannels             : c_int,        /* [r/w] Optional. Specify 0 to ignore. Number of channels available on the ASIO device. */
    pub ASIOChannelList             : **c_char,     /* [r/w] Optional. Specify 0 to ignore. Pointer to an array of strings (number of entries defined by ASIONumChannels) with ASIO channel names. */
    pub ASIOSpeakerList             : *FMOD_SPEAKER,/* [r/w] Optional. Specify 0 to ignore. Pointer to a list of speakers that the ASIO channels map to.  This can be called after System::init to remap ASIO output. */
    pub max3DReverbDSPs             : c_int,        /* [r/w] Optional. Specify 0 to ignore. The max number of 3d reverb DSP's in the system. (NOTE: CURRENTLY DISABLED / UNUSED) */
    pub HRTFMinAngle                : c_float,      /* [r/w] Optional.                      For use with FMOD_INIT_HRTF_LOWPASS.  The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function begins to have an effect. 0 = in front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 180.0. */
    pub HRTFMaxAngle                : c_float,      /* [r/w] Optional.                      For use with FMOD_INIT_HRTF_LOWPASS.  The angle range (0-360) of a 3D sound in relation to the listener, at which the HRTF function has maximum effect. 0 = front of the listener. 180 = from 90 degrees to the left of the listener to 90 degrees to the right. 360 = behind the listener. Default = 360.0. */
    pub HRTFFreq                    : c_float,      /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_HRTF_LOWPASS.  The cutoff frequency of the HRTF's lowpass filter function when at maximum effect. (i.e. at HRTFMaxAngle).  Default = 4000.0. */
    pub vol0virtualvol              : c_float,      /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_VOL0_BECOMES_VIRTUAL.  If this flag is used, and the volume is 0.0, then the sound will become virtual.  Use this value to raise the threshold to a different point where a sound goes virtual. */
    pub eventqueuesize              : c_int,        /* [r/w] Optional. Specify 0 to ignore. For use with FMOD Event system only.  Specifies the number of slots available for simultaneous non blocking loads, across all threads.  Default = 32. */
    pub defaultDecodeBufferSize     : c_uint,       /* [r/w] Optional. Specify 0 to ignore. For streams. This determines the default size of the double buffer (in milliseconds) that a stream uses.  Default = 400ms */
    pub debugLogFilename            : *c_char,      /* [r/w] Optional. Specify 0 to ignore. Gives fmod's logging system a path/filename.  Normally the log is placed in the same directory as the executable and called fmod.log. When using System::getAdvancedSettings, provide at least 256 bytes of memory to copy into. */
    pub profileport                 : c_ushort,     /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_ENABLE_PROFILE.  Specify the port to listen on for connections by the profiler application. */
    pub geometryMaxFadeTime         : c_uint,       /* [r/w] Optional. Specify 0 to ignore. The maximum time in miliseconds it takes for a channel to fade to the new level when its occlusion changes. */
    pub maxSpectrumWaveDataBuffers  : c_uint,       /* [r/w] Optional. Specify 0 to ignore. Tells System::init to allocate a pool of wavedata/spectrum buffers to prevent memory fragmentation, any additional buffers will be allocated normally. */
    pub musicSystemCacheDelay       : c_uint,       /* [r/w] Optional. Specify 0 to ignore. The delay the music system should allow for loading a sample from disk (in milliseconds). Default = 400 ms. */
    pub distanceFilterCenterFreq    : c_float,      /* [r/w] Optional. Specify 0 to ignore. For use with FMOD_INIT_DISTANCE_FILTERING.  The default center frequency in Hz for the distance filtering effect. Default = 1500.0. */
    pub stackSizeStream             : c_uint,       /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD Stream thread in bytes.  Useful for custom codecs that use excess stack.  Default 49,152 (48kb) */
    pub stackSizeNonBlocking        : c_uint,       /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD_NONBLOCKING loading thread.  Useful for custom codecs that use excess stack.  Default 65,536 (64kb) */
    pub stackSizeMixer              : c_uint        /* [r/w] Optional. Specify 0 to ignore. Specify the stack size for the FMOD mixer thread.  Useful for custom dsps that use excess stack.  Default 49,152 (48kb) */
}