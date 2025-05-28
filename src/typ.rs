use std::time::Instant;

#[derive(Clone, Debug)]
pub struct CalendarEvent<'a, D: IntoTime> {
    pub title: &'a str,
    pub start: D,
    pub url: Option<&'a str>,
    pub uid: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub busy: Option<bool>,
    pub stat: Option<EventStatus>,
    pub r_rule: Option<&'a str>,
    pub guests: Option<Vec<&'a str>>,
    pub location: Option<&'a str>,
    pub duration: EventDuration<D>,
    pub organizer: Option<EventOrganizer<'a>>,
}

#[derive(Copy, Clone, Debug)]
pub enum EventDuration<D> {
    AllDay,
    OneHour,
    OneAndHalfHours,
    TwoHours,
    Hours(u8),
    Days(u8),
    EndsAt(D),
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
pub trait IntoTime {
    fn into_date(self) -> Instant;
}

impl IntoTime for Instant {
    fn into_date(self) -> Instant {
        self
    }
}
