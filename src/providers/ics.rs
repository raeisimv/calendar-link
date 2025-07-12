use crate::time::get_timestamp;
use crate::{err::MyResult, time::EventTimeFormat, typ::CalendarEvent};
use std::borrow::Cow;

pub fn ics(event: &CalendarEvent) -> MyResult<String> {
    let fmt_typ = if event.is_all_day() {
        EventTimeFormat::AllDay
    } else {
        EventTimeFormat::DateTimeUtc
    };

    let mut p = vec![
        (Cow::Borrowed("BEGIN"), Cow::Borrowed("VCALENDAR")),
        (Cow::Borrowed("VERSION"), Cow::Borrowed("2.0")),
        (
            Cow::Borrowed("PRODID"),
            Cow::Borrowed("-//AnandChowdhary//calendar-link//EN"),
        ),
        (Cow::Borrowed("BEGIN"), Cow::Borrowed("VEVENT")),
    ];
    if let Some(x) = event.url {
        p.push((Cow::Borrowed("URL"), Cow::Borrowed(x)));
    }
    p.extend_from_slice(&[
        (
            Cow::Borrowed("DTSTART"),
            Cow::Owned(event.start.format_as_string(fmt_typ)),
        ),
        (
            Cow::Borrowed("DTEND"),
            Cow::Owned(event.end_date().format_as_string(fmt_typ)),
        ),
        (Cow::Borrowed("DTSTAMP"), Cow::Owned(get_timestamp())),
    ]);
    if event.is_all_day() {
        p.extend_from_slice(&[
            (
                Cow::Borrowed("X-MICROSOFT-CDO-ALLDAYEVENT"),
                Cow::Borrowed("TRUE"),
            ),
            (
                Cow::Borrowed("X-MICROSOFT-MSNCALENDAR-ALLDAYEVENT"),
                Cow::Borrowed("TRUE"),
            ),
        ]);
    }

    if let Some(x) = event.r_rule {
        p.push((Cow::Borrowed("RRULE"), Cow::Borrowed(x)));
    }

    p.extend_from_slice(&[
        (
            Cow::Borrowed("SUMMARY"),
            Cow::Owned(sanitized_text(event.title)),
        ),
        (
            Cow::Borrowed("DESCRIPTION"),
            Cow::Owned(sanitized_text(event.desc.unwrap_or_default())),
        ),
        (
            Cow::Borrowed("LOCATION"),
            Cow::Owned(sanitized_text(event.location.unwrap_or_default())),
        ),
        (Cow::Borrowed("ORGANIZER"), Cow::Borrowed("")),
        // (
        //     Cow::Borrowed("STATUS"),
        //     Cow::Owned(sanitized_text(event.stat.unwrap_or_default())),
        // ),
        // (Cow::Borrowed("UID"), Cow::Owned("rnd".into())),
        (Cow::Borrowed("END"), Cow::Borrowed("VEVENT")),
        (Cow::Borrowed("END"), Cow::Borrowed("VCALENDAR")),
    ]);

    let x = p
        .into_iter()
        .map(|(k, v)| {
            if k == "ORGANIZER"
                && let Some(x) = &event.organizer
            {
                format!("ORGANIZER;CN={}:MAILTO:{}\r\n", x.name, x.email)
            } else {
                format!("{k}:{v}\r\n")
            }
        })
        .collect::<String>();

    Ok(format!("data:text/calendar;charset=utf8,{x}"))
}

fn sanitized_text(text: &str) -> String {
    text.replace(",", ",")
        .replace(";", ";")
        .replace("\r\n", "\n")
        .replace("\n", "\\n")
    // .replace(/(\\n)[\s\t]+/gm, "\\n"); // TODO: impl this regex
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::providers::__snapshot__::{generate_models, read_snapshot};
//
//     #[test]
//     fn should_provide_ical_calendar_link() {
//         let snapshot = read_snapshot();
//         let models = generate_models();
//         let mut cases = snapshot.get("iCal").unwrap().into_iter();
//         for (i, evt) in models.iter().enumerate() {
//             let act = ics(evt).expect("cannot parse iCal event");
//             let exp = cases.next().expect("sequence contains no elements");
//             // assert_eq!(act, URL::new(exp), "failed at index {i}, evt: {evt:?}");
//         }
//     }
// }
