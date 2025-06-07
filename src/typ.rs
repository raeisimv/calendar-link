use crate::time::EventTime;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct CalendarEvent<'a> {
    pub title: &'a str,
    pub start: EventTime,
    pub end: Option<EventTime>,
    pub url: Option<&'a str>,
    pub uid: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub busy: Option<bool>,
    pub stat: Option<EventStatus>,
    pub r_rule: Option<&'a str>,
    pub guests: Option<Vec<&'a str>>,
    pub all_day: Option<bool>,
    pub location: Option<&'a str>,
    pub duration: Option<chrono::Duration>,
    pub organizer: Option<EventOrganizer<'a>>,
}
impl<'a> CalendarEvent<'a> {
    pub fn end_date(&self) -> EventTime {
        if let Some(end) = self.end {
            return end;
        }

        let dur = if self.all_day.unwrap_or(false) {
            Some(chrono::Duration::days(1))
        } else {
            self.duration
        };

        if let Some(dur) = dur {
            match self.start {
                EventTime::Utc(x) => EventTime::Utc(x.add(dur)),
                EventTime::Local(x) => EventTime::Local(x.add(dur)),
            }
        } else {
            self.start.clone()
        }
    }
    pub fn is_all_day(&self) -> bool {
        self.all_day.unwrap_or(false)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EventStatus {
    Confirmed,
    Tentative,
    Cancelled,
}
#[derive(Clone, Debug)]
pub struct EventOrganizer<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
