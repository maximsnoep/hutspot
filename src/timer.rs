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
        println!("{:>12?}  >  {note}", self.start.elapsed());
    }
}

pub fn bench(function: impl Fn(), label: &str, iterations: usize) {
    let t = Timer::new();
    (0..iterations).into_iter().for_each(|_| {
        function();
    });
    t.report(label);
}
