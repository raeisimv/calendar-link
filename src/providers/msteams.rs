use crate::{err::MyResult, stringify::make_url, time::EventTimeFormat, typ::CalendarEvent};
use std::borrow::Cow;

pub fn msteams(event: &CalendarEvent) -> MyResult<String> {
    let mut p = vec![
        (Cow::Borrowed("subject"), Cow::Borrowed(event.title)),
        (
            Cow::Borrowed("startTime"),
            Cow::Owned(event.start.format_as_string(EventTimeFormat::DateTimeUtc)),
        ),
        (
            Cow::Borrowed("endTime"),
            Cow::Owned(
                event
                    .end_date()
                    .format_as_string(EventTimeFormat::DateTimeUtc),
            ),
        ),
    ];

    if let Some(x) = event.desc {
        p.push((Cow::Borrowed("content"), Cow::Borrowed(x)));
    }

    make_url(
        "https://teams.microsoft.com/l/meeting/new",
        p.iter().map(|(x, y)| (x.as_ref(), y.as_ref())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::__snapshot__::{generate_models, read_snapshot};

    #[test]
    fn should_provide_msteams_calendar_link() {
        let snapshot = read_snapshot();
        let models = generate_models();
        let mut cases = snapshot.get("msteams").unwrap().into_iter();
        for (i, evt) in models.iter().enumerate() {
            let act = msteams(evt).expect("cannot parse msteams event");
            let exp = cases.next().expect("sequence contains no elements");
            assert_eq!(&act, exp, "failed at index {i}, evt: {evt:?}");
        }
    }
}
