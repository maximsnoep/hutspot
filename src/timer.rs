use std::time::Instant;

#[derive(Clone)]
pub struct Timer {
    pub start: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn report(&self, note: &str) {
        log::info!("{:>12?}  >  {note}", self.start.elapsed());
    }
}
