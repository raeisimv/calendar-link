use chrono::{DateTime, Local, Utc};
use core::fmt::{Display, Formatter};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum EventTime {
    Utc(DateTime<Utc>),
    Local(DateTime<Local>),
}

pub fn get_timestamp() -> String {
    Utc::now()
        .format(EventTimeFormat::DateTimeUtc.as_ref())
        .to_string()
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

    pub fn is_utc(&self) -> bool {
        match self {
            EventTime::Utc(_) => true,
            EventTime::Local(_) => false,
        }
    }
    pub fn format_as_string(&self, format: EventTimeFormat) -> String {
        let fmt = format.as_ref();
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
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.is_utc() {
            write!(f, "{}", self.format_as_string(EventTimeFormat::DateTimeUtc))
        } else {
            write!(
                f,
                "{}",
                self.format_as_string(EventTimeFormat::DateTimeLocal)
            )
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EventTimeFormat {
    AllDay,
    DateTimeUtc,
    DateTimeLocal,
}
impl AsRef<str> for EventTimeFormat {
    fn as_ref(&self) -> &str {
        match self {
            EventTimeFormat::AllDay => "%Y%m%d",
            EventTimeFormat::DateTimeUtc => "%Y%m%dT%H%M%SZ",
            EventTimeFormat::DateTimeLocal => "%Y-%m-%dT%H:%M:%S",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_utc() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::Utc(d);
        assert_eq!(x.to_string(), "20191228T120000Z");

        let x = EventTime::Utc(d);
        assert_eq!(x.format_as_string(EventTimeFormat::AllDay), "20191228");
    }
    #[test]
    fn should_format_local() {
        let d = EventTime::fixed_utc_time();
        let x = EventTime::Local(d.into());
        assert_eq!(x.to_string(), "2019-12-28T15:30:00"); // it may not pass on your system

        let x = EventTime::Local(d.into());
        assert_eq!(x.format_as_string(EventTimeFormat::AllDay), "20191228");
    }
}
