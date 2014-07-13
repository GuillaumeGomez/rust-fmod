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

use ffi;
use types::*;
use enums::*;
use vector;

pub fn from_ptr(pointer: ffi::FMOD_REVERB_PROPERTIES) -> ReverbProperties {
    ReverbProperties {
        instance: pointer.Instance,
        environment: pointer.Environment,
        env_diffusion: pointer.EnvDiffusion,
        room: pointer.Room,
        room_HF: pointer.RoomHF,
        room_LF: pointer.RoomLF,
        decay_time: pointer.DecayTime,
        decay_HF_ratio: pointer.DecayHFRatio,
        decay_LF_ratio: pointer.DecayLFRatio,
        reflections: pointer.Reflections,
        reflections_delay: pointer.ReflectionsDelay,
        reverb: pointer.Reverb,
        reverb_delay: pointer.ReverbDelay,
        modulation_time: pointer.ModulationTime,
        modulation_depth: pointer.ModulationDepth,
        HF_reference: pointer.HFReference,
        LF_reference: pointer.LFReference,
        diffusion: pointer.Diffusion,
        density: pointer.Density,
        flags: pointer.Flags
    }
}

pub fn get_ffi(reverb: ReverbProperties) -> ffi::FMOD_REVERB_PROPERTIES {
    ffi::FMOD_REVERB_PROPERTIES {
        Instance: reverb.instance,
        Environment: reverb.environment,
        EnvDiffusion: reverb.env_diffusion,
        Room: reverb.room,
        RoomHF: reverb.room_HF,
        RoomLF: reverb.room_LF,
        DecayTime: reverb.decay_time,
        DecayHFRatio: reverb.decay_HF_ratio,
        DecayLFRatio: reverb.decay_LF_ratio,
        Reflections: reverb.reflections,
        ReflectionsDelay: reverb.reflections_delay,
        Reverb: reverb.reverb,
        ReverbDelay: reverb.reverb_delay,
        ModulationTime: reverb.modulation_time,
        ModulationDepth: reverb.modulation_depth,
        HFReference: reverb.HF_reference,
        LFReference: reverb.LF_reference,
        Diffusion: reverb.diffusion,
        Density: reverb.density,
        Flags: reverb.flags
    }
}

/// Structure defining a reverb environment.
pub struct ReverbProperties
{
    /// [w]   Min: 0 - Max: 3 - Default: 0 - Environment Instance.  (SUPPORTED:SFX(4 instances) and Wii (3 instances))
    pub instance         : i32,
    /// [r/w] Min: -1 - Max: 25 - Default: -1 - Sets all listener properties.  -1 = OFF.  (SUPPORTED:SFX(-1 only)/PSP)
    pub environment      : i32,
    /// [r/w] Min: 0.0 - Max: 1.0 - Default: 1.0 - Environment diffusion  (SUPPORTED:WII)
    pub env_diffusion    : f32,
    /// [r/w] Min: -10000 - Max: 0 - Default: -1000 - Room effect level (at mid frequencies)  (SUPPORTED:SFX/WII/PSP)
    pub room             : i32,
    /// [r/w] Min: -10000 - Max: 0 - Default: -100 - Relative room effect level at high frequencies  (SUPPORTED:SFX)
    pub room_HF          : i32,
    /// [r/w] Min: -10000 - Max: 0 Default: 0 - Relative room effect level at low frequencies  (SUPPORTED:SFX)
    pub room_LF          : i32,
    /// [r/w] Min: 0.1 - Max: 20.0 - Default: 1.49 - Reverberation decay time at mid frequencies  (SUPPORTED:SFX/WII)
    pub decay_time       : f32,
    /// [r/w] Min: 0.1 - Max: 2.0 - Default: 0.83 - High-frequency to mid-frequency decay time ratio  (SUPPORTED:SFX)
    pub decay_HF_ratio   : f32,
    /// [r/w] Min: 0.1 - Max: 2.0 - Default: 1.0 - Low-frequency to mid-frequency decay time ratio  (SUPPORTED:---)
    pub decay_LF_ratio   : f32,
    /// [r/w] Min: -10000 - Max: 1000 - Default: -2602 - Early reflections level relative to room effect  (SUPPORTED:SFX/WII)
    pub reflections      : i32,
    /// [r/w] Min: 0.0 - Max: 0.3 - Default: 0.007 - Initial reflection delay time  (SUPPORTED:SFX)
    pub reflections_delay: f32,
    /// [r/w] Min: -10000 - Max: 2000 - Default: 200 - Late reverberation level relative to room effect  (SUPPORTED:SFX)
    pub reverb           : i32,
    /// [r/w] Min: 0.0 - Max: 0.1 - Default: 0.011 - Late reverberation delay time relative to initial reflection  (SUPPORTED:SFX/WII)
    pub reverb_delay     : f32,
    /// [r/w] Min: 0.04 - Max: 4.0 - Default: 0.25 - Modulation time  (SUPPORTED:---)
    pub modulation_time  : f32,
    /// [r/w] Min: 0.0 - Max: 1.0 - Default: 0.0 - Modulation depth  (SUPPORTED:WII)
    pub modulation_depth : f32,
    /// [r/w] Min: 20.0 - Max: 20000.0 - Default: 5000.0 - Reference high frequency (hz)  (SUPPORTED:SFX)
    pub HF_reference     : f32,
    /// [r/w] Min: 20.0 - Max: 1000.0 - Default: 250.0 - Reference low frequency (hz)  (SUPPORTED:SFX)
    pub LF_reference     : f32,
    /// [r/w] Min: 0.0 - Max: 100.0 - Default: 100.0 - Value that controls the echo density in the late reverberation decay.  (SUPPORTED:SFX)
    pub diffusion        : f32,
    /// [r/w] Min: 0.0 - Max: 100.0 - Default: 100.0 - Value that controls the modal density in the late reverberation decay  (SUPPORTED:SFX)
    pub density          : f32,
    /// [r/w] FMOD_REVERB_FLAGS - modifies the behavior of above properties  (SUPPORTED:WII)
    pub flags            : u32
}

impl ReverbProperties {
    pub fn new() -> ReverbProperties {
        ReverbProperties {
            instance: 0i32,
            environment: -1i32,
            env_diffusion: 1.0f32,
            room: -1000i32,
            room_HF: -100i32,
            room_LF: 0i32,
            decay_time: 1.49f32,
            decay_HF_ratio: 0.83f32,
            decay_LF_ratio: 1f32,
            reflections: -2602i32,
            reflections_delay: 0.007f32,
            reverb: 200i32,
            reverb_delay: 0.011f32,
            modulation_time: 0.25f32,
            modulation_depth: 0f32,
            HF_reference: 5000f32,
            LF_reference: 250f32,
            diffusion: 100f32,
            density: 100f32,
            flags: 0u32
        }
    }
}