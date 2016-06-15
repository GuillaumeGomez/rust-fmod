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

pub fn error_string(errcode: ::Result) -> &'static str {
    match errcode {
        ::Result::AlreadyLocked => "Tried to call lock a second time before unlock was called.",
        ::Result::BadCommand => "Tried to call a function on a data type that does not allow this \
                                 type of functionality (ie calling Sound::lock on a streaming \
                                 sound).",
        ::Result::CDDADrivers => "Neither NTSCSI nor ASPI could be initialised.",
        ::Result::CDDAInit => "An error occurred while initialising the CDDA subsystem.",
        ::Result::CDDAInvalidDevice => "Couldn't find the specified device.",
        ::Result::CDDANoAudio => "No audio tracks on the specified disc.",
        ::Result::CDDANoDevices => "No CD/DVD devices were found.",
        ::Result::CDDANoDisc => "No disc present in the specified drive.",
        ::Result::CDDARead => "A CDDA read error occurred.",
        ::Result::ChannelAlloc => "Error trying to allocate a channel.",
        ::Result::ChannelStolen => "The specified channel has been reused to play another sound.",
        ::Result::COM => "A Win32 COM related error occured. COM failed to initialize or a \
                          QueryInterface failed meaning a Windows codec or driver was not \
                          installed properly.",
        ::Result::DMA => "DMA Failure.  See debug output for more information.",
        ::Result::DSPConnection => "DSP connection error. Connection possibly caused a cyclic \
                                    dependency. Or tried to connect a tree too many units deep \
                                    (more than 128).",
        ::Result::DSPFormat => "DSP Format error.  A DSP unit may have attempted to connect to \
                                this network with the wrong format.",
        ::Result::DSPNotFound => "DSP connection error.  Couldn't find the DSP unit specified.",
        ::Result::DSPRunning => "DSP error.  Cannot perform this operation while the network is in \
                                 the middle of running. This will most likely happen if a \
                                 connection or disconnection is attempted in a DSP callback.",
        ::Result::DSPTooManyConnections => "DSP connection error. The unit being connected to or \
                                            disconnected should only have 1 input or output.",
        ::Result::EventAlreadyLoaded => "The specified project or bank has already been loaded. \
                                         Having multiple copies of the same project loaded \
                                         simultaneously is forbidden.",
        ::Result::EventFailed => "An Event failed to be retrieved, most likely due to 'just fail' \
                                  being specified as the max playbacks behavior.",
        ::Result::EventGuidConflict => "An event with the same GUID already exists.",
        ::Result::EventInfoOnly => "An event with the same GUID already exists.",
        ::Result::EventInternal => "An error occured that wasn't supposed to.  See debug log for \
                                    reason.",
        ::Result::EventMaxStreams => "Event failed because 'Max streams' was hit when \
                                      FMOD_EVENT_INIT_FAIL_ON_MAXSTREAMS was specified.",
        ::Result::EventMismatch => "FSB mismatches the FEV it was compiled with, the stream/sample \
                                    mode it was meant to be created with was different, or the FEV \
                                    was built for a different platform.",
        ::Result::EventNameConflict => "A category with the same name already exists.",
        ::Result::EventNeedsSimple => "Tried to call a function on a complex event that's only \
                                       supported by simple events.",
        ::Result::EventNotFound => "The requested event, event group, event category or event \
                                    property could not be found.",
        ::Result::FileBad => "Error loading file.",
        ::Result::FileCouldNotSeek => "Couldn't perform seek operation.  This is a limitation of \
                                       the medium (ie netstreams) or the file format.",
        ::Result::FileDiskEjected => "Media was ejected while reading.",
        ::Result::FileEOF => "End of file unexpectedly reached while trying to read essential data \
                              (truncated data?).",
        ::Result::FileNotFound => "File not found.",
        ::Result::FileUnwanted => "Unwanted file access occured.",
        ::Result::Format => "Unsupported file or audio format.",
        ::Result::HTTP => "A HTTP error occurred. This is a catch-all for HTTP errors not listed \
                           elsewhere.",
        ::Result::HTTPAccess => "The specified resource requires authentication or is forbidden.",
        ::Result::HTTPProxyAuth => "Proxy authentication is required to access the specified \
                                    resource.",
        ::Result::HTTPServerError => "A HTTP server error occurred.",
        ::Result::HTTPTimeout => "The HTTP request timed out.",
        ::Result::Initialization => "FMOD was not initialized correctly to support this function.",
        ::Result::Initialized => "Cannot call this command after System::init.",
        ::Result::Internal => "An error occured that wasn't supposed to. Contact support.",
        ::Result::InvalidAddress => "On Xbox 360, this memory address passed to FMOD must be \
                                     physical, (ie allocated with XPhysicalAlloc.)",
        ::Result::InvalidFloat => "Value passed in was a NaN, Inf or denormalized float.",
        ::Result::InvalidHandle => "An invalid object handle was used.",
        ::Result::InvalidParam => "An invalid parameter was passed to this function.",
        ::Result::InvalidPosition => "An invalid seek position was passed to this function.",
        ::Result::InvalidSpeaker => "An invalid speaker was passed to this function based on the \
                                     current speaker mode.",
        ::Result::InvalidSyncPoint => "The syncpoint did not come from this sound handle.",
        ::Result::InvalidVector => "The vectors passed in are not unit length, or perpendicular.",
        ::Result::MaxAudible => "Reached maximum audible playback count for this sound's \
                                 soundgroup.",
        ::Result::Memory => "Not enough memory or resources.",
        ::Result::MemoryCantPoint => "Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or \
                                      non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was \
                                      used.",
        ::Result::MemorySRAM => "Not enough memory or resources on console sound ram.",
        ::Result::MusicNoCallback => "The music callback is required, but it has not been set.",
        ::Result::MusicNotFound => "The requested music entity could not be found.",
        ::Result::MusicUninitialized => "Music system is not initialized probably because no music \
                                         data is loaded.",
        ::Result::Needs2D => "Tried to call a command on a 3d sound when the command was meant for \
                              2d sound.",
        ::Result::Needs3D => "Tried to call a command on a 2d sound when the command was meant for \
                              3d sound.",
        ::Result::NeedsHardware => "Tried to use a feature that requires hardware support.  (ie \
                                    trying to play a GCADPCM compressed sound in software on Wii).",
        ::Result::NeedsSoftware => "Tried to use a feature that requires the software engine. \
                                    Software engine has either been turned off, or command was \
                                    executed on a hardware channel which does not support this \
                                    feature.",
        ::Result::NetConnect => "Couldn't connect to the specified host.",
        ::Result::NetSocketError => "A socket error occurred.  This is a catch-all for \
                                     socket-related errors not listed elsewhere.",
        ::Result::NetURL => "The specified URL couldn't be resolved.",
        ::Result::NetWouldBlock => "Operation on a non-blocking socket could not complete \
                                    immediately.",
        ::Result::NotReady => "Operation could not be performed because specified sound/DSP \
                               connection is not ready.",
        ::Result::OutputAllocated => "Error initializing output device, but more specifically, the \
                                      output device is already in use and cannot be reused.",
        ::Result::OutputCreateBuffer => "Error creating hardware sound buffer.",
        ::Result::OutputDriverCall => "A call to a standard soundcard driver failed, which could \
                                       possibly mean a bug in the driver or resources were missing \
                                       or exhausted.",
        ::Result::OutputEnumeration => "Error enumerating the available driver list. List may be \
                                        inconsistent due to a recent device addition or removal.",
        ::Result::OutputFormat => "Soundcard does not support the minimum features needed for this \
                                   soundsystem (16bit stereo output).",
        ::Result::OutputInit => "Error initializing output device.",
        ::Result::OutputNoHardware => "FMOD_HARDWARE was specified but the sound card does not \
                                       have the resources necessary to play it.",
        ::Result::OutputNoSoftware => "Attempted to create a software sound but no software \
                                       channels were specified in System::init.",
        ::Result::Pan => "Panning only works with mono or stereo sound sources.",
        ::Result::Plugin => "An unspecified error has been returned from a 3rd party plugin.",
        ::Result::PluginInstances => "The number of allowed instances of a plugin has been \
                                      exceeded.",
        ::Result::PluginMissing => "A requested output, dsp unit type or codec was not available.",
        ::Result::PluginResource => "A resource that the plugin requires cannot be found. (ie the \
                                     DLS file for MIDI playback or other DLLs that it needs to \
                                     load)",
        ::Result::Preloaded => "The specified sound is still in use by the event system, call \
                                EventSystem::unloadFSB before trying to release it.",
        ::Result::ProgrammerSound => "The specified sound is still in use by the event system, \
                                      wait for the event which is using it finish with it.",
        ::Result::Record => "An error occured trying to initialize the recording device.",
        ::Result::ReverbInstance => "Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. \
                                     Most likely because it is an invalid instance number or the \
                                     reverb doesnt exist.",
        ::Result::Subsounds => "The error occured because the sound referenced contains subsounds \
                                when it shouldn't have, or it doesn't contain subsounds when it \
                                should have. The operation may also not be able to be performed on \
                                a parent sound, or a parent sound was played without setting up a \
                                sentence first.",
        ::Result::SubsoundAllocated => "This subsound is already being used by another sound, you \
                                        cannot have more than one parent to a sound. Null out the \
                                        other parent's entry first.",
        ::Result::SubsoundCantMove => "Shared subsounds cannot be replaced or moved from their \
                                       parent stream, such as when the parent stream is an FSB \
                                       file.",
        ::Result::SubsoundMode => "The subsound's mode bits do not match with the parent sound's \
                                   mode bits. See documentation for function that it was called \
                                   with.",
        ::Result::TagNotFound => "The specified tag could not be found or there are no tags.",
        ::Result::TooManyChannels => "The sound created exceeds the allowable input channel count. \
                                      This can be increased using the maxinputchannels parameter \
                                      in System::setSoftwareFormat.",
        ::Result::Unimplemented => "Something in FMOD hasn't been implemented when it should be! \
                                    Contact support!",
        ::Result::Uninitialized => "This command failed because System::init or System::setDriver \
                                    was not called.",
        ::Result::Unsupported => "A command issued was not supported by this object.  Possibly a \
                                  plugin without certain callbacks specified.",
        ::Result::Update => "An error caused by System::update occured.",
        ::Result::Version => "The version number of this file format is not supported.",
        ::Result::Ok => "No errors.",
        _ => "Unknown error."
    }
}
