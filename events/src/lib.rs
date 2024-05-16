use chrono::Duration;
use colored::Color;

#[derive(Debug, PartialEq, Eq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
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
        f.write_str(&format!("{:?}", self))
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = TrueColor {
            r: self.color.0,
            g: self.color.1,
            b: self.color.2,
        };
        f.write_str(&format!(
            "({}, {}, [{}m{}[0m)",
            self.position,
            self.size,
            color.to_fg_str(),
            self.content
        ))
    }
}

use Color::TrueColor;
use Position::*;

impl<'s> Event<'s> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(msg) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Bottom,
                content: msg.to_string(),
            },
            Event::Registration(dur) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Top,
                content: format!(
                    "You have {}H:{}M:{}s left before the registration ends",
                    dur.num_hours(),
                    dur.num_minutes(),
                    dur.num_seconds()
                ),
            },
            Event::Appointment(msg) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Center,
                content: msg.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
