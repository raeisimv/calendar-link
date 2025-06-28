use crate::{err::MyResult, time::EventTimeFormat, typ::CalendarEvent, url::URL};
use std::borrow::Cow;

pub fn outlook(event: &CalendarEvent) -> MyResult<URL> {
    let p = set_params(event);
    URL::try_build(
        "https://outlook.live.com/calendar/0/action/compose",
        p.iter().rev().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}
pub fn outlook_mobile(event: &CalendarEvent) -> MyResult<URL> {
    let p = set_params(event);
    URL::try_build(
        "https://outlook.live.com/calendar/0/deeplink/compose",
        p.iter().rev().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}
pub fn office_365(event: &CalendarEvent) -> MyResult<URL> {
    let p = set_params(event);
    URL::try_build(
        "https://outlook.office.com/calendar/0/action/compose",
        p.iter().rev().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}
pub fn office_365_mobile(event: &CalendarEvent) -> MyResult<URL> {
    let p = set_params(event);
    URL::try_build(
        "https://outlook.office.com/calendar/0/deeplink/compose",
        p.iter().rev().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

fn set_params<'a>(event: &'a CalendarEvent) -> Vec<(Cow<'a, str>, Cow<'a, str>)> {
    let mut p = vec![
        (
            Cow::Borrowed("path"),
            Cow::Borrowed("/calendar/action/compose"),
        ),
        (Cow::Borrowed("rru"), Cow::Borrowed("addevent")),
        (Cow::Borrowed("subject"), Cow::Borrowed(event.title)),
        (
            Cow::Borrowed("start"),
            Cow::Owned(event.start.format_as_string(EventTimeFormat::DateTimeLocal)),
        ),
        (
            Cow::Borrowed("end"),
            Cow::Owned(
                event
                    .end_date()
                    .format_as_string(EventTimeFormat::DateTimeLocal),
            ),
        ),
    ];

    if let Some(x) = event.desc {
        p.push((Cow::Borrowed("body"), Cow::Borrowed(x)));
    }

    if let Some(x) = event.location {
        p.push((Cow::Borrowed("location"), Cow::Borrowed(x)));
    }
    if let Some(x) = event.r_rule {
        p.push((Cow::Borrowed("recur"), Cow::Owned(format!("RRULE:{}", x))));
    }

    p.push((
        Cow::Borrowed("allday"),
        Cow::Owned(event.is_all_day().to_string()),
    ));

    p
}

#[cfg(test)]
mod tests {
    use crate::prelude::outlook;
    use crate::providers::__snapshot__::{generate_models, read_snapshot};
    use crate::url::URL;

    #[test]
    fn should_provide_outlook_calendar_link() {
        let snapshot = read_snapshot();
        let models = generate_models();
        let mut cases = snapshot.get("outlook").unwrap().into_iter();
        for (i, evt) in models.iter().enumerate() {
            let act = outlook(evt).expect("cannot parse outlook event");
            let exp = cases.next().expect("sequence contains no elements");
            assert_eq!(act, URL::new(exp), "failed at index {i}, evt: {evt:?}");
        }
    }
}
