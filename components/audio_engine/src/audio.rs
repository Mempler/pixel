#[cfg(feature = "audio_fmod")]
use rfmod::{Sound, Channel, TIMEUNIT_MS};

pub struct Audio {
    #[cfg(feature = "audio_fmod")]
    pub(crate) audio: Sound,

    #[cfg(feature = "audio_fmod")]
    pub(crate) channel: Channel,

    #[cfg(feature = "audio_fmod")]
    pub(crate) default_frequency: f32
}

impl Audio {
    pub fn play(&mut self) {
        #[cfg(feature = "audio_fmod")]
        {
            self.channel = self.audio.play().unwrap();
            self.default_frequency = self.channel.get_frequency().unwrap();
        }
    }

    pub fn stop(&mut self) {
        #[cfg(feature = "audio_fmod")]
        self.channel.stop();
    }

    pub fn pause(&mut self) {
        #[cfg(feature = "audio_fmod")]
        self.channel.set_paused(!self.channel.get_paused().unwrap());
    }

    pub fn set_volume(&mut self, volume: f32) {
        #[cfg(feature = "audio_fmod")]
        self.channel.set_volume(volume);
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        #[cfg(feature = "audio_fmod")]
        self.channel.set_frequency(frequency * self.default_frequency);
    }

    pub fn set_speed(&mut self, speed: f32) {
        #[cfg(feature = "audio_fmod")]
        {
            self.channel.set_frequency(speed * self.default_frequency);
            let channel_group = self.channel.get_channel_group().unwrap();
            channel_group.set_pitch(1.0-speed);
        }
    }

    pub fn seek(&mut self, pos: usize) {
        #[cfg(feature = "audio_fmod")]
        self.channel.set_position(pos, TIMEUNIT_MS);
    }

    pub fn reset(&mut self) {
        self.stop();
        self.set_speed(1.0);
        self.set_frequency(1.0);
        self.seek(0);
    }

    pub fn reset_speed(&mut self) {
        self.set_speed(1.0);
        self.set_frequency(1.0);
    }

    pub fn len(&self) -> u32 {
        #[cfg(feature = "audio_fmod")]
        self.audio.get_length(TIMEUNIT_MS).unwrap()
    }
}
