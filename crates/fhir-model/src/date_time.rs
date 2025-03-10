//! FHIR Time, Date, DateTime and Instant types.

use std::cmp::Ordering;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use time::error::Parse;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::error::DateFormatError;

/// FHIR instant type: <https://hl7.org/fhir/datatypes.html#instant>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Instant(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);

/// FHIR date type: <https://hl7.org/fhir/datatypes.html#date>
#[derive(Debug, Clone, Eq, Hash)]
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
#[derive(Debug, Clone, Eq, Hash, Serialize)]
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
	use time::format_description::FormatItem;
	use time::macros::format_description;

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

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Date {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Date::Date(ld), r) => ld.partial_cmp(r).unwrap(),
			(l, Date::Date(rd)) => l.partial_cmp(rd).unwrap(),
			(Date::Year(ly), Date::Year(ry)) => ly.cmp(ry),
			(Date::Year(ly), Date::YearMonth(ry, _rm)) => ly.cmp(ry),
			(Date::YearMonth(ly, _lm), Date::Year(ry)) => ly.cmp(ry),
			(Date::YearMonth(ly, lm), Date::YearMonth(ry, rm)) => match ly.cmp(ry) {
				Ordering::Equal => (*lm as u8).cmp(&(*rm as u8)),
				other => other,
			},
		}
	}
}

impl PartialOrd for DateTime {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for DateTime {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(DateTime::Date(ld), DateTime::Date(rd)) => ld.cmp(rd),
			(DateTime::Date(ld), DateTime::DateTime(Instant(rdtm))) => {
				ld.partial_cmp(&rdtm.date()).unwrap()
			}
			(DateTime::DateTime(Instant(ldtm)), DateTime::Date(rd)) => {
				ldtm.date().partial_cmp(rd).unwrap()
			}
			(DateTime::DateTime(ldtm), DateTime::DateTime(rdtm)) => ldtm.cmp(rdtm),
		}
	}
}

impl PartialEq for DateTime {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(DateTime::Date(ld), DateTime::Date(rd)) => ld.eq(rd),
			(DateTime::Date(ld), DateTime::DateTime(Instant(rdtm))) => ld.eq(rdtm),
			(DateTime::DateTime(Instant(ldtm)), DateTime::Date(rd)) => ldtm.eq(rd),
			(DateTime::DateTime(Instant(ldtm)), DateTime::DateTime(Instant(rdtm))) => ldtm.eq(rdtm),
		}
	}
}

impl PartialEq for Date {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Date::Date(ld), r) => ld.eq(r),
			(l, Date::Date(rd)) => l.eq(rd),
			(Date::Year(ly), Date::Year(ry)) => ly.eq(ry),
			(Date::Year(ly), Date::YearMonth(ry, _rm)) => ly.eq(ry),
			(Date::YearMonth(ly, _lm), Date::Year(ry)) => ly.eq(ry),
			(Date::YearMonth(ly, lm), Date::YearMonth(ry, rm)) => ly.eq(ry) && lm.eq(rm),
		}
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
		assert!(Date::Year(2024) == Date::YearMonth(2024, time::Month::February));
		assert!(Date::Year(2024) > Date::YearMonth(2023, time::Month::February));

		assert!(Date::Year(2024) < Date::Date(date!(2025 - 02 - 11)));
		assert!(Date::Year(2024) == Date::Date(date!(2024 - 02 - 11)));
		assert!(Date::Year(2024) > Date::Date(date!(2023 - 02 - 11)));

		assert!(Date::YearMonth(2024, time::Month::February) < Date::Date(date!(2024 - 03 - 11)));
		assert!(Date::YearMonth(2024, time::Month::February) == Date::Date(date!(2024 - 02 - 11)));
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
				== DateTime::Date(Date::YearMonth(2024, time::Month::February))
		);
		assert!(
			DateTime::Date(Date::Year(2024))
				> DateTime::Date(Date::YearMonth(2023, time::Month::February))
		);

		assert!(
			DateTime::Date(Date::Year(2024)) < DateTime::Date(Date::Date(date!(2025 - 02 - 11)))
		);
		assert!(
			DateTime::Date(Date::Year(2024)) == DateTime::Date(Date::Date(date!(2024 - 02 - 11)))
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
				== DateTime::Date(Date::Date(date!(2024 - 02 - 11)))
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
				== DateTime::Date(Date::Year(2024))
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
				== DateTime::Date(Date::YearMonth(2024, time::Month::February))
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
				== DateTime::Date(Date::Date(date!(2024 - 02 - 11)))
		);
		assert!(
			DateTime::DateTime(Instant(datetime!(2024-02-11 13:00:00 UTC)))
				< DateTime::Date(Date::Date(date!(2024 - 02 - 12)))
		);
	}
}
