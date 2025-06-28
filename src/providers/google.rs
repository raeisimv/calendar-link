use crate::{err::MyResult, time::EventTimeFormat, typ::CalendarEvent, url::URL};
use std::borrow::Cow;

pub fn google(event: &CalendarEvent) -> MyResult<URL> {
    let mut p = vec![(Cow::Borrowed("action"), Cow::Borrowed("TEMPLATE"))];

    if let Some(x) = &event.guests {
        if !x.is_empty() {
            p.push((Cow::Borrowed("add"), Cow::Owned(x.join(","))));
        }
    }

    let dates = {
        let fmt_typ = if event.is_all_day() {
            EventTimeFormat::AllDay
        } else {
            EventTimeFormat::DateTimeUtc
        };
        format!(
            "{}/{}",
            event.start.format_as_string(fmt_typ),
            event.end_date().format_as_string(fmt_typ)
        )
    };
    p.push((Cow::Borrowed("dates"), Cow::Owned(dates)));

    if let Some(x) = event.desc {
        p.push((Cow::Borrowed("details"), Cow::Borrowed(x)));
    }
    if let Some(x) = event.r_rule {
        p.push((Cow::Borrowed("recur"), Cow::Owned(format!("RRULE:{}", x))));
    }

    p.push((Cow::Borrowed("text"), Cow::Borrowed(event.title)));

    if let Some(x) = event.busy {
        p.push((Cow::Borrowed("trp"), Cow::Owned(x.to_string())));
    }

    URL::try_build(
        "https://calendar.google.com/calendar/render",
        p.iter().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

#[cfg(test)]
mod tests {
    use crate::prelude::google;
    use crate::providers::__snapshot__::{generate_models, read_snapshot};
    use crate::url::URL;

    #[test]
    fn should_provide_google_calendar_link() {
        let snapshot = read_snapshot();
        let models = generate_models();
        let mut cases = snapshot.get("google").unwrap().into_iter();
        for (i, evt) in models.iter().enumerate() {
            let act = google(evt).expect("cannot parse google event");
            let exp = cases.next().expect("sequence contains no elements");
            assert_eq!(act, URL::new(exp), "failed at index {i}, evt: {evt:?}");
        }
    }
}
