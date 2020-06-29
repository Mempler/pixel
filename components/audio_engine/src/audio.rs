#[cfg(feature = "audio_fmod")]
use crate::fmod_sys::*;
use std::ptr::{null, null_mut};

#[allow(dead_code)]
pub struct Audio {
    #[cfg(feature = "audio_fmod")]
    pub(crate) audio: *mut FMOD_SOUND,

    #[cfg(feature = "audio_fmod")]
    pub(crate) sys: *mut FMOD_SYSTEM,

    #[cfg(feature = "audio_fmod")]
    pub(crate) channel: *mut FMOD_CHANNEL,

    #[cfg(feature = "audio_fmod")]
    pub(crate) audio_data: Vec<u8>, // To keep it alive

    #[cfg(feature = "audio_fmod")]
    pub(crate) default_frequency: f32
}

impl Audio {
    pub(crate) fn init(&mut self) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            FMOD_System_PlaySound(
                self.sys,
                self.audio,
                null::<FMOD_CHANNELGROUP>() as _,
                1,
                &mut self.channel);

            if !self.channel.is_null() {
                FMOD_Channel_GetFrequency(self.channel, &mut self.default_frequency);
            }
        }
    }

    pub fn play(&mut self) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            if self.channel.is_null() {
                FMOD_System_PlaySound(
                    self.sys,
                    self.audio,
                    null::<FMOD_CHANNELGROUP>() as _,
                    1,
                    &mut self.channel);
            }

            FMOD_Channel_SetPaused(self.channel, 0);
        }
    }

    pub fn stop(&mut self) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            FMOD_Channel_Stop(self.channel);

            self.channel = null_mut();
        }
    }

    pub fn pause(&mut self) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            if self.channel.is_null() {
                self.play();
                return;
            }

            let mut should_pause = 0;
            FMOD_Channel_GetPaused(self.channel, &mut should_pause);

            if should_pause > 0 {
                should_pause = 0;
            } else {
                should_pause = 1;
            }

            FMOD_Channel_SetPaused(self.channel, should_pause);
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            if self.channel.is_null() {
                return; // do nothing
            }

            FMOD_Channel_SetVolume(self.channel, volume);
        }
    }

    pub fn set_frequency_mul(&mut self, mul: f32) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            if self.channel.is_null() {
                return; // do nothing
            }

            FMOD_Channel_SetFrequency(self.channel, self.default_frequency * mul);
        }
    }

    pub fn seek(&mut self, pos: usize) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            if self.channel.is_null() {
                return; // do nothing
            }

            FMOD_Channel_SetPosition(self.channel, pos as u32, FMOD_TIMEUNIT_MS);
        }
    }

    pub fn reset(&mut self) {
        self.stop();
        self.set_frequency_mul(1.0);
        self.seek(0);
    }

    pub fn reset_speed(&mut self) {
        self.set_frequency_mul(1.0);
    }

    pub fn len(&self) -> u32 {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            let len = null_mut();

            FMOD_Sound_GetLength(self.audio, len, FMOD_TIMEUNIT_MS);

            return *len;
        }

        #[cfg(not(feature = "audio_fmod"))]
        return 0;
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        #[cfg(feature = "audio_fmod")]
        unsafe {
            FMOD_Sound_Release(self.audio);
        }
    }
}
