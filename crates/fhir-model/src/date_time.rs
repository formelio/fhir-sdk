//! FHIR Time, Date, DateTime and Instant types.

use std::{cmp::Ordering, str::FromStr};

use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, error::Parse, format_description::well_known::Rfc3339};

use crate::error::DateFormatError;

/// FHIR instant type: <https://hl7.org/fhir/datatypes.html#instant>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Instant(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);

/// FHIR date type: <https://hl7.org/fhir/datatypes.html#date>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Date {
	/// Date in the format of YYYY
	Year(i32),
	/// Date in the format of YYYY-MM
	YearMonth(i32, time::Month),
	/// Date in the format of YYYY-MM-DD
	Date(time::Date),
}

/// Returned when the date did not contain enough information to form a
/// [`time::Date`]
#[derive(Debug)]
pub struct InsufficientDatePrecision;

impl std::fmt::Display for InsufficientDatePrecision {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "insufficient date precision for conversion to time::Date")
	}
}

impl TryFrom<Date> for time::Date {
	type Error = InsufficientDatePrecision;

	fn try_from(date: Date) -> Result<Self, Self::Error> {
		match date {
			Date::Date(d) => Ok(d),
			_ => Err(InsufficientDatePrecision),
		}
	}
}

/// FHIR dateTime type: <https://hl7.org/fhir/datatypes.html#dateTime>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum DateTime {
	/// Date that does not contain time or timezone
	Date(Date),
	/// Full date and time
	DateTime(Instant),
}

impl TryFrom<DateTime> for time::Date {
	type Error = <Self as TryFrom<Date>>::Error;

	fn try_from(datetime: DateTime) -> Result<Self, Self::Error> {
		match datetime {
			DateTime::Date(d) => d.try_into(),
			DateTime::DateTime(dt) => Ok(dt.0.date()),
		}
	}
}

/// FHIR time type: <https://hl7.org/fhir/datatypes.html#time>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Time(#[serde(with = "serde_time")] pub time::Time);

/// Serde module for serialize and deserialize function for the type.
mod serde_time {
	use serde::{Deserialize, Serialize};
	use time::{format_description::FormatItem, macros::format_description};

	/// Time format `hh:mm`.
	const TIME_MINUTE_FORMAT: &[FormatItem<'_>] = format_description!("[hour]:[minute]");

	const OPTIONAL_SECONDS: FormatItem<'_> =
		FormatItem::Optional(&FormatItem::Compound(format_description!(":[second]")));

	/// Time format `hh:mm:ss`.
	const TIME_FORMAT: &[FormatItem<'_>] =
		&[FormatItem::Compound(TIME_MINUTE_FORMAT), OPTIONAL_SECONDS];

	/// Time format for `hh:mm:ss[.SSS]`.
	const TIME_FORMAT_SUBSEC: &[FormatItem<'_>] = {
		/// Optional subseconds.
		const OPTIONAL_SUB_SECONDS: FormatItem<'_> =
			FormatItem::Optional(&FormatItem::Compound(format_description!(".[subsecond]")));
		&[FormatItem::Compound(TIME_FORMAT), OPTIONAL_SUB_SECONDS]
	};

	/// Serialize time, using subseconds iff appropriate.
	#[allow(clippy::trivially_copy_pass_by_ref)] // Parameter types are set by serde.
	pub fn serialize<S>(time: &time::Time, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let format = if time.nanosecond() == 0 { TIME_FORMAT } else { TIME_FORMAT_SUBSEC };
		time.format(format).map_err(serde::ser::Error::custom)?.serialize(serializer)
	}

	/// Deserialize time, subseconds optional.
	pub fn deserialize<'de, D>(deserializer: D) -> Result<time::Time, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		time::Time::parse(&string, TIME_FORMAT_SUBSEC).map_err(serde::de::Error::custom)
	}
}

impl Serialize for Date {
	/// Serialize date.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self {
			// Serialize YYYY
			Date::Year(year) => {
				if (1000..10000).contains(year) {
					year.to_string().serialize(serializer)
				} else {
					Err(serde::ser::Error::custom("Year is not 4 digits long"))
				}
			}
			// Serialize YYYY-MM
			Date::YearMonth(year, month) => {
				if (1000..10000).contains(year) {
					serializer.serialize_str(&format!("{year}-{:02}", *month as u8))
				} else {
					Err(serde::ser::Error::custom("Year is not 4 digits long"))
				}
			}
			// Serialize YYYY-MM-DD
			Date::Date(date) => {
				/// Full date format
				const FORMAT: &[time::format_description::FormatItem<'_>] =
					time::macros::format_description!("[year]-[month]-[day]");
				date.format(FORMAT).map_err(serde::ser::Error::custom)?.serialize(serializer)
			}
		}
	}
}

impl FromStr for Date {
	type Err = DateFormatError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Split date into parts
		// YYYY(1)-MM(2)-DD(3)
		match s.split('-').count() {
			1 => Ok(Date::Year(s.parse::<i32>()?)),
			2 => {
				let (year, month) = s.split_once('-').ok_or(DateFormatError::StringSplit)?;
				// Convert strings into integers
				let year = year.parse::<i32>()?;
				let month = month.parse::<u8>()?;

				Ok(Date::YearMonth(year, month.try_into()?))
			}
			3 => {
				/// Full date format
				const FORMAT: &[time::format_description::FormatItem<'_>] =
					time::macros::format_description!("[year]-[month]-[day]");
				Ok(Date::Date(time::Date::parse(s, FORMAT)?))
			}
			_ => Err(DateFormatError::InvalidDate),
		}
	}
}

impl<'de> Deserialize<'de> for Date {
	/// Deserialize date.
	fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Date::from_str(&string).map_err(serde::de::Error::custom)
	}
}

impl FromStr for DateTime {
	type Err = DateFormatError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.contains('T') {
			let instant = Instant::from_str(s)?;
			Ok(DateTime::DateTime(instant))
		} else {
			let date = Date::from_str(s)?;
			Ok(DateTime::Date(date))
		}
	}
}

impl<'de> Deserialize<'de> for DateTime {
	/// Deserialize datetime.
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Self::from_str(&string).map_err(serde::de::Error::custom)
	}
}

impl FromStr for Instant {
	type Err = Parse;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Instant(OffsetDateTime::parse(s, &Rfc3339)?))
	}
}

impl std::fmt::Display for Instant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0.format(&Rfc3339).map_err(|_| std::fmt::Error)?)
	}
}

impl std::fmt::Display for Date {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Date::Year(year) => write!(f, "{year}"),
			Date::YearMonth(year, month) => write!(f, "{year}-{:02}", *month as u8),
			Date::Date(date) => {
				write!(f, "{:04}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
			}
		}
	}
}

impl std::fmt::Display for DateTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DateTime::Date(date) => write!(f, "{date}"),
			DateTime::DateTime(instant) => write!(f, "{instant}"),
		}
	}
}

impl Date {
	/// A missing month or day is `None`, which sorts before any `Some`, placing
	/// partial dates in front of the more specified values they overlap.
	/// For example: 2026 < 2026-01 < 2026-01-01
	const fn sort_key(&self) -> (i32, Option<time::Month>, Option<u8>) {
		match self {
			Date::Year(year) => (*year, None, None),
			Date::YearMonth(year, month) => (*year, Some(*month), None),
			Date::Date(date) => (date.year(), Some(date.month()), Some(date.day())),
		}
	}
}

impl Ord for Date {
	fn cmp(&self, other: &Self) -> Ordering {
		self.sort_key().cmp(&other.sort_key())
	}
}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl DateTime {
	const fn sort_key(&self) -> ((i32, Option<time::Month>, Option<u8>), Option<time::Time>) {
		match self {
			DateTime::Date(date) => (date.sort_key(), None),
			DateTime::DateTime(Instant(datetime)) => {
				let utc = datetime.to_offset(time::UtcOffset::UTC);
				((utc.year(), Some(utc.month()), Some(utc.day())), Some(utc.time()))
			}
		}
	}
}

impl Ord for DateTime {
	fn cmp(&self, other: &Self) -> Ordering {
		self.sort_key().cmp(&other.sort_key())
	}
}

impl PartialOrd for DateTime {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq<time::Date> for Date {
	fn eq(&self, other: &time::Date) -> bool {
		match self {
			Self::Year(year) => *year == other.year(),
			Self::YearMonth(year, month) => *year == other.year() && *month == other.month(),
			Self::Date(date) => date == other,
		}
	}
}

impl PartialEq<Date> for time::Date {
	fn eq(&self, other: &Date) -> bool {
		match other {
			Date::Year(year) => self.year() == *year,
			Date::YearMonth(year, month) => self.year() == *year && self.month() == *month,
			Date::Date(date) => self == date,
		}
	}
}

impl PartialOrd<time::Date> for Date {
	fn partial_cmp(&self, other: &time::Date) -> Option<Ordering> {
		match self {
			Date::Year(year) => Some(year.cmp(&other.year())),
			Date::YearMonth(year, month) => match year.cmp(&other.year()) {
				Ordering::Less => Some(Ordering::Less),
				Ordering::Greater => Some(Ordering::Greater),
				Ordering::Equal => Some((*month as u8).cmp(&(other.month() as u8))),
			},
			Date::Date(date) => Some(date.cmp(other)),
		}
	}
}

impl PartialOrd<Date> for time::Date {
	fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
		match other {
			Date::Year(year) => Some(self.year().cmp(year)),
			Date::YearMonth(year, month) => match self.year().cmp(year) {
				Ordering::Less => Some(Ordering::Less),
				Ordering::Greater => Some(Ordering::Greater),
				Ordering::Equal => Some((self.month() as u8).cmp(&(*month as u8))),
			},
			Date::Date(date) => Some(self.cmp(date)),
		}
	}
}

impl PartialEq<OffsetDateTime> for DateTime {
	fn eq(&self, other: &OffsetDateTime) -> bool {
		match self {
			Self::Date(date) => *date == other.date(),
			Self::DateTime(Instant(datetime)) => datetime == other,
		}
	}
}

impl PartialEq<DateTime> for OffsetDateTime {
	fn eq(&self, other: &DateTime) -> bool {
		match other {
			DateTime::Date(date) => self.date() == *date,
			DateTime::DateTime(Instant(datetime)) => self == datetime,
		}
	}
}

impl PartialEq<OffsetDateTime> for Date {
	fn eq(&self, other: &OffsetDateTime) -> bool {
		match self {
			Self::Year(year) => *year == other.date().year(),
			Self::YearMonth(year, month) => {
				*year == other.date().year() && *month == other.date().month()
			}
			Self::Date(date) => *date == other.date(),
		}
	}
}

impl PartialEq<Date> for OffsetDateTime {
	fn eq(&self, other: &Date) -> bool {
		match other {
			Date::Year(year) => *year == self.date().year(),
			Date::YearMonth(year, month) => {
				*year == self.date().year() && *month == self.date().month()
			}
			Date::Date(date) => *date == self.date(),
		}
	}
}

impl PartialOrd<OffsetDateTime> for DateTime {
	fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
		match self {
			DateTime::Date(date) => date.partial_cmp(&other.date()),
			DateTime::DateTime(Instant(datetime)) => Some(datetime.cmp(other)),
		}
	}
}

impl PartialOrd<DateTime> for OffsetDateTime {
	fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
		match other {
			DateTime::Date(date) => self.date().partial_cmp(date),
			DateTime::DateTime(Instant(datetime)) => Some(self.cmp(datetime)),
		}
	}
}

#[cfg(test)]
mod tests {
	use time::macros::{date, datetime};

	use super::*;

	#[test]
	fn date_ordering() {
		assert!(Date::Year(2024) < Date::Year(2025));
		assert!(Date::Year(2024) == Date::Year(2024));
		assert!(Date::Year(2024) > Date::Year(2023));

		assert!(Date::Year(2024) < Date::YearMonth(2025, time::Month::February));
		assert!(Date::Year(2024) < Date::YearMonth(2024, time::Month::February));
		assert!(Date::Year(2024) > Date::YearMonth(2023, time::Month::February));

		assert!(Date::Year(2024) < Date::Date(date!(2025 - 02 - 11)));
		assert!(Date::Year(2024) < Date::Date(date!(2024 - 02 - 11)));
		assert!(Date::Year(2024) > Date::Date(date!(2023 - 02 - 11)));

		assert!(Date::YearMonth(2024, time::Month::February) < Date::Date(date!(2024 - 03 - 11)));
		assert!(Date::YearMonth(2024, time::Month::February) < Date::Date(date!(2024 - 02 - 11)));
		assert!(Date::YearMonth(2024, time::Month::February) > Date::Date(date!(2024 - 01 - 11)));
	}

	#[test]
	fn datetime_ordering() {
		assert!(DateTime::Date(Date::Year(2024)) < DateTime::Date(Date::Year(2025)));
		assert!(DateTime::Date(Date::Year(2024)) == DateTime::Date(Date::Year(2024)));
		assert!(DateTime::Date(Date::Year(2024)) > DateTime::Date(Date::Year(2023)));

		assert!(
			DateTime::Date(Date::Year(2024))
				< DateTime::Date(Date::YearMonth(2025, time::Month::February))
		);
		assert!(
			DateTime::Date(Date::Year(2024))
				< DateTime::Date(Date::YearMonth(2024, time::Month::February))
		);
		assert!(
			DateTime::Date(Date::Year(2024))
				> DateTime::Date(Date::YearMonth(2023, time::Month::February))
		);

		assert!(
			DateTime::Date(Date::Year(2024)) < DateTime::Date(Date::Date(date!(2025 - 02 - 11)))
		);
		assert!(
			DateTime::Date(Date::Year(2024)) < DateTime::Date(Date::Date(date!(2024 - 02 - 11)))
		);
		assert!(
			DateTime::Date(Date::Year(2024)) > DateTime::Date(Date::Date(date!(2023 - 02 - 11)))
		);

		assert!(
			DateTime::Date(Date::YearMonth(2024, time::Month::February))
				< DateTime::Date(Date::Date(date!(2024 - 03 - 11)))
		);
		assert!(
			DateTime::Date(Date::YearMonth(2024, time::Month::February))
				< DateTime::Date(Date::Date(date!(2024 - 02 - 11)))
		);
		assert!(
			DateTime::Date(Date::YearMonth(2024, time::Month::February))
				> DateTime::Date(Date::Date(date!(2024 - 01 - 11)))
		);

		assert!(
			DateTime::DateTime(Instant(datetime!(2024-11-01 13:00:00 UTC)))
				> DateTime::DateTime(Instant(datetime!(2024-11-01 12:00:00 UTC)))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-11-01 13:00:00 UTC)))
				== DateTime::DateTime(Instant(datetime!(2024-11-01 13:00:00 UTC)))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-11-01 13:00:00 UTC)))
				< DateTime::DateTime(Instant(datetime!(2024-11-01 14:00:00 UTC)))
		);

		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				> DateTime::Date(Date::Year(2023))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				> DateTime::Date(Date::Year(2024))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				< DateTime::Date(Date::Year(2025))
		);

		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				> DateTime::Date(Date::YearMonth(2024, time::Month::January))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				> DateTime::Date(Date::YearMonth(2024, time::Month::February))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				< DateTime::Date(Date::YearMonth(2024, time::Month::March))
		);

		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				> DateTime::Date(Date::Date(date!(2024 - 02 - 10)))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				> DateTime::Date(Date::Date(date!(2024 - 02 - 11)))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				< DateTime::Date(Date::Date(date!(2024 - 02 - 12)))
		);
	}

	#[test]
	fn displays_date() {
		assert_eq!(Date::Year(2024).to_string(), "2024");
		assert_eq!(Date::YearMonth(2024, time::Month::March).to_string(), "2024-03");
		assert_eq!(Date::YearMonth(2024, time::Month::December).to_string(), "2024-12");
		assert_eq!(Date::Date(date!(2024 - 03 - 07)).to_string(), "2024-03-07");
	}

	#[test]
	fn displays_instant() {
		assert_eq!(Instant(datetime!(2024 - 03 - 07 12:34:56 UTC)).to_string(), "2024-03-07T12:34:56Z");
		assert_eq!(
			Instant(datetime!(2024 - 03 - 07 12:34:56.789 UTC)).to_string(),
			"2024-03-07T12:34:56.789Z"
		);
		assert_eq!(
			Instant(datetime!(2024 - 03 - 07 12:34:56 +2)).to_string(),
			"2024-03-07T12:34:56+02:00"
		);
	}

	#[test]
	fn displays_datetime() {
		assert_eq!(DateTime::Date(Date::Year(2024)).to_string(), "2024");
		assert_eq!(DateTime::Date(Date::YearMonth(2024, time::Month::March)).to_string(), "2024-03");
		assert_eq!(DateTime::Date(Date::Date(date!(2024 - 03 - 07))).to_string(), "2024-03-07");
		assert_eq!(
			DateTime::DateTime(Instant(datetime!(2024 - 03 - 07 12:34:56 UTC))).to_string(),
			"2024-03-07T12:34:56Z"
		);
	}

	#[test]
	fn datetime_round_trip() {
		for input in [
			"2024",
			"2024-03",
			"2024-03-07",
			"2024-03-07T12:34:56Z",
			"2024-03-07T12:34:56.789Z",
			"2024-03-07T12:34:56+02:00",
		] {
			assert_eq!(DateTime::from_str(input).unwrap().to_string(), input);
		}
	}

	/// Sample datetimes covering both precisions and offsets, for exhaustive
	/// pairwise checks of the ordering invariants.
	fn sample_datetimes() -> Vec<DateTime> {
		[
			"2023",
			"2024",
			"2024-01",
			"2024-03",
			"2024-03-07",
			"2024-03-08",
			"2024-03-07T00:00:00Z",
			"2024-03-07T12:34:56Z",
			"2024-03-07T12:34:56.789Z",
			"2024-03-07T23:00:00Z",
			"2024-03-08T00:30:00+02:00",
			"2024-03-08T09:00:00+02:00",
		]
		.iter()
		.map(|s| DateTime::from_str(s).unwrap())
		.collect()
	}

	#[test]
	fn datetime_order_agrees_with_eq() {
		for a in sample_datetimes() {
			for b in sample_datetimes() {
				assert_eq!(
					a.cmp(&b) == Ordering::Equal,
					a == b,
					"{a} vs {b}: cmp {:?}, eq {}",
					a.cmp(&b),
					a == b
				);
			}
		}
	}

	#[test]
	fn datetime_order_is_total() {
		for a in sample_datetimes() {
			for b in sample_datetimes() {
				assert_eq!(a.cmp(&b), b.cmp(&a).reverse(), "{a} vs {b} not antisymmetric");
				for c in sample_datetimes() {
					if a <= b && b <= c {
						assert!(a <= c, "{a} <= {b} <= {c} but {a} > {c}");
					}
				}
			}
		}
	}

	#[test]
	fn datetime_sorts_before() {
		for (less, greater) in [
			// Less precise sorts before the values it overlaps.
			("2024", "2024-01"),
			("2024-03", "2024-03-07"),
			("2024-03-07", "2024-03-07T00:00:00Z"),
			// Chronological where precision allows.
			("2023", "2024"),
			("2024-01", "2024-03"),
			("2024-03-07", "2024-03-08"),
			// Offsets normalise to UTC: 00:30+02:00 is 22:30Z, before 23:00Z.
			("2024-03-08T00:30:00+02:00", "2024-03-07T23:00:00Z"),
		] {
			let less = DateTime::from_str(less).unwrap();
			let greater = DateTime::from_str(greater).unwrap();
			assert!(less < greater, "expected {less} < {greater}");
		}
	}

	#[test]
	fn equal_instants_in_different_offsets_are_equal() {
		let utc = DateTime::from_str("2024-03-07T22:30:00Z").unwrap();
		let plus_two = DateTime::from_str("2024-03-08T00:30:00+02:00").unwrap();
		assert_eq!(utc, plus_two);
		assert_eq!(utc.cmp(&plus_two), Ordering::Equal);
	}
}
