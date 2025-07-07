use crate::{err::MyResult, time::EventTimeFormat, typ::CalendarEvent, url::URL};
use std::borrow::Cow;

pub fn aol(event: &CalendarEvent) -> MyResult<URL> {
    let fmt_typ = if event.is_all_day() {
        EventTimeFormat::AllDay
    } else {
        EventTimeFormat::DateTimeUtc
    };

    let mut p = vec![
        (Cow::Borrowed("v"), Cow::Borrowed("60")),
        (Cow::Borrowed("title"), Cow::Borrowed(event.title)),
        (
            Cow::Borrowed("st"),
            Cow::Owned(event.start.format_as_string(fmt_typ)),
        ),
        (
            Cow::Borrowed("et"),
            Cow::Owned(event.end_date().format_as_string(fmt_typ)),
        ),
    ];

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
        "https://calendar.aol.com",
        p.iter().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::__snapshot__::{generate_models, read_snapshot};

    #[test]
    fn should_provide_aol_calendar_link() {
        let snapshot = read_snapshot();
        let models = generate_models();
        let mut cases = snapshot.get("aol").unwrap().into_iter();
        for (i, evt) in models.iter().enumerate() {
            let act = aol(evt).expect("cannot parse aol event");
            let exp = cases.next().expect("sequence contains no elements");
            assert_eq!(act, URL::new(exp), "failed at index {i}, evt: {evt:?}");
        }
    }
}
