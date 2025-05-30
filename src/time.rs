use chrono::{DateTime, Local, Utc};
use core::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum EventTime {
    DateTime(TimeType),
    DateOnly(TimeType),
}
impl Display for EventTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventTime::DateTime(x) => {
                write!(f, "{}", x.to_datetime_string())
            }
            EventTime::DateOnly(x) => {
                write!(f, "{:?}", x.to_date_only_string())
            }
        }
    }
}
#[derive(Debug, Clone)]
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
