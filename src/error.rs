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

use enums;

pub fn error_string(errcode: enums::Result) -> &'static str {
    match errcode {
        enums::AlreadyLocked => "Tried to call lock a second time before unlock was called.",
        enums::BadCommand => "Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound::lock on a streaming sound).",
        enums::CDDADrivers => "Neither NTSCSI nor ASPI could be initialised.",
        enums::CDDAInit => "An error occurred while initialising the CDDA subsystem.",
        enums::CDDAInvalidDevice => "Couldn't find the specified device.",
        enums::CDDANoAudio => "No audio tracks on the specified disc.",
        enums::CDDANoDevices => "No CD/DVD devices were found.",
        enums::CDDANoDisc => "No disc present in the specified drive.",
        enums::CDDARead => "A CDDA read error occurred.",
        enums::ChannelAlloc => "Error trying to allocate a channel.",
        enums::ChannelStolen => "The specified channel has been reused to play another sound.",
        enums::COM => "A Win32 COM related error occured. COM failed to initialize or a QueryInterface failed meaning a Windows codec or driver was not installed properly.",
        enums::DMA => "DMA Failure.  See debug output for more information.",
        enums::DSPConnection => "DSP connection error.  Connection possibly caused a cyclic dependancy.  Or tried to connect a tree too many units deep (more than 128).",
        enums::DSPFormat => "DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format.",
        enums::DSPNotFound => "DSP connection error.  Couldn't find the DSP unit specified.",
        enums::DSPRunning => "DSP error.  Cannot perform this operation while the network is in the middle of running.  This will most likely happen if a connection or disconnection is attempted in a DSP callback.",
        enums::DSPTooManyConnections => "DSP connection error.  The unit being connected to or disconnected should only have 1 input or output.",
        enums::EventAlreadyLoaded => "The specified project or bank has already been loaded. Having multiple copies of the same project loaded simultaneously is forbidden.",
        enums::EventFailed => "An Event failed to be retrieved, most likely due to 'just fail' being specified as the max playbacks behavior.",
        enums::EventGuidConflict => "An event with the same GUID already exists.",
        enums::EventInfoOnly => "An event with the same GUID already exists.",
        enums::EventInternal => "An error occured that wasn't supposed to.  See debug log for reason.",
        enums::EventMaxStreams => "Event failed because 'Max streams' was hit when FMOD_EVENT_INIT_FAIL_ON_MAXSTREAMS was specified.",
        enums::EventMismatch => "FSB mismatches the FEV it was compiled with, the stream/sample mode it was meant to be created with was different, or the FEV was built for a different platform.",
        enums::EventNameConflict => "A category with the same name already exists.",
        enums::EventNeedsSimple => "Tried to call a function on a complex event that's only supported by simple events.",
        enums::EventNotFound => "The requested event, event group, event category or event property could not be found.",
        enums::FileBad => "Error loading file.",
        enums::FileCouldNotSeek => "Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format.",
        enums::FileDiskEjected => "Media was ejected while reading.",
        enums::FileEOF => "End of file unexpectedly reached while trying to read essential data (truncated data?).",
        enums::FileNotFound => "File not found.",
        enums::FileUnwanted => "Unwanted file access occured.",
        enums::Format => "Unsupported file or audio format.",
        enums::HTTP => "A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.",
        enums::HTTPAccess => "The specified resource requires authentication or is forbidden.",
        enums::HTTPProxyAuth => "Proxy authentication is required to access the specified resource.",
        enums::HTTPServerError => "A HTTP server error occurred.",
        enums::HTTPTimeout => "The HTTP request timed out.",
        enums::Initialization => "FMOD was not initialized correctly to support this function.",
        enums::Initialized => "Cannot call this command after System::init.",
        enums::Internal => "An error occured that wasn't supposed to.  Contact support.",
        enums::InvalidAddress => "On Xbox 360, this memory address passed to FMOD must be physical, (ie allocated with XPhysicalAlloc.)",
        enums::InvalidFloat => "Value passed in was a NaN, Inf or denormalized float.",
        enums::InvalidHandle => "An invalid object handle was used.",
        enums::InvalidParam => "An invalid parameter was passed to this function.",
        enums::InvalidPosition => "An invalid seek position was passed to this function.",
        enums::InvalidSpeaker => "An invalid speaker was passed to this function based on the current speaker mode.",
        enums::InvalidSyncPoint => "The syncpoint did not come from this sound handle.",
        enums::InvalidVector => "The vectors passed in are not unit length, or perpendicular.",
        enums::MaxAudible => "Reached maximum audible playback count for this sound's soundgroup.",
        enums::Memory => "Not enough memory or resources.",
        enums::MemoryCantPoint => "Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used.",
        enums::MemorySRAM => "Not enough memory or resources on console sound ram.",
        enums::MusicNoCallback => "The music callback is required, but it has not been set.",
        enums::MusicNotFound => "The requested music entity could not be found.",
        enums::MusicUninitialized => "Music system is not initialized probably because no music data is loaded.",
        enums::Needs2D => "Tried to call a command on a 3d sound when the command was meant for 2d sound.",
        enums::Needs3D => "Tried to call a command on a 2d sound when the command was meant for 3d sound.",
        enums::NeedsHardware => "Tried to use a feature that requires hardware support.  (ie trying to play a GCADPCM compressed sound in software on Wii).",
        enums::NeedsSoftware => "Tried to use a feature that requires the software engine.  Software engine has either been turned off, or command was executed on a hardware channel which does not support this feature.",
        enums::NetConnect => "Couldn't connect to the specified host.",
        enums::NetSocketError => "A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere.",
        enums::NetURL => "The specified URL couldn't be resolved.",
        enums::NetWouldBlock => "Operation on a non-blocking socket could not complete immediately.",
        enums::NotReady => "Operation could not be performed because specified sound/DSP connection is not ready.",
        enums::OutputAllocated => "Error initializing output device, but more specifically, the output device is already in use and cannot be reused.",
        enums::OutputCreateBuffer => "Error creating hardware sound buffer.",
        enums::OutputDriverCall => "A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted.",
        enums::OutputEnumeration => "Error enumerating the available driver list. List may be inconsistent due to a recent device addition or removal.",
        enums::OutputFormat => "Soundcard does not support the minimum features needed for this soundsystem (16bit stereo output).",
        enums::OutputInit => "Error initializing output device.",
        enums::OutputNoHardware => "FMOD_HARDWARE was specified but the sound card does not have the resources necessary to play it.",
        enums::OutputNoSoftware => "Attempted to create a software sound but no software channels were specified in System::init.",
        enums::Pan => "Panning only works with mono or stereo sound sources.",
        enums::Plugin => "An unspecified error has been returned from a 3rd party plugin.",
        enums::PluginInstances => "The number of allowed instances of a plugin has been exceeded.",
        enums::PluginMissing => "A requested output, dsp unit type or codec was not available.",
        enums::PluginResource => "A resource that the plugin requires cannot be found. (ie the DLS file for MIDI playback or other DLLs that it needs to load)",
        enums::Preloaded => "The specified sound is still in use by the event system, call EventSystem::unloadFSB before trying to release it.",
        enums::ProgrammerSound => "The specified sound is still in use by the event system, wait for the event which is using it finish with it.",
        enums::Record => "An error occured trying to initialize the recording device.",
        enums::ReverbInstance => "Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesnt exist.",
        enums::Subsounds => "The error occured because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound, or a parent sound was played without setting up a sentence first.",
        enums::SubsoundAllocated => "This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first.",
        enums::SubsoundCantMove => "Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file.",
        enums::SubsoundMode => "The subsound's mode bits do not match with the parent sound's mode bits.  See documentation for function that it was called with.",
        enums::TagNotFound => "The specified tag could not be found or there are no tags.",
        enums::TooManyChannels => "The sound created exceeds the allowable input channel count.  This can be increased using the maxinputchannels parameter in System::setSoftwareFormat.",
        enums::Unimplemented => "Something in FMOD hasn't been implemented when it should be! contact support!",
        enums::Uninitialized => "This command failed because System::init or System::setDriver was not called.",
        enums::Unsupported => "A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified.",
        enums::Update => "An error caused by System::update occured.",
        enums::Version => "The version number of this file format is not supported.",
        enums::Ok => "No errors.",
        _ => "Unknown error."
    }
}