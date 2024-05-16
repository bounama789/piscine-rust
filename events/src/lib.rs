use chrono::Duration;
use colored::Color;

#[derive(Debug, PartialEq, Eq)]
pub struct Notification {
    pub size: u32,
    pub color: Color,
    pub position: Position,
    pub content: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug)]
pub enum Event<'s> {
    Remainder(&'s str),
    Registration(Duration),
    Appointment(&'s str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}",self))
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("({}, {}, [{}m{}[0m)", self.position,self.size,self.color.to_fg_str(),self.content))
    }
}

use Position::*;
use Color::TrueColor;

impl<'s> Event<'s> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(msg) => Notification { size: 50, color: TrueColor { r: 50, g: 50, b: 50 }, position:  Bottom, content: msg.to_string() },
            Event::Registration(dur) => Notification { size: 30, color: TrueColor { r: 255, g: 2, b: 22 }, position: Top, content: format!("You have {}H:{}M:{}s left before the registration ends",dur.num_hours(),dur.num_minutes(),dur.num_seconds()) },
            Event::Appointment(msg) => Notification { size: 100, color: TrueColor { r: 200, g: 200, b: 3 }, position: Center, content: msg.to_string() },
            Event::Holiday => Notification { size: 25, color: TrueColor { r: 0, g: 255, b: 0 }, position: Top, content: "Enjoy your holiday".to_string() }
        }
    }
}
