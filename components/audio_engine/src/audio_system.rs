use std::path::Path;
use crate::Audio;

#[cfg(feature = "audio_fmod")]
use crate::fmod_sys::*;
use std::ptr::{null, null_mut};
use std::ffi::c_void;

pub struct AudioSystem {
    #[cfg(feature = "audio_fmod")]
    sys: *mut FMOD_SYSTEM
}

impl AudioSystem {
    pub fn new() -> AudioSystem {
        #[cfg(feature = "audio_fmod")]
        let sys;

        let system_id;
        #[cfg(feature = "audio_fmod")]
            unsafe {
                let mut tmp = null_mut();
                if FMOD_System_Create(&mut tmp) != FMOD_RESULT_FMOD_OK {
                    panic!("Failed to create AudioSystem using FMOD!");
                }

                sys = tmp;

                if FMOD_System_Init(sys, 2048,
                                    FMOD_INIT_NORMAL, null_mut::<c_void>()) != FMOD_RESULT_FMOD_OK {
                    panic!("Failed to initialize AudioSystem using FMOD!");
                }

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

        let file = std::fs::read(path).unwrap();

        self.from_memory(file)
    }

    // TODO: implement
    #[allow(dead_code, unused_variables)]
    pub fn from_memory(&self, buf: Vec<u8>) -> Audio {
        #[cfg(feature = "audio_fmod")]
        let mut audio_ptr = std::ptr::null_mut();

        #[cfg(feature = "audio_fmod")]
        unsafe {
            let sound_info = Box::<FMOD_CREATESOUNDEXINFO>::new_zeroed();
            let mut sound_info = { sound_info.assume_init() };

            sound_info.cbsize = 224;
            sound_info.length = buf.len() as u32;

            let r = FMOD_System_CreateSound(self.sys, buf.as_ptr() as *const i8,
                                            FMOD_OPENMEMORY,
                                            sound_info.as_mut(),
                                            &mut audio_ptr);
            if r != FMOD_RESULT_FMOD_OK {
                panic!("Failed to load Audio {}", r);
            }
        }

        let mut audio = Audio {
            #[cfg(feature = "audio_fmod")]
            audio: audio_ptr,

            #[cfg(feature = "audio_fmod")]
            sys: self.sys,

            #[cfg(feature = "audio_fmod")]
            channel: std::ptr::null_mut(),

            #[cfg(feature = "audio_fmod")]
            audio_data: buf,

            #[cfg(feature = "audio_fmod")]
            default_frequency: 0.0
        };

        audio.init();

        audio
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        unsafe {
            FMOD_System_Close(self.sys);
        }
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
    audio.play(); // Unpause
    sleep(Duration::from_secs(5));
    audio.stop();
    sleep(Duration::from_secs(5));
    audio.play();
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
    audio.set_frequency_mul(1.25);
    sleep(Duration::from_secs(20));
    audio.stop();
}
