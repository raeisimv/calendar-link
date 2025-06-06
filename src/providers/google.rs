use crate::{err::MyResult, stringify::make_url, typ::CalendarEvent};
use std::borrow::Cow;

pub fn google(event: &CalendarEvent) -> MyResult<String> {
    let mut p = vec![(Cow::Borrowed("action"), Cow::Borrowed("TEMPLATE"))];

    // let date_only = event.duration.is_all_day();
    p.push((
        Cow::Borrowed("dates"),
        Cow::Owned(format!(
            "{}/{}",
            event.start.format_as_string(date_only),
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
    if let Some(x) = &event.guests {
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
    use crate::prelude::google;
    use crate::providers::__snapshot__::{generate_models, read_snapshot};

    #[test]
    fn should_provide_google_calendar_link() {
        let snapshot = read_snapshot();
        let models = generate_models();
        let mut cases = snapshot.get("google").unwrap().into_iter();
        for (i, evt) in models.iter().enumerate() {
            let act = google(evt).expect("cannot parse google event");
            let exp = cases.next().expect("sequence contains no elements");
            assert_eq!(&act, exp, "failed at index {i}, evt: {evt:?}");
        }
    }
}
