use crate::prelude::EventTime;
use crate::typ::{CalendarEvent, EventStatus};
use chrono::Duration;
use std::collections::HashMap;
use std::ops::Add;

pub fn read_snapshot() -> HashMap<String, Vec<String>> {
    let raw = std::fs::read_to_string("__snapshots__/index.spec.ts.snap").expect("Can't read file");
    let mut map = HashMap::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('/') {
            continue;
        }
        let parts = line.split(' ').collect::<Vec<_>>();
        let name = parts[0].split('`').last().unwrap();
        let link = parts
            .last()
            .unwrap()
            .replace('`', "")
            .replace('"', "")
            .replace(';', "")
            .replace("%20", "+");

        // println!("name: {name}, link: {link}");
        map.entry(name.to_string())
            .or_insert(vec![])
            .push(link.to_string());
    }

    map
}
pub fn generate_models<'a>() -> Vec<CalendarEvent<'a>> {
    let start = chrono::DateTime::parse_from_rfc3339("2019-12-29T00:00:00Z").unwrap();
    let end = chrono::DateTime::parse_from_rfc3339("2020-01-12T00:00:00Z").unwrap();
    let evt = CalendarEvent {
        title: "Birthday party",
        start: EventTime::Utc(start.into()),
        duration: Some(Duration::hours(2)),
        url: None,
        uid: None,
        desc: None,
        busy: None,
        stat: None,
        r_rule: None,
        guests: None,
        all_day: None,
        location: None,
        organizer: None,
        end: None,
    };
    vec![
        CalendarEvent { ..evt.clone() },
        CalendarEvent {
            desc: Some("Bring gifts!"),
            ..evt.clone()
        },
        CalendarEvent {
            start: EventTime::Utc(start.into()),
            duration: Some(Duration::hours(2).into()),
            guests: Some(vec!["hello@example.com", "another@example.com"]),
            ..evt.clone()
        },
        CalendarEvent {
            start: EventTime::Utc(start.add(Duration::hours(11)).into()),
            // stat: Some(EventStatus::Confirmed),
            // all_day: Some(true),
            ..evt.clone()
        },
        CalendarEvent {
            stat: Some(EventStatus::Confirmed),
            end: Some(EventTime::Utc(end.into())),
            all_day: Some(true),
            ..evt.clone()
        },
        CalendarEvent {
            r_rule: Some("FREQ=YEARLY;INTERVAL=1"),
            ..evt.clone()
        },
        CalendarEvent {
            all_day: Some(true),
            ..evt.clone()
        },
    ]
}
