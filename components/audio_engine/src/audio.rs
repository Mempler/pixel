pub struct Audio {

}

impl Audio {
    pub fn play(&mut self) {

    }

    pub fn stop(&mut self) {

    }

    pub fn pause(&mut self) {

    }

    pub fn set_volume(&mut self, volume: f32) {

    }

    pub fn set_frequency(&mut self, frequency: f32) {

    }

    pub fn set_speed(&mut self, speed: f32) {

    }

    pub fn seek(&mut self, pos: usize) {

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
        return 0;
    }
}
