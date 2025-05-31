# Calendar Link

Rust library to generate an event link for Google Calendar, Yahoo! Calendar, Microsoft Outlook, etc.

### Example

```rust
fn main() {
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
        "https://calendar.google.com/calendar/render?action=TEMPLATE&dates=20191229T000000Z%2F20191229T020000Z&text=Birthday%20party".replace("%20", "+")
    );
}
```

*the library heavily is under construction*

### Stages 1

- [x] Google calendar link
- [ ] Outlook
- [ ] Outlook Mobile
- [ ] Office 365
- [ ] Office 365 Mobile
- [ ] Yahoo
- [ ] AOL
- [ ] MS Teams
- [ ] iCalendar

### LICENSE

MIT

#### Inspiration

This package draws inspiration from and is a port of
the [corresponding NPM](https://www.npmjs.com/package/calendar-link)
package.