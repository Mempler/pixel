#![feature(new_uninit)]

mod audio;
mod audio_system;

#[cfg(feature = "audio_fmod")]
mod fmod_sys;

pub use audio::Audio;
pub use audio_system::AudioSystem;
