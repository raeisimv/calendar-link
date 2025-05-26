use std::time::Instant;

#[derive(Clone, Debug)]
pub struct CalendarEvent<S: AsRef<str>, D: IntoTime, E: IntoEventOrganizer<S>> {
    pub title: S,
    pub start: D,
    pub url: Option<S>,
    pub uid: Option<S>,
    pub desc: Option<S>,
    pub busy: Option<bool>,
    pub stat: EventStatus,
    pub r_rule: Option<S>,
    pub guests: Option<Vec<S>>,
    pub location: Option<S>,
    pub duration: EventDuration<D>,
    pub organizer: Option<E>,
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
pub struct EventOrganizer<S: AsRef<str>> {
    pub name: S,
    pub email: S,
}
pub trait IntoEventOrganizer<S: AsRef<str>> {
    fn into_event_organizer(self) -> EventOrganizer<S>;
}
impl<S: AsRef<str>> IntoEventOrganizer<S> for (S, S) {
    fn into_event_organizer(self) -> EventOrganizer<S> {
        EventOrganizer {
            name: self.0,
            email: self.1,
        }
    }
}
pub trait IntoTime {
    fn into_date(self) -> Instant;
}

impl IntoTime for Instant {
    fn into_date(self) -> Instant {
        self
    }
}
