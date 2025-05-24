pub mod input;



use std::time::Instant;



pub struct FrameCounter {
    begin: Instant, // when was this started.
    frame_count: u64,
    last_frame_time: Instant,
    last_update_time: Instant,
    last_update_val: f64,
}

impl FrameCounter {
    pub fn new() -> Self {
        FrameCounter {
            begin: Instant::now(),
            frame_count: 0,
            last_frame_time: Instant::now(),
            last_update_time: Instant::now(),
            last_update_val: 0.0f64,
        }
    }

    pub fn increment(&mut self) -> f32 {
        let delta = self.last_frame_time.elapsed().as_secs_f32();
        self.frame_count += 1;
        self.last_frame_time = Instant::now();
        delta
    }

    pub fn fps(&mut self) -> Option<f64> {
        if self.last_update_time.elapsed().as_secs() > 2 {
            self.last_update_val = self.frame_count as f64/ self.begin.elapsed().as_secs() as f64;
            self.last_update_time = Instant::now();
            return Some(self.last_update_val);
        }
        None
    }
}
