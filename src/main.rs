use std::{
    io::prelude::*,
    sync::{Arc, Mutex},
};

struct Counter {
    start: std::time::Instant,
    count: usize,
    time_span: std::time::Duration,
}

impl Counter {
    fn new(time_span: std::time::Duration) -> Self {
        Self {
            start: std::time::Instant::now(),
            count: 0,
            time_span,
        }
    }

    fn increase(&mut self) {
        self.count += 1;
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        if now > (self.start + self.time_span) {
            self.print();
            self.reset();
            self.count = 0;
        }
    }

    fn print(&self) {
        let time_span_s = self.time_span.as_secs() as f32;
        let count = self.count as f32;
        let freq = count / time_span_s;
        println!(
            "count: {}, time_span: {}s, frequency: {}",
            self.count, time_span_s, freq
        );
    }

    fn reset(&mut self) {
        self.count = 0;
        self.start = std::time::Instant::now();
    }
}

fn start_counter(counter: Counter) -> Arc<Mutex<Counter>> {
    let counter = Arc::new(Mutex::new(counter));

    let counter_copy = counter.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        {
            let mut counter = counter_copy.lock().unwrap();
            counter.update();
        }
    });

    counter
}

fn main() {
    let time_span: f32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);

    let time_span = std::time::Duration::from_millis((time_span * 1000.0) as u64);

    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    let counter = Counter::new(time_span);
    let counter = start_counter(counter);

    stdin.lines().map(Result::unwrap).for_each(|line| {
        // println!("{}", line);
        let mut counter = counter.lock().unwrap();
        counter.increase();
    });
}
