use crate::typ::EventDuration;
use chrono::{DateTime, Duration, Local, Utc};
use core::{
    fmt::{Display, Formatter},
    ops::Add,
};

#[derive(Debug, Clone, Copy)]
pub enum EventTime {
    Utc(DateTime<Utc>),
    Local(DateTime<Local>),
}
impl EventTime {
    #[cfg(test)]
    pub fn fixed_utc_time() -> DateTime<Utc> {
        let utc_str_z = "2019-12-28T12:00:00.000Z";
        chrono::DateTime::parse_from_rfc3339(utc_str_z)
            .unwrap()
            .to_utc()
    }
    #[cfg(test)]
    pub fn fixed_utc() -> Self {
        EventTime::Utc(Self::fixed_utc_time())
    }

    pub fn format_as_str(&self, date_only: bool) -> String {
        let fmt = if date_only {
            "%Y%m%d"
        } else {
            match self {
                EventTime::Utc(_) => "%Y%m%dT%H%M%SZ",
                EventTime::Local(_) => "%Y-%m-%dT%H:%M:%S",
            }
        };
        match self {
            EventTime::Utc(x) => x.format(fmt).to_string(),
            EventTime::Local(x) => x.format(fmt).to_string(),
        }
    }
}
impl Default for EventTime {
    fn default() -> Self {
        Self::Utc(Default::default())
    }
}
impl Display for EventTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_as_str(false))
    }
}

impl Add<EventDuration> for EventTime {
    type Output = EventTime;

    fn add(self, rhs: EventDuration) -> Self::Output {
        use EventTime::*;

        match self {
            Utc(x) => match rhs {
                EventDuration::AllDay => Utc(x.add(Duration::days(1))),
                EventDuration::For(dur) => Utc(x.add(dur)),
                EventDuration::EndsAt(d) => d,
            },
            Local(x) => match rhs {
                EventDuration::AllDay => Local(x.add(Duration::days(1))),
                EventDuration::For(dur) => Local(x.add(dur)),
                EventDuration::EndsAt(d) => d,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typ::EventDuration;

    #[test]
    fn should_format_utc() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::Utc(d);
        assert_eq!(x.to_string(), "20191228T120000Z");

        let x = EventTime::Utc(d);
        assert_eq!(x.format_as_str(true), "20191228");
    }
    #[test]
    fn should_format_local() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::Local(d.into());
        assert_eq!(x.to_string(), "2019-12-28T15:30:00"); // it may not pass on your system

        let x = EventTime::Local(d.into());
        assert_eq!(x.format_as_str(true), "20191228");
    }
    #[test]
    fn should_sum_up_with_event_duration() {
        let start = EventTime::fixed_utc();
        let dur = EventDuration::AllDay;
        let end = start + dur;
        assert_eq!(end.to_string(), "20191229T120000Z");

        let dur = EventDuration::For(Duration::hours(1));
        let end = start + dur;
        assert_eq!(end.to_string(), "20191228T130000Z");

        let dur = EventDuration::at(EventTime::fixed_utc_time().add(Duration::hours(2)));
        let end = start + dur;
        assert_eq!(end.to_string(), "20191228T140000Z");
    }
}
