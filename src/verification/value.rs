//! Semantic value validation functions for AAS types.

use crate::DataTypeDefXsd;

use super::pattern::{self, RE_DATE_PREFIX, RE_LONG, RE_UNSIGNED_LONG};

const SMALLEST_LONG_WITHOUT_SIGN: &str = "9223372036854775808";
const LARGEST_LONG: &str = "9223372036854775807";
const LARGEST_UNSIGNED_LONG: &str = "18446744073709551615";

const DAYS_IN_MONTH: [u8; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Check if `year` is a leap year.
///
/// Year 1 BCE is the last leap BCE year.
/// See <https://www.w3.org/TR/xmlschema-2/#dateTime>.
pub fn is_leap_year(mut year: i64) -> bool {
    if year < 0 {
        year = -year - 1;
    }
    if year % 4 > 0 {
        return false;
    }
    if year % 100 > 0 {
        return true;
    }
    if year % 400 > 0 {
        return false;
    }
    true
}

/// Compute leap year from year string digits (for years that overflow i64).
///
/// Extracts last 3 digits and sign to determine divisibility by 4/100/400.
fn is_leap_year_from_str(year_str: &str) -> bool {
    let (negative, digits) = if let Some(rest) = year_str.strip_prefix('-') {
        (true, rest)
    } else {
        (false, year_str)
    };

    // Get last 3 digits for mod 400, last 2 for mod 100, last 1 for mod 4.
    let suffix_400: u32 = if digits.len() >= 3 {
        digits[digits.len() - 3..].parse().unwrap_or(0)
    } else {
        digits.parse().unwrap_or(0)
    };

    // For BCE years: adjusted = abs(year) - 1
    let adjusted = if negative {
        // For mod operations: (abs_year - 1) % n = (suffix - 1) % n
        if suffix_400 == 0 {
            399
        } else {
            suffix_400 - 1
        }
    } else {
        suffix_400
    };

    if adjusted % 4 != 0 {
        return false;
    }
    let mod100 = adjusted % 100;
    if mod100 != 0 {
        return true;
    }
    adjusted % 400 == 0
}

/// Check that `value` is a valid xs:date.
pub fn is_xs_date(value: &str) -> bool {
    if !pattern::matches_xs_date(value) {
        return false;
    }

    let caps = match RE_DATE_PREFIX.captures(value) {
        Some(c) => c,
        None => return false,
    };

    let year_str = &caps[1];
    let month: u8 = match caps[2].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let day: u8 = match caps[3].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };

    // Year zero is not valid per XSD spec.
    if year_str == "0000" || year_str == "-0000" {
        return false;
    }

    if day == 0 {
        return false;
    }
    if month == 0 || month > 12 {
        return false;
    }

    let max_days = if month == 2 {
        let leap = match year_str.parse::<i64>() {
            Ok(y) => is_leap_year(y),
            Err(_) => is_leap_year_from_str(year_str),
        };
        if leap {
            29
        } else {
            28
        }
    } else {
        DAYS_IN_MONTH[month as usize]
    };

    day <= max_days
}

/// Check that `value` is a valid xs:dateTime.
pub fn is_xs_date_time(value: &str) -> bool {
    if !pattern::matches_xs_date_time(value) {
        return false;
    }
    let date = value.split('T').next().unwrap_or("");
    is_xs_date(date)
}

/// Check that `value` is a valid xs:dateTime with UTC timezone.
pub fn is_xs_date_time_utc(value: &str) -> bool {
    if !pattern::matches_xs_date_time_utc(value) {
        return false;
    }
    let date = value.split('T').next().unwrap_or("");
    is_xs_date(date)
}

/// Check that `value` is a valid xs:double.
pub fn is_xs_double(value: &str) -> bool {
    if !pattern::matches_xs_double(value) {
        return false;
    }
    if value == "INF" || value == "-INF" || value == "NaN" {
        return true;
    }
    match value.parse::<f64>() {
        Ok(v) => v.is_finite(),
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:float.
pub fn is_xs_float(value: &str) -> bool {
    if !pattern::matches_xs_float(value) {
        return false;
    }
    if value == "INF" || value == "-INF" || value == "NaN" {
        return true;
    }
    match value.parse::<f64>() {
        Ok(v) => {
            if !v.is_finite() {
                return false;
            }
            (v as f32).is_finite()
        }
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:gMonthDay.
pub fn is_xs_g_month_day(value: &str) -> bool {
    if !pattern::matches_xs_g_month_day(value) {
        return false;
    }
    if value.len() < 7 {
        return false;
    }
    let month: u8 = match value[2..4].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let day: u8 = match value[5..7].parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    if month == 0 || month > 12 {
        return false;
    }
    day <= DAYS_IN_MONTH[month as usize]
}

/// Check that `value` is a valid xs:long (64-bit signed integer).
pub fn is_xs_long(value: &str) -> bool {
    let caps = match RE_LONG.captures(value) {
        Some(c) => c,
        None => return false,
    };

    let sign = caps.get(1).map_or("", |m| m.as_str());
    let number_part = match caps.get(2) {
        Some(m) => m.as_str(),
        None => return false,
    };

    let limit = if sign == "-" {
        SMALLEST_LONG_WITHOUT_SIGN
    } else {
        LARGEST_LONG
    };

    if number_part.len() < limit.len() {
        return true;
    }
    if number_part.len() > limit.len() {
        return false;
    }

    for (a, b) in number_part.bytes().zip(limit.bytes()) {
        if a > b {
            return false;
        } else if a < b {
            return true;
        }
    }
    true
}

/// Check that `value` is a valid xs:int (32-bit signed integer).
pub fn is_xs_int(value: &str) -> bool {
    if !pattern::matches_xs_int(value) {
        return false;
    }
    match value.parse::<i64>() {
        Ok(v) => (-2_147_483_648..=2_147_483_647).contains(&v),
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:short (16-bit signed integer).
pub fn is_xs_short(value: &str) -> bool {
    if !pattern::matches_xs_short(value) {
        return false;
    }
    match value.parse::<i64>() {
        Ok(v) => (-32768..=32767).contains(&v),
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:byte (8-bit signed integer).
pub fn is_xs_byte(value: &str) -> bool {
    if !pattern::matches_xs_byte(value) {
        return false;
    }
    match value.parse::<i64>() {
        Ok(v) => (-128..=127).contains(&v),
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:unsignedLong (64-bit unsigned integer).
pub fn is_xs_unsigned_long(value: &str) -> bool {
    let caps = match RE_UNSIGNED_LONG.captures(value) {
        Some(c) => c,
        None => return false,
    };

    let number_part = match caps.get(2) {
        Some(m) => m.as_str(),
        None => return true, // "-0" or "+0" case
    };

    if number_part.len() < LARGEST_UNSIGNED_LONG.len() {
        return true;
    }
    if number_part.len() > LARGEST_UNSIGNED_LONG.len() {
        return false;
    }

    for (a, b) in number_part.bytes().zip(LARGEST_UNSIGNED_LONG.bytes()) {
        if a > b {
            return false;
        } else if a < b {
            return true;
        }
    }
    true
}

/// Check that `value` is a valid xs:unsignedInt (32-bit unsigned integer).
pub fn is_xs_unsigned_int(value: &str) -> bool {
    if !pattern::matches_xs_unsigned_int(value) {
        return false;
    }
    match value.parse::<i64>() {
        Ok(v) => (0..=4_294_967_295).contains(&v),
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:unsignedShort (16-bit unsigned integer).
pub fn is_xs_unsigned_short(value: &str) -> bool {
    if !pattern::matches_xs_unsigned_short(value) {
        return false;
    }
    match value.parse::<i64>() {
        Ok(v) => (0..=65535).contains(&v),
        Err(_) => false,
    }
}

/// Check that `value` is a valid xs:unsignedByte (8-bit unsigned integer).
pub fn is_xs_unsigned_byte(value: &str) -> bool {
    if !pattern::matches_xs_unsigned_byte(value) {
        return false;
    }
    match value.parse::<i64>() {
        Ok(v) => (0..=255).contains(&v),
        Err(_) => false,
    }
}

/// Check that `value` is consistent with the given XSD type.
pub fn value_consistent_with_xsd_type(value: &str, value_type: DataTypeDefXsd) -> bool {
    match value_type {
        DataTypeDefXsd::AnyUri => pattern::matches_xs_any_uri(value),
        DataTypeDefXsd::Base64Binary => pattern::matches_xs_base64_binary(value),
        DataTypeDefXsd::Boolean => pattern::matches_xs_boolean(value),
        DataTypeDefXsd::Byte => is_xs_byte(value),
        DataTypeDefXsd::Date => is_xs_date(value),
        DataTypeDefXsd::DateTime => is_xs_date_time(value),
        DataTypeDefXsd::Decimal => pattern::matches_xs_decimal(value),
        DataTypeDefXsd::Double => is_xs_double(value),
        DataTypeDefXsd::Duration => pattern::matches_xs_duration(value),
        DataTypeDefXsd::Float => is_xs_float(value),
        DataTypeDefXsd::GDay => pattern::matches_xs_g_day(value),
        DataTypeDefXsd::GMonth => pattern::matches_xs_g_month(value),
        DataTypeDefXsd::GMonthDay => is_xs_g_month_day(value),
        DataTypeDefXsd::GYear => pattern::matches_xs_g_year(value),
        DataTypeDefXsd::GYearMonth => pattern::matches_xs_g_year_month(value),
        DataTypeDefXsd::HexBinary => pattern::matches_xs_hex_binary(value),
        DataTypeDefXsd::Int => is_xs_int(value),
        DataTypeDefXsd::Integer => pattern::matches_xs_integer(value),
        DataTypeDefXsd::Long => is_xs_long(value),
        DataTypeDefXsd::NegativeInteger => pattern::matches_xs_negative_integer(value),
        DataTypeDefXsd::NonNegativeInteger => pattern::matches_xs_non_negative_integer(value),
        DataTypeDefXsd::NonPositiveInteger => pattern::matches_xs_non_positive_integer(value),
        DataTypeDefXsd::PositiveInteger => pattern::matches_xs_positive_integer(value),
        DataTypeDefXsd::Short => is_xs_short(value),
        DataTypeDefXsd::String => pattern::matches_xs_string(value),
        DataTypeDefXsd::Time => pattern::matches_xs_time(value),
        DataTypeDefXsd::UnsignedByte => is_xs_unsigned_byte(value),
        DataTypeDefXsd::UnsignedInt => is_xs_unsigned_int(value),
        DataTypeDefXsd::UnsignedLong => is_xs_unsigned_long(value),
        DataTypeDefXsd::UnsignedShort => is_xs_unsigned_short(value),
    }
}
