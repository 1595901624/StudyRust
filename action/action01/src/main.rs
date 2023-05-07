use std::thread::sleep;
use std::time::Duration;

struct Clock {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl Clock {
    fn new() -> Clock {
        Clock {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    fn add_seconds(&mut self, seconds: u8) {
        self.seconds += seconds;
        if self.seconds >= 60 {
            self.seconds -= 60;
            self.add_minutes(1);
        }
    }

    fn add_minutes(&mut self, minutes: u8) {
        self.minutes += minutes;
        if self.minutes >= 60 {
            self.minutes -= 60;
            self.add_hours(1);
        }
    }

    fn add_hours(&mut self, hours: u8) {
        self.hours = (self.hours + hours) % 24;
    }

    fn display_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

fn main() {
    let mut c = Clock::new();
    loop {
        println!("{}", c.display_time());
        c.add_seconds(1);
        sleep(Duration::from_secs(1));
    }
}