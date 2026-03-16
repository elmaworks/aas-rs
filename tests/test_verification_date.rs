//! Tests for date-related verification functions.
//! Mirrors aas-core3.0-typescript/test/verification.date.spec.ts

use aas_rs::verification::is_leap_year;

#[test]
fn test_leap_years_ce() {
    assert!(is_leap_year(2000));
    assert!(is_leap_year(1600));
    assert!(is_leap_year(1604));

    assert!(!is_leap_year(1700));
    assert!(!is_leap_year(1800));
}

#[test]
fn test_leap_years_bce() {
    // Year 0 in astronomical year numbering = 1 BCE (proleptic Gregorian)
    assert!(is_leap_year(-1));
    assert!(is_leap_year(-5));

    assert!(!is_leap_year(-4));
}
