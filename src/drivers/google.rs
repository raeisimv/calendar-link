use crate::{stringify, CalendarEvent, MyResult};
use std::borrow::Cow;

pub fn google(event: CalendarEvent) -> MyResult<String> {
    let mut p = vec![
        (Cow::Borrowed("action"), Cow::Borrowed("TEMPLATE")),
        (
            Cow::Borrowed("dates"),
            Cow::Owned(format!("{}/{}", event.start, event.start + event.duration)),
        ),
        (Cow::Borrowed("text"), Cow::Borrowed(event.title)),
    ];

    if let Some(x) = event.desc {
        p.push((Cow::Borrowed("details"), Cow::Borrowed(x)));
    }
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
    use crate::{EventDuration, EventTime, TimeType};
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
}

// exports[`google service generate a google link 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T000000Z%2F20191229T020000Z&text=Birthday%20party"`;
//
// exports[`google service generate a google link with description 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T000000Z%2F20191229T020000Z&details=Bring%20gifts%21&text=Birthday%20party"`;
//
// exports[`google service generate a google link with guests 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&add=hello%40example.com%2Canother%40example.com&dates=20191229T000000Z%2F20191229T020000Z&text=Birthday%20party"`;
//
// exports[`google service generate a google link with time & timezone 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T110000Z%2F20191229T130000Z&text=Birthday%20party"`;
//
// exports[`google service generate a multi day google link 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229%2F20200112&text=Birthday%20party"`;
//
// exports[`google service generate a recurring google link 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T000000Z%2F20191229T020000Z&recur=RRULE%3AFREQ%3DYEARLY%3BINTERVAL%3D1&text=Birthday%20party"`;
//
// exports[`google service generate an all day google link 1`] = `"https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229%2F20191230&text=Birthday%20party"`;
