#[cfg(test)]
mod __snapshot__;
mod aol;
mod google;
mod ical;
mod msteams;
mod outlook;
mod yahoo;

pub use aol::*;
pub use google::*;
pub use ical::*;
pub use msteams::*;
pub use outlook::*;
pub use yahoo::*;
