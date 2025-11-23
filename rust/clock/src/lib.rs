use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const DAY: u32 = 24 * 60;
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let result: Self = Clock {
            minutes: 0,
            hours: 0,
        };

        result.add_minutes(hours * 60 + minutes)
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut minutes = (0..)
            .map(move |x| {
                let r = (x * DAY as i32) + self.minutes + minutes + (hours) * 60;

                return r;
            })
            .filter(|x| x.is_positive())
            .nth(1)
            .unwrap_or(0);
        hours = minutes.div_euclid(60);
        if minutes > 0 {
            minutes = minutes.rem_euclid(60);
        }
        if hours > 0 {
            hours = hours.rem_euclid(24);
        }
        Clock { hours, minutes }
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
