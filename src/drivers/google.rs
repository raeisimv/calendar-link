use crate::{CalendarEvent, IntoTime, MyResult, stringify};
use std::borrow::Cow;

pub fn google<D: IntoTime>(event: CalendarEvent<D>) -> MyResult<String> {
    let mut p = vec![
        (Cow::Borrowed("action"), Cow::Borrowed("TEMPLATE")),
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
    use crate::EventDuration;
    use std::time::Instant;

    #[test]
    fn should_create_google_calendar_link() {
        let evt = CalendarEvent {
            title: "The birthday",
            start: Instant::now(),
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
        assert_eq!(link.as_str(), "https://calendar.google.com/calendar/render");
    }
}
