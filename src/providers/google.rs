use crate::{err::MyResult, stringify::make_url, typ::CalendarEvent};
use std::borrow::Cow;

pub fn google(event: CalendarEvent) -> MyResult<String> {
    let mut p = vec![(Cow::Borrowed("action"), Cow::Borrowed("TEMPLATE"))];

    let date_only = event.duration.is_all_day();
    p.push((
        Cow::Borrowed("dates"),
        Cow::Owned(format!(
            "{}/{}",
            event.start.format_as_str(date_only),
            (event.start + event.duration).format_as_str(date_only)
        )),
    ));

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

    make_url(
        "https://calendar.google.com/calendar/render",
        p.iter().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::EventTime;
    use crate::typ::EventDuration;
    use chrono::Duration;

    #[test]
    fn should_create_google_calendar_link() {
        let date = chrono::DateTime::parse_from_rfc3339("2019-12-29T00:00:00Z").unwrap();
        let evt = CalendarEvent {
            title: "Birthday party",
            start: EventTime::Utc(date.into()),
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
            start: EventTime::Utc(date.into()),
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
    #[test]
    fn should_create_google_calendar_link_all_day() {
        let date = chrono::DateTime::parse_from_rfc3339("2019-12-29T00:00:00Z").unwrap();
        let evt = CalendarEvent {
            title: "Birthday party",
            start: EventTime::Utc(date.into()),
            duration: EventDuration::AllDay,
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
            "https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229%2F20191230&text=Birthday%20party".replace("%20","+")
        );
    }
}
