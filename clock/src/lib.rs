use std::fmt;

const DAY : i32 = 24 * 60;
const HOUR : i32 = 60;

#[derive(Debug,PartialEq)]
pub struct Clock {
    hours:i32,
    minutes : i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {  
        let total_minutes = (((hours * HOUR + minutes)%DAY)+DAY)%DAY;
        Clock { hours: total_minutes/HOUR, minutes: total_minutes%HOUR }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours,self.minutes+minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:02}:{:02}",self.hours,self.minutes)
    }
}
