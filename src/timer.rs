use log::info;
use std::time::Instant;

#[derive(Clone)]
pub struct Timer {
    pub start: Instant,
}

impl Timer {
    #[must_use]
    pub fn new() -> Self {
        Self { start: Instant::now() }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn report(&self, note: &str) {
        println!("{:>12?}  >  {note}", self.start.elapsed());
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn bench(function: impl Fn(usize), label: &str, iterations: usize) {
    let t = Timer::new();
    (0..iterations).for_each(|i| {
        function(i);
    });
    t.report(label);
}
