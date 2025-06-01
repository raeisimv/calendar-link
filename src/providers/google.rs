use crate::err::MyResult;
use crate::stringify::stringify;
use crate::typ::CalendarEvent;
use std::borrow::Cow;

pub fn google(event: CalendarEvent) -> MyResult<String> {
    let mut p = vec![
        (Cow::Borrowed("action"), Cow::Borrowed("TEMPLATE")),
        (
            Cow::Borrowed("dates"),
            Cow::Owned(format!("{}/{}", event.start, event.start + event.duration)),
        ),
    ];

    if let Some(x) = event.desc {
        p.push((Cow::Borrowed("details"), Cow::Borrowed(x)));
    }
    p.push((Cow::Borrowed("text"), Cow::Borrowed(event.title)));

    if let Some(x) = event.busy {
        p.push((Cow::Borrowed("trp"), Cow::Owned(x.to_string())));
    }
    if let Some(x) = event.r_rule {
        p.push((Cow::Borrowed("recur"), Cow::Owned(format!("RRULE: {}", x))));
    }
    if let Some(x) = event.guests {
        if !x.is_empty() {
            p.push((Cow::Borrowed("add"), Cow::Owned(x.join(","))));
        }
    }

    stringify(
        "https://calendar.google.com/calendar/render",
        p.iter().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{EventTime, TimeType};
    use crate::typ::EventDuration;
    use chrono::Duration;

    #[test]
    fn should_create_google_calendar_link() {
        let date = chrono::DateTime::parse_from_rfc3339("2019-12-29T00:00:00Z").unwrap();
        let evt = CalendarEvent {
            title: "Birthday party",
            start: EventTime::DateTime(TimeType::Utc(date.into())),
            duration: EventDuration::For(Duration::hours(2)),
            url: None,
            uid: None,
            desc: None,
            busy: None,
            stat: None,
            r_rule: None,
            guests: None,
            location: None,
            organizer: None,
        };
        let link = google(evt).unwrap();
        assert_eq!(
            link.as_str(),
            "https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T000000Z%2F20191229T020000Z&text=Birthday%20party".replace("%20","+")
        );
    }
    #[test]
    fn should_create_google_calendar_link_with_desc() {
        let date = chrono::DateTime::parse_from_rfc3339("2019-12-29T00:00:00Z").unwrap();
        let evt = CalendarEvent {
            title: "Birthday party",
            start: EventTime::DateTime(TimeType::Utc(date.into())),
            duration: EventDuration::For(Duration::hours(2)),
            url: None,
            uid: None,
            desc: Some("Bring gifts!"),
            busy: None,
            stat: None,
            r_rule: None,
            guests: None,
            location: None,
            organizer: None,
        };
        let link = google(evt).unwrap();
        assert_eq!(
            link.as_str(),
            "https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T000000Z%2F20191229T020000Z&details=Bring%20gifts%21&text=Birthday%20party".replace("%20","+")
        );
    }
}
