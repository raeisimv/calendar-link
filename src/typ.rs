use crate::time::EventTime;

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
    pub all_day: Option<bool>,
    pub location: Option<&'a str>,
    pub duration: Option<chrono::Duration>,
    pub organizer: Option<EventOrganizer<'a>>,
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
