use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::text::Font;
use kiss3d::window::Window;

struct Counter {
    stop: Arc<AtomicBool>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn run(&self, dur: std::time::Duration, count: Arc<Mutex<usize>>) {
        let to_stop = Arc::clone(&self.stop);
        thread::spawn(move || loop {
            if to_stop.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            thread::sleep(dur);
            *count.lock().unwrap() += 1;
        });
    }
    pub fn stop(&self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed)
    }
}

struct WindowThread {
    count: Arc<Mutex<usize>>,
    counter: Counter,
    window: Window,
}
impl WindowThread {
    fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
            counter: Counter::new(),
            window: Window::new("counter gui"),
        }
    }
    fn run(&mut self) {
        let time = std::time::Duration::from_millis(300);
        self.counter.run(time, Arc::clone(&self.count));
        let font = Font::default();
        while self.window.render() {
            let count = *self.count.lock().unwrap();
            if count >= 20 {
                self.counter.stop();
            }
            self.window.draw_text(
                &format!("count: {}", count),
                &Point2::new(1.0, 1.0),
                60.0,
                &font,
                &Point3::new(1.0, 1.0, 1.0),
            );
        }
    }
}
fn main() {
    let mut window_therad = WindowThread::new();
    window_therad.run()
}
