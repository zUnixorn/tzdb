// GENERATED FILE
// ALL CHANGES MADE IN THIS FOLDER WILL BE LOST!

// SPDX-License-Identifier: MIT-0 OR MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2022-2026 René Kijewski <crates.io@k6i.de>

#![allow(unknown_lints)]
#![allow(clippy::pedantic)]

#[cfg(all(test, not(miri)))]
mod test_all_names;

pub(crate) mod by_name;
mod raw_tzdata;
mod tz_names;
mod tzdata;

/// All defined time zones statically accessible
pub mod time_zone;

/// The version of the source Time Zone Database
pub const VERSION: &str = "2026d";

/// The SHA512 hash of the source Time Zone Database (using the "Complete Distribution")
pub const VERSION_HASH: &str = "b2f4622ef0c2a33ee4a9282c8346dc2732e044617b3b7a1288bc065910653ed8109f01b1423b4b497373f9bc2b9fefb0a75a02c6b79e266ea2fe9767ec32ee89";

#[allow(unreachable_pub)] // false positive
pub use self::tz_names::TZ_NAMES;
