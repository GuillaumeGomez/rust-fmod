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

pub fn new() -> ReverbProperties {
    ReverbProperties {
        instance: 0i32,
        environment: 0i32,
        env_diffusion: 0f32,
        room: 0i32,
        room_HF: 0i32,
        room_LF: 0i32,
        decay_time: 0f32,
        decay_HF_ratio: 0f32,
        decay_LF_ratio: 0f32,
        reflections: 0i32,
        reflections_delay: 0f32,
        reverb: 0i32,
        reverb_delay: 0f32,
        modulation_time: 0f32,
        modulation_depth: 0f32,
        HF_reference: 0f32,
        LF_reference: 0f32,
        diffusion: 0f32,
        density: 0f32,
        flags: 0u32
    }
}

pub struct ReverbProperties
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
    pub flags            : u32      /* [r/w] *mut FMOD_REVERB_FLAGS - modifies the behavior of above properties                                (SUPPORTED:WII) */
}