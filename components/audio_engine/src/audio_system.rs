use std::path::Path;
use crate::Audio;
#[cfg(feature = "audio_fmod")]
use rfmod::Channel;

pub struct AudioSystem {
    #[cfg(feature = "audio_fmod")]
    sys: rfmod::Sys
}

impl AudioSystem {
    pub fn new() -> AudioSystem {
        #[cfg(feature = "audio_fmod")]
        let sys;

        #[cfg(feature = "audio_fmod")]
            {
                sys = rfmod::Sys::new().unwrap();
                sys.init();
            }

        let system_id;
        #[cfg(feature = "audio_fmod")]
            {
                system_id = "fmod"
            }

        #[cfg(feature = "audio_none")]
            {
                system_id = "none"
            }

        if system_id == "none" {
            log::warn!("Audio system is not set!\n");
        } else {
            log::info!("Initialized AudioSystem with {}\n", system_id);
        }

        AudioSystem {
            #[cfg(feature = "audio_fmod")]
            sys
        }
    }

    pub fn from_file<P: AsRef<Path>>(&self, path: P) -> Audio {
        let path = path.as_ref();
        #[cfg(feature = "audio_fmod")]
        let audio;

        #[cfg(feature = "audio_fmod")]
        {
            audio = self.sys.create_sound(path.to_str().unwrap(), None, None)
                .unwrap();
        }

        Audio {
            #[cfg(feature = "audio_fmod")]
            audio,

            #[cfg(feature = "audio_fmod")]
            channel: Channel::new(),

            #[cfg(feature = "audio_fmod")]
            default_frequency: 0.0,
        }
    }

    // TODO: implement
    #[allow(dead_code, unused_variables)]
    pub fn from_memory(&self, buf: &[u8]) -> Audio {
        #[cfg(feature = "audio_fmod")]
        unimplemented!("Not implemented for FMOD!");

        unimplemented!();
    }
}

#[test]
fn play_pause_stop_audio_test() {
    use std::thread::sleep;
    use std::time::Duration;

    let sys = AudioSystem::new();
    let mut audio = sys.from_file("test/music/Jonth - Collapse [NCS Release].mp3");

    audio.play();
    sleep(Duration::from_secs(5));
    audio.pause();
    sleep(Duration::from_secs(5));
    audio.pause();
    sleep(Duration::from_secs(5));
    audio.stop();
}

#[test]
fn play_frequency_audio_test() {
    use std::thread::sleep;
    use std::time::Duration;

    let sys = AudioSystem::new();
    let mut audio = sys.from_file("test/music/Jonth - Collapse [NCS Release].mp3");

    audio.play();
    //sleep(Duration::from_secs(5));
    audio.set_frequency(1.25);
    sleep(Duration::from_secs(20));
    audio.stop();
}
