use crate::time::{EventTime, TimeType};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct CalendarEvent<'a> {
    pub title: &'a str,
    pub start: EventTime,
    pub url: Option<&'a str>,
    pub uid: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub busy: Option<bool>,
    pub stat: Option<EventStatus>,
    pub r_rule: Option<&'a str>,
    pub guests: Option<Vec<&'a str>>,
    pub location: Option<&'a str>,
    pub duration: EventDuration,
    pub organizer: Option<EventOrganizer<'a>>,
}

#[derive(Clone, Debug)]
pub enum EventDuration {
    AllDay,
    For(chrono::Duration),
    EndsAt(EventTime),
}
impl EventDuration {
    pub fn all_day() -> EventDuration {
        EventDuration::AllDay
    }
    pub fn at(at: DateTime<Utc>) -> EventDuration {
        EventDuration::EndsAt(EventTime::DateTime(TimeType::Utc(at)))
    }
    pub fn ends_at(at: EventTime) -> EventDuration {
        EventDuration::EndsAt(at)
    }
    pub fn with_duration(dur: chrono::Duration) -> EventDuration {
        EventDuration::For(dur)
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
