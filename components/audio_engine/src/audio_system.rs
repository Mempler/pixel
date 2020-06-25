use std::path::Path;
use crate::Audio;

pub struct AudioSystem {

}

impl AudioSystem {
    pub fn new() -> AudioSystem {
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

        }
    }

    pub fn from_file<P: AsRef<Path>>(&self, path: P) -> Audio {
        let path = path.as_ref();

        Audio {

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
