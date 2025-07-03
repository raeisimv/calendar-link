use crate::{err::MyResult, time::EventTimeFormat, typ::CalendarEvent, url::URL};
use std::borrow::Cow;

pub fn ical(event: &CalendarEvent) -> MyResult<URL> {
    let mut p = vec![
        (Cow::Borrowed("v"), Cow::Borrowed("60")),
        (Cow::Borrowed("title"), Cow::Borrowed(event.title)),
    ];

    let fmt_typ = if event.is_all_day() {
        EventTimeFormat::AllDay
    } else {
        EventTimeFormat::DateTimeUtc
    };
    p.push((
        Cow::Borrowed("st"),
        Cow::Owned(event.start.format_as_string(fmt_typ)),
    ));
    p.push((
        Cow::Borrowed("et"),
        Cow::Owned(event.end_date().format_as_string(fmt_typ)),
    ));

    if let Some(x) = event.desc {
        p.push((Cow::Borrowed("desc"), Cow::Borrowed(x)));
    }
    if let Some(x) = event.location {
        p.push((Cow::Borrowed("in_loc"), Cow::Borrowed(x)));
    }
    if event.is_all_day() {
        p.push((Cow::Borrowed("dur"), Cow::Borrowed("allday")));
    } else {
        p.push((Cow::Borrowed("dur"), Cow::Borrowed("false")));
    }

    URL::try_build(
        "https://calendar.ical.com",
        p.iter().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::__snapshot__::{generate_models, read_snapshot};

    #[test]
    fn should_provide_ical_calendar_link() {
        let snapshot = read_snapshot();
        let models = generate_models();
        let mut cases = snapshot.get("iCal").unwrap().into_iter();
        for (i, evt) in models.iter().enumerate() {
            let act = ical(evt).expect("cannot parse iCal event");
            let exp = cases.next().expect("sequence contains no elements");
            assert_eq!(act, URL::new(exp), "failed at index {i}, evt: {evt:?}");
        }
    }
}
