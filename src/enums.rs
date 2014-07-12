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
use types::{FmodTimeUnit, FmodMemoryBits};

pub mod fmod {
    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// error codes. Returned from every function.
    pub enum Result
    {
        /// No errors.
        Ok,
        /// Tried to call lock a second time before unlock was called.
        Err_AlreadyLocked,
        /// Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound::lock on a streaming sound).
        ErrBadCommand,
        /// Neither NTSCSI nor ASPI could be initialised.
        ErrCDDADrivers,
        /// An error occurred while initialising the CDDA subsystem.
        ErrCDDAInit,
        /// Couldn't find the specified device.
        ErrCDDAInvalidDevice,
        /// No audio tracks on the specified disc.
        ErrCDDANoAudio,
        /// No CD/DVD devices were found.
        ErrCDDANoDevices,
        /// No disc present in the specified drive.
        ErrCDDANoDisc,
        /// A CDDA read error occurred.
        ErrCDDARead,
        /// Error trying to allocate a channel.
        ErrChannelAlloc,
        /// The specified channel has been reused to play another sound.
        ErrChannelStolen,
        /// A Win32 COM related error occured. COM failed to initialize or a QueryInterface failed meaning a Windows codec or driver was not installed properly.
        ErrCOM,
        /// DMA Failure.  See debug output for more information.
        ErrDMA,
        /// DSP connection error.  Connection possibly caused a cyclic dependancy.  Or tried to connect a tree too many units deep (more than 128).
        ErrDSPConnection,
        /// DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format.
        ErrDSPFormat,
        /// DSP connection error.  Couldn't find the DSP unit specified.
        ErrDSPNotFound,
        /// DSP error.  Cannot perform this operation while the network is in the middle of running.  This will most likely happen if a connection or disconnection is attempted in a DSP callback.
        ErrDSPRunning,
        /// DSP connection error.  The unit being connected to or disconnected should only have 1 input or output.
        ErrDSPTooManyConnections,
        /// Error loading file.
        ErrFileBad,
        /// Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format.
        ErrFileCouldNotSeek,
        /// Media was ejected while reading.
        ErrFileDiskEjected,
        /// End of file unexpectedly reached while trying to read essential data (truncated data ?).
        ErrFileEOF,
        /// File not found.
        ErrFileNotFound,
        /// Unwanted file access occured.
        ErrFileUnwanted,
        /// Unsupported file or audio format.
        ErrFormat,
        /// A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.
        ErrHTTP,
        /// The specified resource requires authentication or is forbidden.
        ErrHTTPAccess,
        /// Proxy authentication is required to access the specified resource.
        ErrHTTPProxyAuth,
        /// A HTTP server error occurred.
        ErrHTTPServerError,
        /// The HTTP request timed out.
        ErrHTTPTimeout,
        /// FMOD was not initialized correctly to support this function.
        ErrInitialization,
        /// Cannot call this command after System::init.
        ErrInitialized,
        /// An error occured that wasn't supposed to.  Contact support.
        ErrInternal,
        /// On Xbox 360, this memory address passed to FMOD must be physical, (ie allocated with XPhysicalAlloc.)
        ErrInvalidAddress,
        /// Value passed in was a NaN, Inf or denormalized float.
        ErrInvalidFloat,
        /// An invalid object handle was used.
        ErrInvalidHandle,
        /// An invalid parameter was passed to this function.
        ErrInvalidParam,
        /// An invalid seek position was passed to this function.
        ErrInvalidPosition,
        /// An invalid speaker was passed to this function based on the current speaker mode.
        ErrInvalidSpeaker,
        /// The syncpoint did not come from this sound handle.
        ErrInvalidSyncPoint,
        /// The vectors passed in are not unit length, or perpendicular.
        ErrInvalidVector,
        /// Reached maximum audible playback count for this sound's soundgroup.
        ErrMaxAudible,
        /// Not enough memory or resources.
        ErrMemory,
        /// Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used.
        ErrMemoryCantPoint,
        /// Not enough memory or resources on console sound ram.
        ErrMemorySRam,
        /// Tried to call a command on a 3d sound when the command was meant for 2d sound.
        ErrNeeds2D,
        /// Tried to call a command on a 2d sound when the command was meant for 3d sound.
        ErrNeeds3D,
        /// Tried to use a feature that requires hardware support.  (ie trying to play a GCADPCM compressed sound in software on Wii).
        ErrNeedsHardware,
        /// Tried to use a feature that requires the software engine.  Software engine has either been turned off, or command was executed on a hardware channel which does not support this feature.
        ErrNeedsSoftware,
        /// Couldn't connect to the specified host.
        ErrNetConnect,
        /// A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere.
        ErrNetSocketError,
        /// The specified URL couldn't be resolved.
        ErrNetURL,
        /// Operation on a non-blocking socket could not complete immediately.
        ErrNetWouldBlock,
        /// Operation could not be performed because specified sound/DSP connection is not ready.
        ErrNotReady,
        /// Error initializing output device, but more specifically, the output device is already in use and cannot be reused.
        ErrOutputAllocated,
        /// Error creating hardware sound buffer.
        ErrOutputCreateBuffer,
        /// A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted.
        ErrOutputDriverCall,
        /// Error enumerating the available driver list. List may be inconsistent due to a recent device addition or removal.
        ErrOutputEnumeration,
        /// Soundcard does not support the minimum features needed for this soundsystem (16bit stereo output).
        ErrOutputFormat,
        /// Error initializing output device.
        ErrOutputInit,
        /// FMOD_HARDWARE was specified but the sound card does not have the resources necessary to play it.
        ErrOutputNoHardware,
        /// Attempted to create a software sound but no software channels were specified in System::init.
        ErrOutputNoSoftware,
        /// Panning only works with mono or stereo sound sources.
        ErrPan,
        /// An unspecified error has been returned from a 3rd party plugin.
        ErrPlugin,
        /// The number of allowed instances of a plugin has been exceeded.
        ErrPluginInstances,
        /// A requested output, dsp unit type or codec was not available.
        ErrPluginMissing,
        /// A resource that the plugin requires cannot be found. (ie the DLS file for MIDI playback or other DLLs that it needs to load)
        ErrPluginResource,
        /// The specified sound is still in use by the event system, call EventSystem::unloadFSB before trying to release it.
        ErrPreloaded,
        /// The specified sound is still in use by the event system, wait for the event which is using it finish with it.
        ErrProgrammerSound,
        /// An error occured trying to initialize the recording device.
        ErrRecord,
        /// Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesnt exist.
        ErrReverbInstance,
        /// This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first.
        ErrSubsoundAllocated,
        /// Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file.
        ErrSubsoundCantMove,
        /// The subsound's mode bits do not match with the parent sound's mode bits.  See documentation for function that it was called with.
        ErrSubsoundMode,
        /// The error occured because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound, or a parent sound was played without setting up a sentence first.
        ErrSubsounds,
        /// The specified tag could not be found or there are no tags.
        ErrTagNotFound,
        /// The sound created exceeds the allowable input channel count.  This can be increased using the maxinputchannels parameter in System::setSoftwareFormat.
        ErrTooManyChannels,
        /// Something in FMOD hasn't been implemented when it should be ! contact support !
        ErrUnimplemented,
        /// This command failed because System::init or System::setDriver was not called.
        ErrUnintialized,
        /// A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified.
        ErrUnsupported,
        /// An error caused by System::update occured.
        ErrUpdate,
        /// The version number of this file format is not supported.
        ErrVersion,

        /// An Event failed to be retrieved, most likely due to 'just fail' being specified as the max playbacks behavior.
        ErrEventFailed,
        /// Can't execute this command on an EVENT_INFOONLY event.
        ErrEventINFOONLY,
        /// An error occured that wasn't supposed to.  See debug log for reason.
        ErrEventInternal,
        /// Event failed because 'Max streams' was hit when FMOD_EVENT_INIT_FAIL_ON_MAXSTREAMS was specified.
        ErrEventMaxStreams,
        /// FSB mismatches the FEV it was compiled with, the stream/sample mode it was meant to be created with was different, or the FEV was built for a different platform.
        ErrEventMismatch,
        /// A category with the same name already exists.
        ErrEventNameConflict,
        /// The requested event, event group, event category or event property could not be found.
        ErrEventNotFound,
        /// Tried to call a function on a complex event that's only supported by simple events.
        ErrEventNeedSimple,
        /// An event with the same GUID already exists.
        ErrEventGuidConflict,
        /// The specified project or bank has already been loaded. Having multiple copies of the same project loaded simultaneously is forbidden.
        ErrEventAlreadyLoaded,

        /// Music system is not initialized probably because no music data is loaded.
        ErrMusicUnintialized,
        /// The requested music entity could not be found.
        ErrMusicNotFound,
        /// The music callback is required, but it has not been set.
        ErrMusicNoCallback,

        /// Makes sure this enum is signed 32bit.
        ResultForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// When creating a multichannel sound, FMOD will pan them to their default speaker locations:
    /// * For example a 6 channel sound will default to one channel per 5.1 output speaker.
    /// * Another example is a stereo sound.  It will default to left = front left, right = front right.
    /// * This is for sounds that are not 'default'.  For example you might have a sound that is 6 channels but actually made up of 3 stereo pairs, that should all be located in front left, front right only.
    pub enum SpeakerMapType
    {
        /// This is the default, and just means FMOD decides which speakers it puts the source channels.
        SpeakerMapTypeDefault,
        /// This means the sound is made up of all mono sounds.  All voices will be panned to the front center by default in this case.
        SpeakerMapTypeAllMono,
        /// This means the sound is made up of all stereo sounds.  All voices will be panned to front left and front right alternating every second channel.
        SpeakerMapTypeAllStereo,
        /// Map a 5.1 sound to use protools L C R Ls Rs LFE mapping.  Will return an error if not a 6 channel sound.
        SpeakerMapType51ProTools
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// These definitions describe the native format of the hardware or software buffer that will be used.
    pub enum SoundFormat
    {
        /// Uninitialized / unknown.
        SoundFormatNone,
        /// 8bit integer PCM data.
        SoundFormatPCM8,
        /// 16bit integer PCM data.
        SoundFormatPCM16,
        /// 24bit integer PCM data.
        SoundFormatPCM24,
        /// 32bit integer PCM data.
        SoundFormatPCM32,
        /// 32bit floating point PCM data.
        SoundFormatPCMFloat,
        /// Compressed Nintendo 3DS/Wii DSP data.
        SoundFormatGCADPCM,
        /// Compressed IMA ADPCM data.
        SoundFormatIMAADPCM,
        /// Compressed PlayStation Portable ADPCM data.
        SoundFormatVAG,
        /// Compressed PSVita ADPCM data.
        SoundFormatHEVAG,
        /// Compressed Xbox360 XMA data.
        SoundFormatXMA,
        /// Compressed MPEG layer 2 or 3 data.
        SoundFormatMPEG,
        /// Compressed CELT data.
        SoundFormatCELT,
        /// Compressed PSVita ATRAC9 data.
        SoundFormatAT9,
        /// Compressed Xbox360 xWMA data.
        SoundFormatXWMA,
        /// Compressed Vorbis data.
        SoundFormatVORBIS,

        /// Maximum number of sound formats supported.
        SoundFormatMax,
        /// Makes sure this enum is signed 32bit.
        SoundFormatForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// These definitions describe the type of song being played.
    pub enum SoundType
    {
        /// 3rd party / unknown plugin format.
        SoundTypeUnknown,
        /// AIFF.
        SoundTypeAIFF,
        /// Microsoft Advanced Systems Format (ie WMA/ASF/WMV).
        SoundTypeASF,
        /// Sony ATRAC 3 format
        SoundTypeAT3,
        /// Digital CD audio.
        SoundTypeCDDA,
        /// Sound font / downloadable sound bank.
        SoundTypeDLS,
        /// FLAC lossless codec.
        SoundTypeFLAC,
        /// FMOD Sample Bank.
        SoundTypeFSB,
        /// Nintendo GameCube/Wii ADPCM
        SoundTypeGCADPCM,
        /// Impulse Tracker.
        SoundTypeIT,
        /// MIDI. extracodecdata is a pointer to an FMOD_MIDI_EXTRACODECDATA structure.
        SoundTypeMIDI,
        /// Protracker / Fasttracker MOD.
        SoundTypeMOD,
        /// MP2/MP3 MPEG.
        SoundTypeMPEG,
        /// Ogg vorbis.
        SoundTypeOGGVORBIS,
        /// Information only from ASX/PLS/M3U/WAX playlists
        SoundTypePlaylist,
        /// Raw PCM data.
        SoundTypeRaw,
        /// ScreamTracker 3.
        SoundTypeS3M,
        /// Sound font 2 format.
        SoundTypeSF2,
        /// User created sound.
        SoundTypeUser,
        /// Microsoft WAV.
        SoundTypeWAV,
        /// FastTracker 2 XM.
        SoundTypeXM,
        /// Xbox360 XMA
        SoundTypeXMA,
        /// PlayStation Portable ADPCM VAG format.
        SoundTypeVAG,
        /// iPhone hardware decoder, supports AAC, ALAC and MP3. extracodecdata is a pointer to an FMOD_AUDIOQUEUE_EXTRACODECDATA structure.
        SoundTypeAudioQueue,
        /// Xbox360 XWMA
        SoundTypeXWMA,
        /// 3DS BCWAV container format for DSP ADPCM and PCM
        SoundTypeBCWAV,
        /// NGP ATRAC 9 format
        SoundTypeAT9,
        /// Raw vorbis
        SoundTypeVORBIS,
        /// Microsoft Media Foundation wrappers, supports ASF/WMA
        SoundTypeMediaFoundation,

        /// Maximum number of sound types supported.
        SoundTypeMax,
        /// Makes sure this enum is signed 32bit.
        SoundTypeForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// List of tag types that could be stored within a sound.  These include id3 tags, metadata from netstreams and vorbis/asf data.
    pub enum TagType
    {
        TagTypeUnknown = 0,
        TagTypeID3V1,
        TagTypeID3V2,
        TagTypeVORBISComment,
        TagTypeShoutCast,
        TagTypeIceCast,
        TagTypeASF,
        TagTypeMIDI,
        TagTypePlaylist,
        TagTypeFmod,
        TagTypeUser,

        /// Maximum number of tag types supported.
        TagTypeMax,
        /// Makes sure this enum is signed 32bit.
        TagTypeForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// List of data types that can be returned by Sound::getTag
    pub enum TagDataType
    {
        TagDataTypeBinary = 0,
        TagDataTypeInt,
        TagDataTypeFloat,
        TagDataTypeString,
        TagDataTypeStringUTF16,
        TagDataTypeStringUTF16BE,
        TagDataTypeStringUTF8,
        TagDataTypeCDTOC,

        /// Maximum number of tag datatypes supported.
        TagDataTypeMax,
        /// Makes sure this enum is signed 32bit.
        TagDataTypeForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// Special channel index values for FMOD functions.
    pub enum ChannelIndex
    {
        /// For a channel index, FMOD chooses a free voice using the priority system.
        ChannelFree  = -1,
        /// For a channel index, re-use the channel handle that was passed in.
        ChannelReUse = -2,
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// * List of windowing methods used in spectrum analysis to reduce leakage / transient signals intefering with the analysis.
    /// * This is a problem with analysis of continuous signals that only have a small portion of the signal sample (the fft window size).
    /// * Windowing the signal with a curve or triangle tapers the sides of the fft window to help alleviate this problem.
    pub enum DSP_FFT_Window
    {
        /// w[n] = 1.0
        DSP_FFT_WindowRect,
        /// w[n] = TRI(2n/N)
        DSP_FFT_WindowTriangle,
        /// w[n] = 0.54 - (0.46 * COS(n/N) )
        DSP_FFT_WindowHamming,
        /// w[n] = 0.5 *  (1.0  - COS(n/N) )
        DSP_FFT_WindowHanning,
        /// w[n] = 0.42 - (0.5  * COS(n/N) ) + (0.08 * COS(2.0 * n/N) )
        DSP_FFT_WindowBlackMan,
        /// w[n] = 0.35875 - (0.48829 * COS(1.0 * n/N)) + (0.14128 * COS(2.0 * n/N)) - (0.01168 * COS(3.0 * n/N))
        DSP_FFT_WindowBlackManHarris,

        /// Maximum number of FFT window types supported.
        DSP_FFT_WindowMax,
        /// Makes sure this enum is signed 32bit.
        DSP_FFT_WindowForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    /// Types of delay that can be used with [`Channel::set_delay`](/struct.Channel.html#method.set_delay) / [`Channel::get_delay`](struct.Channel.html#method.get_delay).
    pub enum DelayType
    {
        /// Delay at the end of the sound in milliseconds.  Use delayhi only. [`Channel::is_playing`](struct.Channel.html#method.is_playing) will remain true until this delay has passed even though the sound itself has stopped playing.
        DelayTypeEndMS,
        /// Time the sound started if [`Channel::get_delay`](struct.Channel.html#method.get_delay) is used, or if [`Channel::set_delay`](struct.Channel.html#method.set_delay) is used, the sound will delay playing until this exact tick.
        DelayTypeDSPClockStart,
        /// Time the sound should end. If this is non-zero, the channel will go silent at this exact tick.
        DelayTypeDSPClockEnd,
        /// Time the sound should pause. If this is non-zero, the channel will pause at this exact tick.
        DelayTypeDSPClockPause,

        /// Maximum number of tag datatypes supported.
        DelayTypeMax,
        /// Makes sure this enum is signed 32bit.
        DelayTypeForceInt = 65536
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum OutputType
    {
        OutputTypeAutoDetect,      /* Picks the best output mode for the platform.  This is the default. */
                                         
        OutputTypeUnknown,         /* All             - 3rd party plugin, unknown.  This is for use with System::getOutput only. */
        OutputTypeNoSound,         /* All             - All calls in this mode succeed but make no sound. */
        OutputTypeWAVWriter,       /* All             - Writes output to fmodoutput.wav by default.  Use the 'extradriverdata' parameter in System::init, by simply passing the filename as a string, to set the wav filename. */
        OutputTypeNoSoundNRT,      /* All             - Non-realtime version of FMOD_OUTPUTTYPE_NOSOUND.  User can drive mixer with System::update at whatever rate they want. */
        OutputTypeWAVWriterNRT,    /* All             - Non-realtime version of FMOD_OUTPUTTYPE_WAVWRITER.  User can drive mixer with System::update at whatever rate they want. */
                                         
        OutputTypeDSound,          /* Win32/Win64     - DirectSound output.                       (Default on Windows XP and below) */
        OutputTypeWinMM,           /* Win32/Win64     - Windows Multimedia output. */
        OutputTypeWASAPI,          /* Win32           - Windows Audio Session API.                (Default on Windows Vista and above) */
        OutputTypeASIO,            /* Win32           - Low latency ASIO 2.0 driver. */
        OutputTypeOSS,             /* Linux/Linux64   - Open Sound System output.                 (Default on Linux, third preference) */
        OutputTypeALSA,            /* Linux/Linux64   - Advanced Linux Sound Architecture output. (Default on Linux, second preference if available) */
        OutputTypeESD,             /* Linux/Linux64   - Enlightment Sound Daemon output. */
        OutputTypePulseAudio,      /* Linux/Linux64   - PulseAudio output.                        (Default on Linux, first preference if available) */
        OutputTypeCoreAudio,       /* Mac             - Macintosh CoreAudio output.               (Default on Mac) */
        OutputTypeXbox360,         /* Xbox 360        - Native Xbox360 output.                    (Default on Xbox 360) */
        OutputTypePSP,             /* PSP             - Native PSP output.                        (Default on PSP) */
        OutputTypePS3,             /* PS3             - Native PS3 output.                        (Default on PS3) */
        OutputTypeNGP,             /* NGP             - Native NGP output.                        (Default on NGP) */
        OutputTypeWii,             /* Wii             - Native Wii output.                        (Default on Wii) */
        OutputType3DS,             /* 3DS             - Native 3DS output                         (Default on 3DS) */
        OutputTypeAudioTrack,      /* Android         - Java Audio Track output.                  (Default on Android 2.2 and below) */
        OutputTypeOpenSL,          /* Android         - OpenSL ES output.                         (Default on Android 2.3 and above) */   
        OutputTypeNACL,            /* Native Client   - Native Client output.                     (Default on Native Client) */
        OutputTypeWiiU,            /* Wii U           - Native Wii U output.                      (Default on Wii U) */
        OutputTypeASound,          /* BlackBerry      - Native BlackBerry asound output.          (Default on BlackBerry) */
        OutputTypeAudioOut,        /* Orbis           - Audio Out output.                         (Default on Orbis) */
        OutputTypeXAudio,          /* Durango         - XAudio2 output. */

        OutputTypeMax,             /* Maximum number of output types supported. */
        OutputTypeForceInt = 65536 /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialOrd, Show, PartialEq)]
    #[repr(C)]
    //FIXME
    pub enum Speaker
    {
        SpeakerFrontLeft,
        SpeakerFrontRight,
        SpeakerFrontCenter,
        SpeakerLowFrequency,
        SpeakerBackLeft,
        SpeakerBackRight,
        SpeakerSideLeft,
        SpeakerSideRight,
        
        SpeakerMax,                 /* Maximum number of speaker types supported. */
        //SpeakerMono        = 0,     /* For use with fmod::SpeakerMODE_MONO and Channel::SetSpeakerLevels.  Mapped to same value as fmod::Speaker_FRONT_LEFT. */
        SpeakerNull        = 65535, /* A non speaker.  Use this with ASIO mapping to ignore a speaker. */
        //SpeakerSBL         = 6,     /* For use with fmod::SpeakerMODE_7POINT1 on PS3 where the extra speakers are surround back inside of side speakers. */
        //SpeakerSBR         = 7,     /* For use with fmod::SpeakerMODE_7POINT1 on PS3 where the extra speakers are surround back inside of side speakers. */
        SpeakerForceInt    = 65536  /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum SpeakerMode
    {
        SpeakerModeRaw,              /* There is no specific speakermode.  Sound channels are mapped in order of input to output.  Use System::setSoftwareFormat to specify speaker count. See remarks for more information. */
        SpeakerModeMono,             /* The speakers are monaural. */
        SpeakerModeStereo,           /* The speakers are stereo (DEFAULT). */
        SpeakerModeQuad,             /* 4 speaker setup.  This includes front left, front right, rear left, rear right.  */
        SpeakerModeSurround,         /* 5 speaker setup.  This includes front left, front right, center, rear left, rear right. */
        SpeakerMode5Point1,          /* 5.1 speaker setup.  This includes front left, front right, center, rear left, rear right and a subwoofer. */
        SpeakerMode7Point1,          /* 7.1 speaker setup.  This includes front left, front right, center, rear left, rear right, side left, side right and a subwoofer. */
        
        SpeakerModeSRS5_1_Matrix,    /* Stereo compatible output, embedded with surround information. SRS 5.1/Prologic/Prologic2 decoders will split the signal into a 5.1 speaker set-up or SRS virtual surround will decode into a 2-speaker/headphone setup.  See remarks about limitations.*/
        SpeakerModeDOLBY5_1_Matrix,  /* Stereo compatible output, embedded with surround information. Dolby Pro Logic II decoders will split the signal into a 5.1 speaker set-up. */
        SpeakerModeMYears,           /* Stereo output, but data is encoded using personalized HRTF algorithms.  See myears.net.au */

        SpeakerModeMax,              /* Maximum number of speaker modes supported. */
        SpeakerModeForceInt = 65536  /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum DSPResampler
    {
        DSPResamplerNoInterp,        /* No interpolation.  High frequency aliasing hiss will be audible depending on the sample rate of the sound. */
        DSPResamplerLinear,          /* Linear interpolation (default method).  Fast and good quality, causes very slight lowpass effect on low frequency sounds. */
        DSPResamplerCubic,           /* Cubic interpolation.  Slower than linear interpolation but better quality. */
        DSPResamplerSpline,          /* 5 point spline interpolation.  Slowest resampling method but best quality. */

        DSPResamplerMax,             /* Maximum number of resample methods supported. */
        DSPResamplerForceInt = 65536 /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum PluginType
    {
        PluginTypeOutput,          /* The plugin type is an output module.  FMOD mixed audio will play through one of these devices */
        PluginTypeCodec,           /* The plugin type is a file format codec.  FMOD will use these codecs to load file formats for playback. */
        PluginTypeDSP,             /* The plugin type is a DSP unit.  FMOD will use these plugins as part of its DSP network to apply effects to output or generate sound in realtime. */

        PluginTypeMax,             /* Maximum number of plugin types supported. */
        PluginTypeForceInt = 65536 /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum OpenState
    {
        OpenStateReady = 0,       /* Opened and ready to play. */
        OpenStateLoading,         /* Initial load in progress. */
        OpenStateError,           /* Failed to open - file not found, out of memory etc.  See return value of Sound::getOpenState for what happened. */
        OpenStateConnecting,      /* Connecting to remote host (internet sounds only). */
        OpenStateBuffering,       /* Buffering data. */
        OpenStateSeeking,         /* Seeking to subsound and re-flushing stream buffer. */
        OpenStatePlaying,         /* Ready and playing, but not possible to release at this time without stalling the main thread. */
        OpenStateSetPosition,     /* Seeking within a stream to a different position. */

        OpenStateMax,             /* Maximum number of open state types. */
        OpenStateForceInt = 65536 /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum SystemCallbackType
    {
        SystemCallbackTypeDeviceListChanged,         /* Called from System::update when the enumerated list of devices has changed. */
        SystemCallbackTypeDeviceLost,                /* Called from System::update when an output device has been lost due to control panel parameter changes and FMOD cannot automatically recover. */
        SystemCallbackTypeMemoryAllocationFailed,    /* Called directly when a memory allocation fails somewhere in FMOD.  (NOTE - 'system' will be NULL in this callback type.)*/
        SystemCallbackTypeThreadCreated,             /* Called directly when a thread is created. */
        SystemCallbackTypeBadDSPConnection,          /* Called when a bad connection was made with DSP::addInput. Usually called from mixer thread because that is where the connections are made.  */
        SystemCallbackTypeBadDSPLevel,               /* Called when too many effects were added exceeding the maximum tree depth of 128.  This is most likely caused by accidentally adding too many DSP effects. Usually called from mixer thread because that is where the connections are made.  */
        SystemCallbackTypeThreadDestroyed,           /* Called directly when a thread is destroyed. */

        SystemCallbackTypeMax,                       /* Maximum number of callback types supported. */
        SystemCallbackTypeForceInt = 65536           /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum SoundGroupBehavior
    {
        SoundGroupBehaviorFail,              /* Any sound played that puts the sound count over the SoundGroup::setMaxAudible setting, will simply fail during System::playSound. */
        SoundGroupBehaviorMute,              /* Any sound played that puts the sound count over the SoundGroup::setMaxAudible setting, will be silent, then if another sound in the group stops the sound that was silent before becomes audible again. */
        SoundGroupBehaviorStealLowest,       /* Any sound played that puts the sound count over the SoundGroup::setMaxAudible setting, will steal the quietest / least important sound playing in the group. */

        SoundGroupBehaviorMax,               /* Maximum number of open state types. */
        SoundGroupBehaviorForceInt = 65536   /* Makes sure this enum is signed 32bit. */
    }

    #[deriving(PartialEq, PartialOrd, Show)]
    #[repr(C)]
    pub enum DspType
    {
        DspTypeUnknown,                 /* This unit was created via a non FMOD plugin so has an unknown purpose. */
        DspTypeMixer,                   /* This unit does nothing but take inputs and mix them together then feed the result to the soundcard unit. */
        DspTypeOscillator,              /* This unit generates sine/square/saw/triangle or noise tones. */
        DspTypeLowPass,                 /* This unit filters sound using a high quality, resonant lowpass filter algorithm but consumes more CPU time. */
        DspTypeITLowPass,               /* This unit filters sound using a resonant lowpass filter algorithm that is used in Impulse Tracker, but with limited cutoff range (0 to 8060hz). */
        DspTypeHighPass,                /* This unit filters sound using a resonant highpass filter algorithm. */
        DspTypeEcho,                    /* This unit produces an echo on the sound and fades out at the desired rate. */
        DspTypeFlange,                  /* This unit produces a flange effect on the sound. */
        DspTypeDistortion,              /* This unit distorts the sound. */
        DspTypeNormalize,               /* This unit normalizes or amplifies the sound to a certain level. */
        DspTypeParameq,                 /* This unit attenuates or amplifies a selected frequency range. */
        DspTypePitchShift,              /* This unit bends the pitch of a sound without changing the speed of playback. */
        DspTypeChorus,                  /* This unit produces a chorus effect on the sound. */
        DspTypeVSTPlugin,               /* This unit allows the use of Steinberg VST plugins */
        DspTypeWinampPlugin,            /* This unit allows the use of Nullsoft Winamp plugins */
        DspTypeITEcho,                  /* This unit produces an echo on the sound and fades out at the desired rate as is used in Impulse Tracker. */
        DspTypeCompressor,              /* This unit implements dynamic compression (linked multichannel, wideband) */
        DspTypeSFXReverb,               /* This unit implements SFX reverb */
        DspTypeLowPassSimple,           /* This unit filters sound using a simple lowpass with no resonance, but has flexible cutoff and is fast. */
        DspTypeDelay,                   /* This unit produces different delays on individual channels of the sound. */
        DspTypeTremolo,                 /* This unit produces a tremolo / chopper effect on the sound. */
        DspTypeLADSPAPlugin,            /* This unit allows the use of LADSPA standard plugins. */
        DspTypeHighPassSimple,          /* This unit filters sound using a simple highpass with no resonance, but has flexible cutoff and is fast. */
        DspTypeHardware = 1000,         /* Offset that platform specific FMOD_HARDWARE DSPs will start at. */
        DspTypeForceInt = 65536         /* Makes sure this enum is signed 32bit. */
    }
}

pub static FMOD_DEFAULT                : c_uint = 0x00000000;  /* Default for all modes listed below. FMOD_LOOP_OFF, FMOD_2D, FMOD_HARDWARE */
pub static FMOD_LOOP_OFF               : c_uint = 0x00000001;  /* For non looping sounds. (DEFAULT).  Overrides FMOD_LOOP_NORMAL / FMOD_LOOP_BIDI. */
pub static FMOD_LOOP_NORMAL            : c_uint = 0x00000002;  /* For forward looping sounds. */
pub static FMOD_LOOP_BIDI              : c_uint = 0x00000004;  /* For bidirectional looping sounds. (only works on software mixed static sounds). */
pub static FMOD_2D                     : c_uint = 0x00000008;  /* Ignores any 3d processing. (DEFAULT). */
pub static FMOD_3D                     : c_uint = 0x00000010;  /* Makes the sound positionable in 3D.  Overrides FMOD_2D. */
pub static FMOD_HARDWARE               : c_uint = 0x00000020;  /* Attempts to make sounds use hardware acceleration. (DEFAULT).  Note on platforms that don't support FMOD_HARDWARE (only 3DS, PS Vita, PSP, Wii and Wii U support FMOD_HARDWARE), this will be internally treated as FMOD_SOFTWARE. */
pub static FMOD_SOFTWARE               : c_uint = 0x00000040;  /* Makes the sound be mixed by the FMOD CPU based software mixer.  Overrides FMOD_HARDWARE.  Use this for FFT, DSP, compressed sample support, 2D multi-speaker support and other software related features. */
pub static FMOD_CREATESTREAM           : c_uint = 0x00000080;  /* Decompress at runtime, streaming from the source provided (ie from disk).  Overrides FMOD_CREATESAMPLE and FMOD_CREATECOMPRESSEDSAMPLE.  Note a stream can only be played once at a time due to a stream only having 1 stream buffer and file handle.  Open multiple streams to have them play concurrently. */
pub static FMOD_CREATESAMPLE           : c_uint = 0x00000100;  /* Decompress at loadtime, decompressing or decoding whole file into memory as the target sample format (ie PCM).  Fastest for FMOD_SOFTWARE based playback and most flexible.  */
pub static FMOD_CREATECOMPRESSEDSAMPLE : c_uint = 0x00000200;  /* Load MP2/MP3/IMAADPCM/CELT/Vorbis/AT9 or XMA into memory and leave it compressed.  CELT/Vorbis/AT9 encoding only supported in the FSB file format.  During playback the FMOD software mixer will decode it in realtime as a 'compressed sample'.  Can only be used in combination with FMOD_SOFTWARE.  Overrides FMOD_CREATESAMPLE.  If the sound data is not one of the supported formats, it will behave as if it was created with FMOD_CREATESAMPLE and decode the sound into PCM. */
pub static FMOD_OPENUSER               : c_uint = 0x00000400;  /* Opens a user created static sample or stream. Use FMOD_CREATESOUNDEXINFO to specify format and/or read callbacks.  If a user created 'sample' is created with no read callback, the sample will be empty.  Use Sound::lock and Sound::unlock to place sound data into the sound if this is the case. */
pub static FMOD_OPENMEMORY             : c_uint = 0x00000800;  /* "name_or_data" will be interpreted as a pointer to memory instead of filename for creating sounds.  Use FMOD_CREATESOUNDEXINFO to specify length.  If used with FMOD_CREATESAMPLE or FMOD_CREATECOMPRESSEDSAMPLE, FMOD duplicates the memory into its own buffers.  Your own buffer can be freed after open.  If used with FMOD_CREATESTREAM, FMOD will stream out of the buffer whose pointer you passed in.  In this case, your own buffer should not be freed until you have finished with and released the stream.*/
pub static FMOD_OPENMEMORY_POINT       : c_uint = 0x10000000;  /* "name_or_data" will be interpreted as a pointer to memory instead of filename for creating sounds.  Use FMOD_CREATESOUNDEXINFO to specify length.  This differs to FMOD_OPENMEMORY in that it uses the memory as is, without duplicating the memory into its own buffers.  For Wii/PSP FMOD_HARDWARE supports this flag for the GCADPCM/VAG formats.  On other platforms FMOD_SOFTWARE must be used, as sound hardware on the other platforms (ie PC) cannot access main ram.  Cannot be freed after open, only after Sound::release.   Will not work if the data is compressed and FMOD_CREATECOMPRESSEDSAMPLE is not used. */
pub static FMOD_OPENRAW                : c_uint = 0x00001000;  /* Will ignore file format and treat as raw pcm.  Use FMOD_CREATESOUNDEXINFO to specify format.  Requires at least defaultfrequency, numchannels and format to be specified before it will open.  Must be little endian data. */
pub static FMOD_OPENONLY               : c_uint = 0x00002000;  /* Just open the file, dont prebuffer or read.  Good for fast opens for info, or when sound::readData is to be used. */
pub static FMOD_ACCURATETIME           : c_uint = 0x00004000;  /* For System::createSound - for accurate Sound::getLength/Channel::setPosition on VBR MP3, and MOD/S3M/XM/IT/MIDI files.  Scans file first, so takes longer to open. FMOD_OPENONLY does not affect this. */
pub static FMOD_MPEGSEARCH             : c_uint = 0x00008000;  /* For corrupted / bad MP3 files.  This will search all the way through the file until it hits a valid MPEG header.  Normally only searches for 4k. */
pub static FMOD_NONBLOCKING            : c_uint = 0x00010000;  /* For opening sounds and getting streamed subsounds (seeking) asyncronously.  Use Sound::getOpenState to poll the state of the sound as it opens or retrieves the subsound in the background. */
pub static FMOD_UNIQUE                 : c_uint = 0x00020000;  /* Unique sound, can only be played one at a time */
pub static FMOD_3D_HEADRELATIVE        : c_uint = 0x00040000;  /* Make the sound's position, velocity and orientation relative to the listener. */
pub static FMOD_3D_WORLDRELATIVE       : c_uint = 0x00080000;  /* Make the sound's position, velocity and orientation absolute (relative to the world). (DEFAULT) */
pub static FMOD_3D_INVERSEROLLOFF      : c_uint = 0x00100000;  /* This sound will follow the inverse rolloff model where mindistance = full volume, maxdistance = where sound stops attenuating, and rolloff is fixed according to the global rolloff factor.  (DEFAULT) */
pub static FMOD_3D_LINEARROLLOFF       : c_uint = 0x00200000;  /* This sound will follow a linear rolloff model where mindistance = full volume, maxdistance = silence.  Rolloffscale is ignored. */
pub static FMOD_3D_LINEARSQUAREROLLOFF : c_uint = 0x00400000;  /* This sound will follow a linear-square rolloff model where mindistance = full volume, maxdistance = silence.  Rolloffscale is ignored. */
pub static FMOD_3D_CUSTOMROLLOFF       : c_uint = 0x04000000;  /* This sound will follow a rolloff model defined by Sound::set3DCustomRolloff / Channel::set3DCustomRolloff.  */
pub static FMOD_3D_IGNOREGEOMETRY      : c_uint = 0x40000000;  /* Is not affect by geometry occlusion.  If not specified in Sound::setMode, or Channel::setMode, the flag is cleared and it is affected by geometry again. */
pub static FMOD_UNICODE                : c_uint = 0x01000000;  /* Filename is double-byte unicode. */
pub static FMOD_IGNORETAGS             : c_uint = 0x02000000;  /* Skips id3v2/asf/etc tag checks when opening a sound, to reduce seek/read overhead when opening files (helps with CD performance). */
pub static FMOD_LOWMEM                 : c_uint = 0x08000000;  /* Removes some features from samples to give a lower memory overhead, like Sound::getName.  See remarks. */
pub static FMOD_LOADSECONDARYRAM       : c_uint = 0x20000000;  /* Load sound into the secondary RAM of supported platform. On PS3, sounds will be loaded into RSX/VRAM. */
pub static FMOD_VIRTUAL_PLAYFROMSTART  : c_uint = 0x80000000;  /* For sounds that start virtual (due to being quiet or low importance), instead of swapping back to audible, and playing at the correct offset according to time, this flag makes the sound play from the start. */

/* FMOD_INITFLAGS values list */
pub static FMOD_INIT_NORMAL                    : c_uint = 0x00000000; /* All platforms - Initialize normally */
pub static FMOD_INIT_STREAM_FROM_UPDATE        : c_uint = 0x00000001; /* All platforms - No stream thread is created internally.  Streams are driven from System::update.  Mainly used with non-realtime outputs. */
pub static FMOD_INIT_3D_RIGHTHANDED            : c_uint = 0x00000002; /* All platforms - FMOD will treat +X as right, +Y as up and +Z as backwards (towards you). */
pub static FMOD_INIT_SOFTWARE_DISABLE          : c_uint = 0x00000004; /* All platforms - Disable software mixer to save memory.  Anything created with FMOD_SOFTWARE will fail and DSP will not work. */
pub static FMOD_INIT_OCCLUSION_LOWPASS         : c_uint = 0x00000008; /* All platforms - All FMOD_SOFTWARE (and FMOD_HARDWARE on 3DS and NGP) with FMOD_3D based voices will add a software lowpass filter effect into the DSP chain which is automatically used when Channel::set3DOcclusion is used or the geometry API. */
pub static FMOD_INIT_HRTF_LOWPASS              : c_uint = 0x00000010; /* All platforms - All FMOD_SOFTWARE (and FMOD_HARDWARE on 3DS and NGP) with FMOD_3D based voices will add a software lowpass filter effect into the DSP chain which causes sounds to sound duller when the sound goes behind the listener.  Use System::setAdvancedSettings to adjust cutoff frequency. */
pub static FMOD_INIT_DISTANCE_FILTERING        : c_uint = 0x00000200; /* All platforms - All FMOD_SOFTWARE with FMOD_3D based voices will add a software lowpass and highpass filter effect into the DSP chain which will act as a distance-automated bandpass filter. Use System::setAdvancedSettings to adjust the center frequency. */
pub static FMOD_INIT_REVERB_PREALLOCBUFFERS    : c_uint = 0x00000040; /* All platforms - FMOD Software reverb will preallocate enough buffers for reverb per channel, rather than allocating them and freeing them at runtime. */
pub static FMOD_INIT_ENABLE_PROFILE            : c_uint = 0x00000020; /* All platforms - Enable TCP/IP based host which allows FMOD Designer or FMOD Profiler to connect to it, and view memory, CPU and the DSP network graph in real-time. */
pub static FMOD_INIT_VOL0_BECOMES_VIRTUAL      : c_uint = 0x00000080; /* All platforms - Any sounds that are 0 volume will go virtual and not be processed except for having their positions updated virtually.  Use System::setAdvancedSettings to adjust what volume besides zero to switch to virtual at. */
pub static FMOD_INIT_WASAPI_EXCLUSIVE          : c_uint = 0x00000100; /* Win32 Vista only - for WASAPI output - Enable exclusive access to hardware, lower latency at the expense of excluding other applications from accessing the audio hardware. */
pub static FMOD_INIT_PS3_PREFERDTS             : c_uint = 0x00800000; /* PS3 only - Prefer DTS over Dolby Digital if both are supported. Note: 8 and 6 channel LPCM is always preferred over both DTS and Dolby Digital. */
pub static FMOD_INIT_PS3_FORCE2CHLPCM          : c_uint = 0x01000000; /* PS3 only - Force PS3 system output mode to 2 channel LPCM. */
pub static FMOD_INIT_DISABLEDOLBY              : c_uint = 0x00100000; /* Wii / 3DS - Disable Dolby Pro Logic surround. Speakermode will be set to STEREO even if user has selected surround in the system settings. */
pub static FMOD_INIT_SYSTEM_MUSICMUTENOTPAUSE  : c_uint = 0x00200000; /* Xbox 360 / PS3 - The "music" channelgroup which by default pauses when custom 360 dashboard / PS3 BGM music is played, can be changed to mute (therefore continues playing) instead of pausing, by using this flag. */
pub static FMOD_INIT_SYNCMIXERWITHUPDATE       : c_uint = 0x00400000; /* Win32/Wii/PS3/Xbox/Xbox 360 - FMOD Mixer thread is woken up to do a mix when System::update is called rather than waking periodically on its own timer. */
pub static FMOD_INIT_GEOMETRY_USECLOSEST       : c_uint = 0x04000000; /* All platforms - With the geometry engine, only process the closest polygon rather than accumulating all polygons the sound to listener line intersects. */
pub static FMOD_INIT_DISABLE_MYEARS_AUTODETECT : c_uint = 0x08000000; /* Win32 - Disables automatic setting of fmod::SpeakerMODE_STEREO to fmod::SpeakerMODE_MYEARS if the MyEars profile exists on the PC.  MyEars is HRTF 7.1 downmixing through headphones. */
pub static FMOD_INIT_PS3_DISABLEDTS            : c_uint = 0x10000000; /* PS3 only - Disable DTS output mode selection */
pub static FMOD_INIT_PS3_DISABLEDOLBYDIGITAL   : c_uint = 0x20000000; /* PS3 only - Disable Dolby Digital output mode selection */
pub static FMOD_INIT_7POINT1_DOLBYMAPPING      : c_uint = 0x40000000; /* PS3/PS4 only - FMOD uses the WAVEFORMATEX Microsoft 7.1 speaker mapping where the last 2 pairs of speakers are 'rears' then 'sides', but on PS3/PS4 these are mapped to 'surrounds' and 'backs'.  Use this flag to swap fmod's last 2 pair of speakers on PS3/PS4 to avoid needing to do a special case for these platforms. */

pub static FMOD_TIMEUNIT_MS               : FmodTimeUnit = FmodTimeUnit(0x00000001);  /* Milliseconds. */
pub static FMOD_TIMEUNIT_PCM              : FmodTimeUnit = FmodTimeUnit(0x00000002);  /* PCM samples, related to milliseconds * samplerate / 1000. */
pub static FMOD_TIMEUNIT_PCMBYTES         : FmodTimeUnit = FmodTimeUnit(0x00000004);  /* Bytes, related to PCM samples * channels * datawidth (ie 16bit = 2 bytes). */
pub static FMOD_TIMEUNIT_RAWBYTES         : FmodTimeUnit = FmodTimeUnit(0x00000008);  /* Raw file bytes of (compressed) sound data (does not include headers).  Only used by Sound::getLength and Channel::getPosition. */
pub static FMOD_TIMEUNIT_PCMFRACTION      : FmodTimeUnit = FmodTimeUnit(0x00000010);  /* Fractions of 1 PCM sample.  Unsigned int range 0 to 0xFFFFFFFF.  Used for sub-sample granularity for DSP purposes. */
pub static FMOD_TIMEUNIT_MODORDER         : FmodTimeUnit = FmodTimeUnit(0x00000100);  /* MOD/S3M/XM/IT.  Order in a sequenced module format.  Use Sound::getFormat to determine the PCM format being decoded to. */
pub static FMOD_TIMEUNIT_MODROW           : FmodTimeUnit = FmodTimeUnit(0x00000200);  /* MOD/S3M/XM/IT.  Current row in a sequenced module format.  Sound::getLength will return the number of rows in the currently playing or seeked to pattern. */
pub static FMOD_TIMEUNIT_MODPATTERN       : FmodTimeUnit = FmodTimeUnit(0x00000400);  /* MOD/S3M/XM/IT.  Current pattern in a sequenced module format.  Sound::getLength will return the number of patterns in the song and Channel::getPosition will return the currently playing pattern. */
pub static FMOD_TIMEUNIT_SENTENCE_MS      : FmodTimeUnit = FmodTimeUnit(0x00010000);  /* Currently playing subsound in a sentence time in milliseconds. */
pub static FMOD_TIMEUNIT_SENTENCE_PCM     : FmodTimeUnit = FmodTimeUnit(0x00020000);  /* Currently playing subsound in a sentence time in PCM Samples, related to milliseconds * samplerate / 1000. */
pub static FMOD_TIMEUNIT_SENTENCE_PCMBYTES: FmodTimeUnit = FmodTimeUnit(0x00040000);  /* Currently playing subsound in a sentence time in bytes, related to PCM samples * channels * datawidth (ie 16bit = 2 bytes). */
pub static FMOD_TIMEUNIT_SENTENCE         : FmodTimeUnit = FmodTimeUnit(0x00080000);  /* Currently playing sentence index according to the channel. */
pub static FMOD_TIMEUNIT_SENTENCE_SUBSOUND: FmodTimeUnit = FmodTimeUnit(0x00100000);  /* Currently playing subsound index in a sentence. */
pub static FMOD_TIMEUNIT_BUFFERED         : FmodTimeUnit = FmodTimeUnit(0x10000000);  /* Time value as seen by buffered stream.  This is always ahead of audible time, and is only used for processing. */

pub static FMOD_MEMBITS_OTHER             : FmodMemoryBits = FmodMemoryBits(0x00000001);  /* Memory not accounted for by other types */
pub static FMOD_MEMBITS_STRING            : FmodMemoryBits = FmodMemoryBits(0x00000002);  /* String data */
pub static FMOD_MEMBITS_SYSTEM            : FmodMemoryBits = FmodMemoryBits(0x00000004);  /* System object and various internals */
pub static FMOD_MEMBITS_PLUGINS           : FmodMemoryBits = FmodMemoryBits(0x00000008);  /* Plugin objects and internals */
pub static FMOD_MEMBITS_OUTPUT            : FmodMemoryBits = FmodMemoryBits(0x00000010);  /* Output module object and internals */
pub static FMOD_MEMBITS_CHANNEL           : FmodMemoryBits = FmodMemoryBits(0x00000020);  /* Channel related memory */
pub static FMOD_MEMBITS_CHANNELGROUP      : FmodMemoryBits = FmodMemoryBits(0x00000040);  /* ChannelGroup objects and internals */
pub static FMOD_MEMBITS_CODEC             : FmodMemoryBits = FmodMemoryBits(0x00000080);  /* Codecs allocated for streaming */
pub static FMOD_MEMBITS_FILE              : FmodMemoryBits = FmodMemoryBits(0x00000100);  /* Codecs allocated for streaming */
pub static FMOD_MEMBITS_SOUND             : FmodMemoryBits = FmodMemoryBits(0x00000200);  /* Sound objects and internals */
pub static FMOD_MEMBITS_SOUND_SECONDARYRAM: FmodMemoryBits = FmodMemoryBits(0x00000400);  /* Sound data stored in secondary RAM */
pub static FMOD_MEMBITS_SOUNDGROUP        : FmodMemoryBits = FmodMemoryBits(0x00000800);  /* SoundGroup objects and internals */
pub static FMOD_MEMBITS_STREAMBUFFER      : FmodMemoryBits = FmodMemoryBits(0x00001000);  /* Stream buffer memory */
pub static FMOD_MEMBITS_DSPCONNECTION     : FmodMemoryBits = FmodMemoryBits(0x00002000);  /* DSPConnection objects and internals */
pub static FMOD_MEMBITS_DSP               : FmodMemoryBits = FmodMemoryBits(0x00004000);  /* DSP implementation objects */
pub static FMOD_MEMBITS_DSPCODEC          : FmodMemoryBits = FmodMemoryBits(0x00008000);  /* Realtime file format decoding DSP objects */
pub static FMOD_MEMBITS_PROFILE           : FmodMemoryBits = FmodMemoryBits(0x00010000);  /* Profiler memory footprint. */
pub static FMOD_MEMBITS_RECORDBUFFER      : FmodMemoryBits = FmodMemoryBits(0x00020000);  /* Buffer used to store recorded data from microphone */
pub static FMOD_MEMBITS_REVERB            : FmodMemoryBits = FmodMemoryBits(0x00040000);  /* Reverb implementation objects */
pub static FMOD_MEMBITS_REVERBCHANNELPROPS: FmodMemoryBits = FmodMemoryBits(0x00080000);  /* Reverb channel properties structs */
pub static FMOD_MEMBITS_GEOMETRY          : FmodMemoryBits = FmodMemoryBits(0x00100000);  /* Geometry objects and internals */
pub static FMOD_MEMBITS_SYNCPOINT         : FmodMemoryBits = FmodMemoryBits(0x00200000);  /* Sync point memory. */
pub static FMOD_MEMBITS_ALL               : FmodMemoryBits = FmodMemoryBits(0xffffffff);  /* All memory used by FMOD Ex */

pub static FMOD_EVENT_MEMBITS_EVENTSYSTEM          : c_uint = 0x00000001;  /* EventSystem and various internals */
pub static FMOD_EVENT_MEMBITS_MUSICSYSTEM          : c_uint = 0x00000002;  /* MusicSystem and various internals */
pub static FMOD_EVENT_MEMBITS_FEV                  : c_uint = 0x00000004;  /* Definition of objects contained in all loaded projects e.g. events, groups, categories */
pub static FMOD_EVENT_MEMBITS_MEMORYFSB            : c_uint = 0x00000008;  /* Data loaded with preloadFSB */
pub static FMOD_EVENT_MEMBITS_EVENTPROJECT         : c_uint = 0x00000010;  /* EventProject objects and internals */
pub static FMOD_EVENT_MEMBITS_EVENTGROUPI          : c_uint = 0x00000020;  /* EventGroup objects and internals */
pub static FMOD_EVENT_MEMBITS_SOUNDBANKCLASS       : c_uint = 0x00000040;  /* Objects used to manage wave banks */
pub static FMOD_EVENT_MEMBITS_SOUNDBANKLIST        : c_uint = 0x00000080;  /* Data used to manage lists of wave bank usage */
pub static FMOD_EVENT_MEMBITS_STREAMINSTANCE       : c_uint = 0x00000100;  /* Stream objects and internals */
pub static FMOD_EVENT_MEMBITS_SOUNDDEFCLASS        : c_uint = 0x00000200;  /* Sound definition objects */
pub static FMOD_EVENT_MEMBITS_SOUNDDEFDEFCLASS     : c_uint = 0x00000400;  /* Sound definition static data objects */
pub static FMOD_EVENT_MEMBITS_SOUNDDEFPOOL         : c_uint = 0x00000800;  /* Sound definition pool data */
pub static FMOD_EVENT_MEMBITS_REVERBDEF            : c_uint = 0x00001000;  /* Reverb definition objects */
pub static FMOD_EVENT_MEMBITS_EVENTREVERB          : c_uint = 0x00002000;  /* Reverb objects */
pub static FMOD_EVENT_MEMBITS_USERPROPERTY         : c_uint = 0x00004000;  /* User property objects */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCE        : c_uint = 0x00008000;  /* Event instance base objects */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCE_COMPLEX: c_uint = 0x00010000;  /* Complex event instance objects */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCE_SIMPLE : c_uint = 0x00020000;  /* Simple event instance objects */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCE_LAYER  : c_uint = 0x00040000;  /* Event layer instance objects */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCE_SOUND  : c_uint = 0x00080000;  /* Event sound instance objects */
pub static FMOD_EVENT_MEMBITS_EVENTENVELOPE        : c_uint = 0x00100000;  /* Event envelope objects */
pub static FMOD_EVENT_MEMBITS_EVENTENVELOPEDEF     : c_uint = 0x00200000;  /* Event envelope definition objects */
pub static FMOD_EVENT_MEMBITS_EVENTPARAMETER       : c_uint = 0x00400000;  /* Event parameter objects */
pub static FMOD_EVENT_MEMBITS_EVENTCATEGORY        : c_uint = 0x00800000;  /* Event category objects */
pub static FMOD_EVENT_MEMBITS_EVENTENVELOPEPOINT   : c_uint = 0x01000000;  /* Event envelope point object+s */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCEPOOL    : c_uint = 0x02000000;  /* Event instance pool data */
pub static FMOD_EVENT_MEMBITS_ALL                  : c_uint = 0xffffffff;  /* All memory used by FMOD Event System */

/* All event instance memory */
pub static FMOD_EVENT_MEMBITS_EVENTINSTANCE_GROUP  : c_uint = FMOD_EVENT_MEMBITS_EVENTINSTANCE | FMOD_EVENT_MEMBITS_EVENTINSTANCE_COMPLEX | FMOD_EVENT_MEMBITS_EVENTINSTANCE_SIMPLE | FMOD_EVENT_MEMBITS_EVENTINSTANCE_LAYER | FMOD_EVENT_MEMBITS_EVENTINSTANCE_SOUND;

/* All sound definition memory */
pub static FMOD_EVENT_MEMBITS_SOUNDDEF_GROUP       : c_uint = FMOD_EVENT_MEMBITS_SOUNDDEFCLASS | FMOD_EVENT_MEMBITS_SOUNDDEFDEFCLASS | FMOD_EVENT_MEMBITS_SOUNDDEFPOOL;