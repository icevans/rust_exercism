use std::cmp::min;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_hours = minutes.div_euclid(60);
        let minute = minutes.rem_euclid(60);

        let hours = hours + extra_hours;
        let hour = hours.rem_euclid(24);

        Clock { hour, minute }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minute + minutes;

        Self::new(self.hour, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.minute)
    }
}
