use crate::EventDuration;
use chrono::{DateTime, Duration, Local, Utc};
use core::{
    fmt::{Display, Formatter},
    ops::Add,
};

#[derive(Debug, Clone, Copy)]
pub enum EventTime {
    DateTime(TimeType),
    DateOnly(TimeType),
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
        EventTime::DateTime(TimeType::Utc(Self::fixed_utc_time()))
    }
}
impl Default for EventTime {
    fn default() -> Self {
        Self::DateTime(Default::default())
    }
}
impl Display for EventTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventTime::DateTime(x) => {
                write!(f, "{}", x.to_datetime_string())
            }
            EventTime::DateOnly(x) => {
                write!(f, "{}", x.to_date_only_string())
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TimeType {
    Utc(DateTime<Utc>),
    Local(DateTime<Local>),
}
impl TimeType {
    pub fn to_date_only_string(&self) -> String {
        //  allDay: "YYYYMMDD"
        // Example: "20250530"
        match self {
            TimeType::Utc(x) => x.format("%Y%m%d").to_string(),
            TimeType::Local(x) => x.format("%Y%m%d").to_string(),
        }
    }
    pub fn to_datetime_string(&self) -> String {
        match self {
            TimeType::Utc(x) => {
                // dateTimeUTC: "YYYYMMDD[T]HHmmss[Z]"
                // Example: "20250530T175111Z" (UTC time)
                x.format("%Y%m%dT%H%M%SZ").to_string()
            }
            TimeType::Local(x) => {
                // dateTimeLocal: "YYYY-MM-DD[T]HH:mm:ss"
                // Example: "2025-05-30T19:51:11" (assuming CEST, which is UTC+2)
                x.format("%Y-%m-%dT%H:%M:%S").to_string()
            }
        }
    }
}
impl Default for TimeType {
    fn default() -> Self {
        Self::Utc(Utc::now())
    }
}

impl Add<EventDuration> for EventTime {
    type Output = EventTime;

    fn add(self, rhs: EventDuration) -> Self::Output {
        use EventTime::*;
        use TimeType::*;

        let start = match self {
            DateTime(x) => x,
            DateOnly(x) => x,
        };
        match start {
            Utc(x) => match rhs {
                EventDuration::AllDay => DateTime(Utc(x.add(Duration::days(1)))),
                EventDuration::For(dur) => DateTime(Utc(x.add(dur))),
                EventDuration::EndsAt(d) => d,
            },
            Local(x) => match rhs {
                EventDuration::AllDay => DateTime(Local(x.add(Duration::days(1)))),
                EventDuration::For(dur) => DateTime(Local(x.add(dur))),
                EventDuration::EndsAt(d) => d,
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_utc() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::DateTime(TimeType::Utc(d));
        assert_eq!(x.to_string(), "20191228T120000Z");

        let x = EventTime::DateOnly(TimeType::Utc(d));
        assert_eq!(x.to_string(), "20191228");
    }
    #[test]
    fn should_format_local() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::DateTime(TimeType::Local(d.into()));
        assert_eq!(x.to_string(), "2019-12-28T15:30:00"); // it may not pass on your system

        let x = EventTime::DateOnly(TimeType::Local(d.into()));
        assert_eq!(x.to_string(), "20191228");
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
