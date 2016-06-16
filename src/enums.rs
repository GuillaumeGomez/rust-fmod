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

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Error codes. Returned from every function.
pub enum Status {
    /// No errors.
    Ok,
    /// Tried to call lock a second time before unlock was called.
    AlreadyLocked,
    /// Tried to call a function on a data type that does not allow this type of functionality (ie
    /// calling Sound::lock on a streaming sound).
    BadCommand,
    /// Neither NTSCSI nor ASPI could be initialised.
    CDDADrivers,
    /// An error occurred while initialising the CDDA subsystem.
    CDDAInit,
    /// Couldn't find the specified device.
    CDDAInvalidDevice,
    /// No audio tracks on the specified disc.
    CDDANoAudio,
    /// No CD/DVD devices were found.
    CDDANoDevices,
    /// No disc present in the specified drive.
    CDDANoDisc,
    /// A CDDA read error occurred.
    CDDARead,
    /// Error trying to allocate a channel.
    ChannelAlloc,
    /// The specified channel has been reused to play another sound.
    ChannelStolen,
    /// A Win32 COM related error occured. COM failed to initialize or a QueryInterface failed
    /// meaning a Windows codec or driver was not installed properly.
    COM,
    /// DMA Failure. See debug output for more information.
    DMA,
    /// DSP connection error. Connection possibly caused a cyclic dependancy. Or tried to connect a
    /// tree too many units deep (more than 128).
    DSPConnection,
    /// DSP Format error. A DSP unit may have attempted to connect to this network with the wrong
    /// format.
    DSPFormat,
    /// DSP connection error. Couldn't find the DSP unit specified.
    DSPNotFound,
    /// DSP error. Cannot perform this operation while the network is in the middle of running. This
    /// will most likely happen if a connection or disconnection is attempted in a DSP callback.
    DSPRunning,
    /// DSP connection error. The unit being connected to or disconnected should only have 1 input
    /// or output.
    DSPTooManyConnections,
    /// Error loading file.
    FileBad,
    /// Couldn't perform seek operation. This is a limitation of the medium (ie netstreams) or the
    /// file format.
    FileCouldNotSeek,
    /// Media was ejected while reading.
    FileDiskEjected,
    /// End of file unexpectedly reached while trying to read essential data (truncated data ?).
    FileEOF,
    /// File not found.
    FileNotFound,
    /// Unwanted file access occured.
    FileUnwanted,
    /// Unsupported file or audio format.
    Format,
    /// A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.
    HTTP,
    /// The specified resource requires authentication or is forbidden.
    HTTPAccess,
    /// Proxy authentication is required to access the specified resource.
    HTTPProxyAuth,
    /// A HTTP server error occurred.
    HTTPServerError,
    /// The HTTP request timed out.
    HTTPTimeout,
    /// FMOD was not initialized correctly to support this function.
    Initialization,
    /// Cannot call this command after System::init.
    Initialized,
    /// An error occured that wasn't supposed to. Contact support.
    Internal,
    /// On Xbox 360, this memory address passed to FMOD must be physical, (ie allocated with
    /// XPhysicalAlloc.)
    InvalidAddress,
    /// Value passed in was a NaN, Inf or denormalized float.
    InvalidFloat,
    /// An invalid object handle was used.
    InvalidHandle,
    /// An invalid parameter was passed to this function.
    InvalidParam,
    /// An invalid seek position was passed to this function.
    InvalidPosition,
    /// An invalid speaker was passed to this function based on the current speaker mode.
    InvalidSpeaker,
    /// The syncpoint did not come from this sound handle.
    InvalidSyncPoint,
    /// The vectors passed in are not unit length, or perpendicular.
    InvalidVector,
    /// Reached maximum audible playback count for this sound's soundgroup.
    MaxAudible,
    /// Not enough memory or resources.
    Memory,
    /// Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if
    /// FMOD_CREATECOMPRESSEDSAMPLE was used.
    MemoryCantPoint,
    /// Not enough memory or resources on console sound ram.
    MemorySRAM,
    /// Tried to call a command on a 3d sound when the command was meant for 2d sound.
    Needs2D,
    /// Tried to call a command on a 2d sound when the command was meant for 3d sound.
    Needs3D,
    /// Tried to use a feature that requires hardware support. (ie trying to play a GCADPCM
    /// compressed sound in software on Wii).
    NeedsHardware,
    /// Tried to use a feature that requires the software engine. Software engine has either been
    /// turned off, or command was executed on a hardware channel which does not support this
    /// feature.
    NeedsSoftware,
    /// Couldn't connect to the specified host.
    NetConnect,
    /// A socket error occurred. This is a catch-all for socket-related errors not listed elsewhere.
    NetSocketError,
    /// The specified URL couldn't be resolved.
    NetURL,
    /// Operation on a non-blocking socket could not complete immediately.
    NetWouldBlock,
    /// Operation could not be performed because specified sound/DSP connection is not ready.
    NotReady,
    /// Error initializing output device, but more specifically, the output device is already in
    /// use and cannot be reused.
    OutputAllocated,
    /// Error creating hardware sound buffer.
    OutputCreateBuffer,
    /// A call to a standard soundcard driver failed, which could possibly mean a bug in the driver
    /// or resources were missing or exhausted.
    OutputDriverCall,
    /// Error enumerating the available driver list. List may be inconsistent due to a recent device
    /// addition or removal.
    OutputEnumeration,
    /// Soundcard does not support the minimum features needed for this soundsystem (16bit stereo
    /// output).
    OutputFormat,
    /// Error initializing output device.
    OutputInit,
    /// FMOD_HARDWARE was specified but the sound card does not have the resources necessary to play
    /// it.
    OutputNoHardware,
    /// Attempted to create a software sound but no software channels were specified in
    /// System::init.
    OutputNoSoftware,
    /// Panning only works with mono or stereo sound sources.
    Pan,
    /// An unspecified error has been returned from a 3rd party plugin.
    Plugin,
    /// The number of allowed instances of a plugin has been exceeded.
    PluginInstances,
    /// A requested output, dsp unit type or codec was not available.
    PluginMissing,
    /// A resource that the plugin requires cannot be found. (ie the DLS file for MIDI playback or
    /// other DLLs that it needs to load)
    PluginResource,
    /// The specified sound is still in use by the event system, call EventSystem::unloadFSB before
    /// trying to release it.
    Preloaded,
    /// The specified sound is still in use by the event system, wait for the event which is using
    /// it finish with it.
    ProgrammerSound,
    /// An error occured trying to initialize the recording device.
    Record,
    /// Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an
    /// invalid instance number or the reverb doesnt exist.
    ReverbInstance,
    /// This subsound is already being used by another sound, you cannot have more than one parent
    /// to a sound. Null out the other parent's entry first.
    SubsoundAllocated,
    /// Shared subsounds cannot be replaced or moved from their parent stream, such as when the
    /// parent stream is an FSB file.
    SubsoundCantMove,
    /// The subsound's mode bits do not match with the parent sound's mode bits. See documentation
    /// for function that it was called with.
    SubsoundMode,
    /// The error occured because the sound referenced contains subsounds when it shouldn't have, or
    /// it doesn't contain subsounds when it should have. The operation may also not be able to be
    /// performed on a parent sound, or a parent sound was played without setting up a sentence
    /// first.
    Subsounds,
    /// The specified tag could not be found or there are no tags.
    TagNotFound,
    /// The sound created exceeds the allowable input channel count. This can be increased using the
    /// maxinputchannels parameter in System::setSoftwareFormat.
    TooManyChannels,
    /// Something in FMOD hasn't been implemented when it should be ! contact support !
    Unimplemented,
    /// This command failed because System::init or System::setDriver was not called.
    Uninitialized,
    /// A command issued was not supported by this object. Possibly a plugin without certain
    /// callbacks specified.
    Unsupported,
    /// An error caused by System::update occured.
    Update,
    /// The version number of this file format is not supported.
    Version,
    /// An Event failed to be retrieved, most likely due to 'just fail' being specified as the max
    /// playbacks behavior.
    EventFailed,
    /// Can't execute this command on an EVENT_INFOONLY event.
    EventInfoOnly,
    /// An error occured that wasn't supposed to. See debug log for reason.
    EventInternal,
    /// Event failed because 'Max streams' was hit when FMOD_EVENT_INIT_FAIL_ON_MAXSTREAMS was
    /// specified.
    EventMaxStreams,
    /// FSB mismatches the FEV it was compiled with, the stream/sample mode it was meant to be
    /// created with was different, or the FEV was built for a different platform.
    EventMismatch,
    /// A category with the same name already exists.
    EventNameConflict,
    /// The requested event, event group, event category or event property could not be found.
    EventNotFound,
    /// Tried to call a function on a complex event that's only supported by simple events.
    EventNeedsSimple,
    /// An event with the same GUID already exists.
    EventGuidConflict,
    /// The specified project or bank has already been loaded. Having multiple copies of the same
    /// project loaded simultaneously is forbidden.
    EventAlreadyLoaded,
    /// Music system is not initialized probably because no music data is loaded.
    MusicUninitialized,
    /// The requested music entity could not be found.
    MusicNotFound,
    /// The music callback is required, but it has not been set.
    MusicNoCallback,
    /// Makes sure this enum is signed 32bit.
    StatusForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// When creating a multichannel sound, FMOD will pan them to their default speaker locations:
/// * For example a 6 channel sound will default to one channel per 5.1 output speaker.
/// * Another example is a stereo sound. It will default to left = front left, right = front right.
/// * This is for sounds that are not 'default'. For example you might have a sound that is 6
///   channels but actually made up of 3 stereo pairs, that should all be located in front left,
///   front right only.
pub enum SpeakerMapType {
    /// This is the default, and just means FMOD decides which speakers it puts the source channels.
    Default,
    /// This means the sound is made up of all mono sounds. All voices will be panned to the front
    /// center by default in this case.
    AllMono,
    /// This means the sound is made up of all stereo sounds. All voices will be panned to front
    /// left and front right alternating every second channel.
    AllStereo,
    /// Map a 5.1 sound to use protools L C R Ls Rs LFE mapping. Will return an error if not a 6
    /// channel sound.
    _51ProTools,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These definitions describe the native format of the hardware or software buffer that will be used.
pub enum SoundFormat {
    /// Uninitialized / unknown.
    None,
    /// 8bit integer PCM data.
    PCM8,
    /// 16bit integer PCM data.
    PCM16,
    /// 24bit integer PCM data.
    PCM24,
    /// 32bit integer PCM data.
    PCM32,
    /// 32bit floating point PCM data.
    PCMFloat,
    /// Compressed Nintendo 3DS/Wii DSP data.
    GCADPCM,
    /// Compressed IMA ADPCM data.
    IMAADPCM,
    /// Compressed PlayStation Portable ADPCM data.
    VAG,
    /// Compressed PSVita ADPCM data.
    HEVAG,
    /// Compressed Xbox360 XMA data.
    XMA,
    /// Compressed MPEG layer 2 or 3 data.
    MPEG,
    /// Compressed CELT data.
    CELT,
    /// Compressed PSVita ATRAC9 data.
    AT9,
    /// Compressed Xbox360 xWMA data.
    XWMA,
    /// Compressed Vorbis data.
    VORBIS,
    /// Maximum number of sound formats supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These definitions describe the type of song being played.
pub enum SoundType {
    /// 3rd party / unknown plugin format.
    Unknown,
    /// AIFF.
    AIFF,
    /// Microsoft Advanced Systems Format (ie WMA/ASF/WMV).
    ASF,
    /// Sony ATRAC 3 format
    AT3,
    /// Digital CD audio.
    CDDA,
    /// Sound font / downloadable sound bank.
    DLS,
    /// FLAC lossless codec.
    FLAC,
    /// FMOD Sample Bank.
    FSB,
    /// Nintendo GameCube/Wii ADPCM
    GCADPCM,
    /// Impulse Tracker.
    IT,
    /// MIDI. extracodecdata is a pointer to an FMOD_MIDI_EXTRACODECDATA structure.
    MIDI,
    /// Protracker / Fasttracker MOD.
    MOD,
    /// MP2/MP3 MPEG.
    MPEG,
    /// Ogg vorbis.
    OGGVORBIS,
    /// Information only from ASX/PLS/M3U/WAX playlists
    Playlist,
    /// Raw PCM data.
    Raw,
    /// ScreamTracker 3.
    S3M,
    /// Sound font 2 format.
    SF2,
    /// User created sound.
    User,
    /// Microsoft WAV.
    WAV,
    /// FastTracker 2 XM.
    XM,
    /// Xbox360 XMA
    XMA,
    /// PlayStation Portable ADPCM VAG format.
    VAG,
    /// iPhone hardware decoder, supports AAC, ALAC and MP3. extracodecdata is a pointer to an
    /// FMOD_AUDIOQUEUE_EXTRACODECDATA structure.
    AudioQueue,
    /// Xbox360 XWMA
    XWMA,
    /// 3DS BCWAV container format for DSP ADPCM and PCM
    BCWAV,
    /// NGP ATRAC 9 format
    AT9,
    /// Raw vorbis
    VORBIS,
    /// Microsoft Media Foundation wrappers, supports ASF/WMA
    MediaFoundation,
    /// Maximum number of sound types supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// List of tag types that could be stored within a sound. These include id3 tags, metadata from
/// netstreams and vorbis/asf data.
pub enum TagType {
    Unknown = 0,
    ID3V1,
    ID3V2,
    VORBISComment,
    ShoutCast,
    IceCast,
    ASF,
    MIDI,
    Playlist,
    Fmod,
    User,

    /// Maximum number of tag types supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// List of data types that can be returned by
/// [`Sound::get_tag`](../../struct.Sound.html#method.get_tag)
pub enum TagDataType {
    Binary = 0,
    Int,
    Float,
    String,
    StringUTF16,
    StringUTF16BE,
    StringUTF8,
    CDTOC,
    /// Maximum number of tag datatypes supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Special channel index values for FMOD functions.
pub enum ChannelIndex {
    /// For a channel index, FMOD chooses a free voice using the priority system.
    Free  = -1,
    /// For a channel index, re-use the channel handle that was passed in.
    ReUse = -2,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// List of windowing methods used in spectrum analysis to reduce leakage / transient signals
/// intefering with the analysis. This is a problem with analysis of continuous signals that only
/// have a small portion of the signal sample (the fft window size). Windowing the signal with a
/// curve or triangle tapers the sides of the fft window to help alleviate this problem.
pub enum DspFftWindow {
    /// w[n] = 1.0
    Rect,
    /// w[n] = TRI(2n/N)
    Triangle,
    /// w[n] = 0.54 - (0.46 * COS(n/N) )
    Hamming,
    /// w[n] = 0.5 *  (1.0  - COS(n/N) )
    Hanning,
    /// w[n] = 0.42 - (0.5  * COS(n/N) ) + (0.08 * COS(2.0 * n/N) )
    BlackMan,
    /// w[n] = 0.35875 - (0.48829 * COS(1.0 * n/N)) + (0.14128 * COS(2.0 * n/N)) - (0.01168 *
    /// COS(3.0 * n/N))
    BlackManHarris,
    /// Maximum number of FFT window types supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Types of delay that can be used with
/// [`Channel::set_delay`](../../struct.Channel.html#method.set_delay) /
/// [`Channel::get_delay`](../../struct.Channel.html#method.get_delay).
pub enum DelayType {
    /// Delay at the end of the sound in milliseconds. Use delayhi only.
    /// [`Channel::is_playing`](../../struct.Channel.html#method.is_playing) will remain true until
    /// this delay has passed even though the sound itself has stopped playing.
    EndMS,
    /// Time the sound started if [`Channel::get_delay`](../../struct.Channel.html#method.get_delay)
    /// is used, or if [`Channel::set_delay`](../../struct.Channel.html#method.set_delay) is used,
    /// the sound will delay playing until this exact tick.
    DSPClockStart,
    /// Time the sound should end. If this is non-zero, the channel will go silent at this exact
    /// tick.
    DSPClockEnd,
    /// Time the sound should pause. If this is non-zero, the channel will pause at this exact tick.
    DSPClockPause,
    /// Maximum number of tag datatypes supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These output types are used with [`Sys::set_output`](../../struct.Sys.html#method.set_output) /
/// [`Sys::get_output`](../../struct.Sys.html#method.get_output), to choose which output method to use.
pub enum OutputType {
    /// Picks the best output mode for the platform. This is the default.
    AutoDetect,
    /// All             - 3rd party plugin, unknown. This is for use with
    /// [`Sys::get_output`](../../struct.Sys.html#method.get_output) only.
    Unknown,
    /// All             - All calls in this mode succeed but make no sound.
    NoSound,
    /// All             - Writes output to fmodoutput.wav by default. Use the 'extradriverdata'
    /// parameter in [`Sys::init`](../../struct.Sys.html#method.init), by simply passing the
    /// filename as a string, to set the wav filename.
    WAVWriter,
    /// All             - Non-realtime version of FMOD_OUTPUTYPE_NOSOUND. User can drive mixer with
    /// [`Sys::update`](../../struct.Sys.html#method.update) at whatever rate they want.
    NoSoundNRT,
    /// All             - Non-realtime version of FMOD_OUTPUTYPE_WAVWRITER. User can drive mixer
    /// with [`Sys::update`](../../struct.Sys.html#method.update) at whatever rate they want.
    WAVWriterNRT,
    /// Win32/Win64     - DirectSound output.                     (Default on Windows XP and below)
    DSound,
    /// Win32/Win64     - Windows Multimedia output.
    WinMM,
    /// Win32           - Windows Audio Session API.              (Default on Windows Vista and
    /// above)
    WASAPI,
    /// Win32           - Low latency ASIO 2.0 driver.
    ASIO,
    /// Linux/Linux64   - Open Sound System output.               (Default on Linux, third
    /// preference)
    OSS,
    /// Linux/Linux64   - Advanced Linux Sound Architecture output. (Default on Linux, second
    /// preference if available)
    ALSA,
    /// Linux/Linux64   - Enlightment Sound Daemon output.
    ESD,
    /// Linux/Linux64   - PulseAudio output.                      (Default on Linux, first
    /// preference if available)
    PulseAudio,
    /// Mac             - Macintosh CoreAudio output.             (Default on Mac)
    CoreAudio,
    /// Xbox 360        - Native Xbox360 output.                  (Default on Xbox 360)
    Xbox360,
    /// PSP             - Native PSP output.                      (Default on PSP)
    PSP,
    /// PS3             - Native PS3 output.                      (Default on PS3)
    PS3,
    /// NGP             - Native NGP output.                      (Default on NGP)
    NGP,
    /// Wii             - Native Wii output.                      (Default on Wii)
    Wii,
    /// 3DS             - Native 3DS output                       (Default on 3DS)
    _3DS,
    /// Android         - Java Audio Track output.                (Default on Android 2.2 and below)
    AudioTrack,
    /// Android         - OpenSL ES output.                       (Default on Android 2.3 and above)
    OpenSL,
    /// Native Client   - Native Client output.                   (Default on Native Client)
    NACL,
    /// Wii U           - Native Wii U output.                    (Default on Wii U)
    WiiU,
    /// BlackBerry      - Native BlackBerry asound output.        (Default on BlackBerry)
    ASound,
    /// Orbis           - Audio Out output.                       (Default on Orbis)
    AudioOut,
    /// Durango         - XAudio2 output.
    XAudio,
    /// Maximum number of output types supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
// FIXME
/// These are speaker types defined for use with the
/// [`Channel::set_speaker_level`](../../struct.Channel.html#method.set_speaker_level) command. It
/// can also be used for speaker placement in the
/// [`Sys::set_3D_speaker_position`](../../struct.Sys.html#method.set_3D_speaker_position) command.
pub enum Speaker {
    FrontLeft,
    FrontRight,
    FrontCenter,
    LowFrequency,
    BackLeft,
    BackRight,
    SideLeft,
    SideRight,
    /// Maximum number of speaker types supported.
    Max,
    // For use with_MONO and Channel::SetSpeakerLevels. Mapped to same value asSpeaker_FRONT_LEFT.
    //SpeakerMono        = 0,
    /// A non speaker. Use this with ASIO mapping to ignore a speaker.
    Null        = 65535,
    // For use with_7POINT1 on PS3 where the extra speakers are surround back inside of side
    // speakers.
    //SpeakerSBL         = 6,
    // For use with_7POINT1 on PS3 where the extra speakers are surround back inside of side
    // speakers.
    //SpeakerSBR         = 7,
    /// Makes sure this enum is signed 32bit.
    ForceInt    = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These are speaker types defined for use with the
/// [`Sys::set_speaker_mode`](../../struct.Sys.html#method.set_speaker_mode) or
/// [`Sys::get_speaker_mode`](../../struct.Sys.html#method.get_speaker_mode) command.
pub enum SpeakerMode {
    /// There is no specific . Sound channels are mapped in order of input to output. Use
    /// [`Sys::set_software_format`](../../struct.Sys.html#method.set_software_format) to specify
    /// speaker count. See remarks for more information.
    Raw,
    /// The speakers are monaural.
    Mono,
    /// The speakers are stereo (DEFAULT).
    Stereo,
    /// 4 speaker setup. This includes front left, front right, rear left, rear right.
    Quad,
    /// 5 speaker setup. This includes front left, front right, center, rear left, rear right.
    Surround,
    /// 5.1 speaker setup. This includes front left, front right, center, rear left, rear right and
    /// a subwoofer.
    _5Point1,
    /// 7.1 speaker setup. This includes front left, front right, center, rear left, rear right,
    /// side left, side right and a subwoofer.
    _7Point1,
    /// Stereo compatible output, embedded with surround information. SRS 5.1/Prologic/Prologic2
    /// decoders will split the signal into a 5.1 speaker set-up or SRS virtual surround will decode
    /// into a 2-speaker/headphone setup. See remarks about limitations.
    SRS5_1_Matrix,
    /// Stereo compatible output, embedded with surround information. Dolby Pro Logic II decoders
    /// will split the signal into a 5.1 speaker set-up.
    DOLBY5_1_Matrix,
    /// Stereo output, but data is encoded using personalized HRTF algorithms. See myears.net.au
    MYears,
    /// Maximum number of speaker modes supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// List of interpolation types that the FMOD Ex software mixer supports.
pub enum DspResampler {
    /// No interpolation. High frequency aliasing hiss will be audible depending on the sample rate
    /// of the sound.
    NoInterp,
    /// Linear interpolation (default method). Fast and good quality, causes very slight lowpass
    /// effect on low frequency sounds.
    Linear,
    /// Cubic interpolation. Slower than linear interpolation but better quality.
    Cubic,
    /// 5 point spline interpolation. Slowest resampling method but best quality.
    Spline,
    /// Maximum number of resample methods supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These are plugin types defined for use with the
/// [`Sys::get_num_plugins`](../../struct.Sys.html#method.get_num_plugins),
/// [`Sys::get_plugin_info`](../../struct.Sys.html#method.get_plugin_info) and
/// [`Sys::unload_plugin`](../../struct.Sys.html#method.unload_plugin) functions.
pub enum PluginType {
    /// The plugin type is an output module. FMOD mixed audio will play through one of these
    /// devices.
    Output,
    /// The plugin type is a file format codec. FMOD will use these codecs to load file formats for
    /// playback.
    Codec,
    /// The plugin type is a DSP unit. FMOD will use these plugins as part of its DSP network to
    /// apply effects to output or generate sound in realtime.
    DSP,
    /// Maximum number of plugin types supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These values describe what state a sound is in after FMOD_NONBLOCKING has been used to open it.
pub enum OpenState {
    /// Opened and ready to play.
    Ready = 0,
    /// Initial load in progress.
    Loading,
    /// Failed to open - file not found, out of memory etc. See return value of
    /// [`Sound::get_open_state`](../../struct.Sound.html#method.get_open_state) for what happened.
    Error,
    /// Connecting to remote host (internet sounds only).
    Connecting,
    /// Buffering data.
    Buffering,
    /// Seeking to subsound and re-flushing stream buffer.
    Seeking,
    /// Ready and playing, but not possible to release at this time without stalling the main
    /// thread.
    Playing,
    /// Seeking within a stream to a different position.
    SetPosition,
    /// Maximum number of open state types.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These callback types are used with
/// [`Channel::set_callback`](../../struct.Channel.html#method.set_callback).
pub enum SystemCallbackType {
    /// Called from [`Sys::update`](../../struct.Sys.html#method.update) when the enumerated list of
    /// devices has changed.
    DeviceListChanged,
    /// Called from [`Sys::update`](../../struct.Sys.html#method.update) when an output device has
    /// been lost due to control panel parameter changes and FMOD cannot automatically recover.
    DeviceLost,
    /// Called directly when a memory allocation fails somewhere in FMOD. (NOTE - 'system' will be
    /// NULL in this callback type.)
    MemoryAllocationFailed,
    /// Called directly when a thread is created.
    ThreadCreated,
    /// Called when a bad connection was made with
    /// [`Dsp::add_input`](../../struct.Dsp.html#method.add_input). Usually called from mixer thread
    /// because that is where the connections are made.
    BadDSPConnection,
    /// Called when too many effects were added exceeding the maximum tree depth of 128. This is
    /// most likely caused by accidentally adding too many DSP effects. Usually called from mixer
    /// thread because that is where the connections are made.
    BadDSPLevel,
    /// Called directly when a thread is destroyed.
    ThreadDestroyed,
    /// Maximum number of callback types supported.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These flags are used with
/// [`SoundGroup::set_max_audible_behavior`](../../struct.SoundGroup.html#method.set_max_audible_behavior)
/// to determine what happens when more sounds are played than are specified with
/// [`SoundGroup::set_max_audible`](../../struct.SoundGroup.html#method.set_max_audible).
pub enum SoundGroupBehavior {
    /// Any sound played that puts the sound count over the
    /// [`SoundGroup::set_max_audible`](../../struct.SoundGroup.html#method.set_max_audible)
    /// setting, will simply fail during [`Sound::play`](../../struct.Sound.html#method.play).
    Fail,
    /// Any sound played that puts the sound count over the
    /// [`SoundGroup::set_max_audible`](../../struct.SoundGroup.html#method.set_max_audible)
    /// setting, will be silent, then if another sound in the group stops the sound that was silent
    /// before becomes audible again.
    Mute,
    /// Any sound played that puts the sound count over the
    /// [`SoundGroup::set_max_audible`](../../struct.SoundGroup.html#method.set_max_audible)
    /// setting, will steal the quietest / least important sound playing in the group.
    StealLowest,
    /// Maximum number of open state types.
    Max,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// These definitions can be used for creating FMOD defined special effects or DSP units.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter).
pub enum DspType {
    /// This unit was created via a non FMOD plugin so has an unknown purpose.
    Unknown,
    /// This unit does nothing but take inputs and mix them together then feed the result to the
    /// soundcard unit.
    Mixer,
    /// This unit generates sine/square/saw/triangle or noise tones.
    Oscillator,
    /// This unit filters sound using a high quality, resonant lowpass filter algorithm but consumes
    /// more CPU time.
    LowPass,
    /// This unit filters sound using a resonant lowpass filter algorithm that is used in Impulse
    /// Tracker, but with limited Cutoff range (0 to 8060hz).
    ITLowPass,
    /// This unit filters sound using a resonant highpass filter algorithm.
    HighPass,
    /// This unit produces an echo on the sound and fades out at the desired rate.
    Echo,
    /// This unit produces a flange effect on the sound.
    Flange,
    /// This unit distorts the sound.
    Distortion,
    /// This unit normalizes or amplifies the sound to a certain level.
    Normalize,
    /// This unit attenuates or amplifies a selected frequency range.
    Parameq,
    /// This unit bends the pitch of a sound without changing the speed of playback.
    PitchShift,
    /// This unit produces a chorus effect on the sound.
    Chorus,
    /// This unit allows the use of Steinberg VST plugins
    VSTPlugin,
    /// This unit allows the use of Nullsoft Winamp plugins
    WinampPlugin,
    /// This unit produces an echo on the sound and fades out at the desired rate as is used in
    /// Impulse Tracker.
    ITEcho,
    /// This unit implements dynamic compression (linked multichannel, wideband)
    Compressor,
    /// This unit implements SFX reverb
    SFXReverb,
    /// This unit filters sound using a simple lowpass with no resonance, but has flexible Cutoff
    /// and is fast.
    LowPassSimple,
    /// This unit produces different delays on individual channels of the sound.
    Delay,
    /// This unit produces a tremolo / chopper effect on the sound.
    Tremolo,
    /// This unit allows the use of LADSPA standard plugins.
    LADSPAPlugin,
    /// This unit filters sound using a simple highpass with no resonance, but has flexible Cutoff
    /// and is fast.
    HighPassSimple,
    /// Offset that platform specific FMOD_HARDWARE DSPs will start at.
    Hardware = 1000,
    /// Makes sure this enum is signed 32bit.
    ForceInt = 65536,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_OSCILLATOR filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspOscillator {
    /// Waveform type. 0 = sine. 1 = square. 2 = sawup. 3 = sawdown. 4 = triangle. 5 = noise.
    Type,
    /// Frequency of the sinewave in hz. 1.0 to 22000.0. Default = 220.0.
    Rate,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_LOWPASS filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspLowPass {
    /// Lowpass Cutoff frequency in hz. 10.0 to 22000.0. Default = 5000.0.
    Cutoff,
    /// Lowpass resonance Q value. 1.0 to 10.0. Default = 1.0.
    Resonance,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_ITLOWPASS filter.
/// This is different to the default FMOD_DSP_TYPE_ITLOWPASS filter in that it uses a different
/// quality algorithm and is the filter used to produce the correct sounding playback in .IT files.
/// FMOD Ex's .IT playback uses this filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspITLowPass {
    /// Lowpass Cutoff frequency in hz. 1.0 to 22000.0. Default = 5000.0
    Cutoff,
    /// Lowpass resonance Q value. 0.0 to 127.0. Default = 1.0.
    Resonance,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_HIGHPASS filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter).
pub enum DspHighPass {
    /// Highpass Cutoff frequency in hz. 1.0 to output 22000.0. Default = 5000.0.
    Cutoff,
    /// Highpass resonance Q value. 1.0 to 10.0. Default = 1.0.
    Resonance,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the DspTypeEcho filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter).
pub enum DspTypeEcho {
    /// Echo delay in ms. 10 to 5000. Default = 500.
    Delay,
    /// Echo decay per delay. 0 to 1. 1.0 = No decay, 0.0 = total decay (ie simple 1 line delay).
    /// Default = 0.5.
    DecayRatio,
    /// Maximum channels supported. 0 to 16. 0 = same as fmod's default output polyphony, 1 = mono,
    /// 2 = stereo etc. See remarks for more. Default = 0. It is suggested to leave at 0!
    MaxChannels,
    /// Volume of original signal to pass to output. 0.0 to 1.0. Default = 1.0.
    DryMix,
    /// Volume of echo signal to pass to output. 0.0 to 1.0. Default = 1.0.
    WetMix,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_DELAY filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspDelay {
    /// Channel #0 Delay in ms. 0  to 10000. Default = 0.
    CH0,
    /// Channel #1 Delay in ms. 0  to 10000. Default = 0.
    CH1,
    /// Channel #2 Delay in ms. 0  to 10000. Default = 0.
    CH2,
    /// Channel #3 Delay in ms. 0  to 10000. Default = 0.
    CH3,
    /// Channel #4 Delay in ms. 0  to 10000. Default = 0.
    CH4,
    /// Channel #5 Delay in ms. 0  to 10000. Default = 0.
    CH5,
    /// Channel #6 Delay in ms. 0  to 10000. Default = 0.
    CH6,
    /// Channel #7 Delay in ms. 0  to 10000. Default = 0.
    CH7,
    /// Channel #8 Delay in ms. 0  to 10000. Default = 0.
    CH8,
    /// Channel #9 Delay in ms. 0  to 10000. Default = 0.
    CH9,
    /// Channel #10 Delay in ms. 0  to 10000. Default = 0.
    CH10,
    /// Channel #11 Delay in ms. 0  to 10000. Default = 0.
    CH11,
    /// Channel #12 Delay in ms. 0  to 10000. Default = 0.
    CH12,
    /// Channel #13 Delay in ms. 0  to 10000. Default = 0.
    CH13,
    /// Channel #14 Delay in ms. 0  to 10000. Default = 0.
    CH14,
    /// Channel #15 Delay in ms. 0  to 10000. Default = 0.
    CH15,
    /// Maximum delay in ms. 0  to 10000. Default = 10.
    MaxDelay,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_FLANGE filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspFlange {
    /// Volume of original signal to pass to output. 0.0 to 1.0. Default = 0.45.
    DryMix,
    /// Volume of flange signal to pass to output. 0.0 to 1.0. Default = 0.55.
    WetMix,
    /// Flange depth (percentage of 40ms delay). 0.01 to 1.0. Default = 1.0.
    Depth,
    /// Flange speed in hz. 0.0 to 20.0. Default = 0.1.
    Rate,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_TREMOLO filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspTremolo {
    /// LFO frequency in Hz. 0.1 to 20. Default = 4.
    Frequency,
    /// Tremolo depth. 0 to 1. Default = 0.
    Depth,
    /// LFO shape morph between triangle and sine. 0 to 1. Default = 0.
    Shape,
    /// Time-skewing of LFO cycle. -1 to 1. Default = 0.
    Skew,
    /// LFO on-time. 0 to 1. Default = 0.5.
    Duty,
    /// Flatness of the LFO shape. 0 to 1. Default = 0.
    Square,
    /// Instantaneous LFO phase. 0 to 1. Default = 0.
    Phase,
    /// Rotation / auto-pan effect. -1 to 1. Default = 0.
    Spread
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_DISTORTION filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspDistortion {
    /// Distortion value. 0.0 to 1.0. Default = 0.5.
    Level,
    /// Useless enum
    Unused,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_NORMALIZE filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspNormalize {
    /// Time to ramp the silence to full in ms. 0.0 to 20000.0. Default = 5000.0.
    FadeTime,
    /// Lower volume range threshold to ignore. 0.0 to 1.0. Default = 0.1. Raise higher to stop
    /// amplification of very quiet signals.
    Threshold,
    /// Maximum amplification allowed. 1.0 to 100000.0. Default = 20.0. 1.0 = no amplifaction,
    /// higher values allow more boost.
    MaxAmp,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the DspTypeParameq filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspTypeParameq {
    /// Frequency center. 20.0 to 22000.0. Default = 8000.0.
    Center,
    /// Octave range around the center frequency to filter. 0.2 to 5.0. Default = 1.0.
    Bandwidth,
    /// Frequency Gain. 0.05 to 3.0. Default = 1.0.
    Gain,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_PITCHSHIFT filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspPitchShift {
    /// Pitch value. 0.5 to 2.0. Default = 1.0. 0.5 = one octave down, 2.0 = one octave up. 1.0 does
    /// not change the pitch.
    Pitch,
    /// FFT window size. 256, 512, 1024, 2048, 4096. Default = 1024. Increase this to reduce
    /// 'smearing'. This effect is a warbling sound similar to when an mp3 is encoded at very low
    /// bitrates.
    FFTSize,
    /// Removed. Do not use. FMOD now uses 4 overlaps and cannot be changed.
    OverLap,
    /// Maximum channels supported. 0 to 16. 0 = same as fmod's default output polyphony, 1 = mono,
    /// 2 = stereo etc. See remarks for more. Default = 0. It is suggested to leave at 0!
    MaxChannels,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_CHORUS filter.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspChorus {
    /// Volume of original signal to pass to output. 0.0 to 1.0. Default = 0.5.
    DryMix,
    /// Volume of 1st chorus tap. 0.0 to 1.0. Default = 0.5.
    WetMix1,
    /// Volume of 2nd chorus tap. This tap is 90 degrees out of phase of the first tap. 0.0 to 1.0.
    /// Default = 0.5.
    WetMix2,
    /// Volume of 3rd chorus tap. This tap is 90 degrees out of phase of the second tap. 0.0 to 1.0.
    /// Default = 0.5.
    WetMix3,
    /// Chorus delay in ms. 0.1 to 100.0. Default = 40.0 ms.
    Delay,
    /// Chorus modulation rate in hz. 0.0 to 20.0. Default = 0.8 hz.
    Rate,
    /// Chorus modulation depth. 0.0 to 1.0. Default = 0.03.
    Depth,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_ITECHO filter.
/// This is effectively a software based echo filter that emulates the DirectX DMO echo effect.
/// Impulse tracker files can support this, and FMOD will produce the effect on ANY platform, not
/// just those that support DirectX effects!
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspITEcho {
    /// Ratio of wet (processed) signal to dry (unprocessed) signal. Must be in the range from 0.0
    /// through 100.0 (all wet). The default value is 50.
    WetDryMix,
    /// Percentage of output fed back into input, in the range from 0.0 through 100.0. The default
    /// value is 50.
    FeedBack,
    /// Delay for left channel, in milliseconds, in the range from 1.0 through 2000.0. The default
    /// value is 500 ms.
    LeftDelay,
    /// Delay for right channel, in milliseconds, in the range from 1.0 through 2000.0. The default
    /// value is 500 ms.
    RightDelay,
    /// Value that specifies whether to swap left and right delays with each successive echo. The
    /// default value is zero, meaning no swap. Possible values are defined as 0.0 (equivalent to
    /// FALSE) and 1.0 (equivalent to TRUE). CURRENTLY NOT SUPPORTED.
    PanDelay,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_COMPRESSOR unit.
/// This is a simple linked multichannel software limiter that is uniform across the whole spectrum.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter)
pub enum DspCompressor {
    /// Threshold level (dB) in the range from -60 through 0. The default value is 0.
    Threshold,
    /// Gain reduction attack time (milliseconds), in the range from 10 through 200. The default
    /// value is 50.
    Attack,
    /// Gain reduction release time (milliseconds), in the range from 20 through 1000. The default
    /// value is 50.
    Release,
    /// Make-up gain (dB) applied after limiting, in the range from 0 through 30. The default value
    /// is 0.
    GainMakeup,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_SFXREVERB unit.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter).
pub enum DspSfxReverb {
    /// Dry Level      : Mix level of dry signal in output in mB. Ranges from -10000.0 to 0.0.
    /// Default is 0.
    DryLevel,
    /// Room           : Room effect level at low frequencies in mB. Ranges from -10000.0 to 0.0.
    /// Default is -10000.0.
    Room,
    /// Room HF        : Room effect high-frequency level re. low frequency level in mB. Ranges from
    /// -10000.0 to 0.0. Default is 0.0.
    RoomHF,
    /// Decay Time     : Reverberation decay time at low-frequencies in seconds. Ranges from 0.1 to
    /// 20.0. Default is 1.0.
    DecayTime,
    /// Decay HF Ratio : High-frequency to low-frequency decay time ratio. Ranges from 0.1 to 2.0.
    /// Default is 0.5.
    DecayHFRatio,
    /// Reflections    : Early reflections level relative to room effect in mB. Ranges from -10000.0
    /// to 1000.0. Default is -10000.0.
    ReflectionsLevel,
    /// Reflect Delay  : Delay time of first reflection in seconds. Ranges from 0.0 to 0.3. Default
    /// is 0.02.
    ReflectionsDelay,
    /// Reverb         : Late reverberation level relative to room effect in mB. Ranges from
    /// -10000.0 to 2000.0. Default is 0.0.
    ReverbLevel,
    /// Reverb Delay   : Late reverberation delay time relative to first reflection in seconds.
    /// Ranges from 0.0 to 0.1. Default is 0.04.
    ReverbDelay,
    /// Diffusion      : Reverberation diffusion (echo density) in percent. Ranges from 0.0 to
    /// 100.0. Default is 100.0.
    Diffusion,
    /// Density        : Reverberation density (modal density) in percent. Ranges from 0.0 to 100.0.
    /// Default is 100.0.
    Density,
    /// HF Reference   : Reference high frequency in Hz. Ranges from 20.0 to 20000.0. Default is
    /// 5000.0.
    HFReference,
    /// Room LF        : Room effect low-frequency level in mB. Ranges from -10000.0 to 0.0. Default
    /// is 0.0.
    RoomLF,
    /// LF Reference   : Reference low-frequency in Hz. Ranges from 20.0 to 1000.0. Default is
    /// 250.0.
    LFReference,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_LOWPASS_SIMPLE filter.
/// This is a very simple low pass filter, based on two single-pole RC time-constant modules.
/// The emphasis is on speed rather than accuracy, so this should not be used for task requiring
/// critical filtering.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter).
pub enum DspLowPassSimple {
    /// Lowpass Cutoff frequency in hz. 10.0 to 22000.0. Default = 5000.0
    Cutoff,
    /// Useless enum
    Unused,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
#[repr(C)]
/// Parameter types for the FMOD_DSP_TYPE_HIGHPASS_SIMPLE filter.
/// This is a very simple single-order high pass filter.
/// The emphasis is on speed rather than accuracy, so this should not be used for task requiring
/// critical filtering.
/// Used with [`Dsp::set_parameter`](../struct.Dsp.html#method.set_parameter) and
/// [`Dsp::get_parameter`](../struct.Dsp.html#method.get_parameter).
pub enum DspHighPassSimple {
    /// Highpass cutoff frequency in hz. 10.0 to 22000.0. Default = 1000.0
    Cutoff,
    /// Useless enum
    Unused,
}
