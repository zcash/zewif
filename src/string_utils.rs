//! Utility functions for formatting numerical values as human-readable strings.
//!
//! This module provides helper functions for converting raw numeric values to
//! formatted strings, primarily for display purposes. It includes functions
//! for formatting large numbers with underscores as separators and for
//! converting Zcash amounts between zatoshi and ZEC representations.

/// Formats a number with underscores as thousand separators for improved readability.
///
/// This function takes a numeric value and formats it with underscores between
/// each group of three digits to improve readability of large numbers. For example,
/// `1000000` becomes `1_000_000`.
///
/// # Arguments
/// * `amount` - Any value that can be converted to u64
///
/// # Returns
/// A string with the formatted number
///
/// # Examples
/// ```
/// # use zewif::format_with_underscores;
/// #
/// assert_eq!(format_with_underscores(1000_u64), "1_000");
/// assert_eq!(format_with_underscores(1234567_u64), "1_234_567");
/// assert_eq!(format_with_underscores(1_u64), "1");
/// ```
pub fn format_with_underscores(amount: impl Into<u64>) -> String {
    let s = amount.into().to_string();
    let mut result = String::new();

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('_');
        }
        result.push(c);
    }

    result.chars().rev().collect()
}

/// Formats an amount in zatoshi (smallest Zcash unit) as a human-readable ZEC value.
///
/// This function converts a zatoshi amount to a ZEC amount with the proper decimal places.
/// There are 100,000,000 zatoshi in 1 ZEC, similar to how there are 100,000,000 satoshi
/// in 1 Bitcoin.
///
/// # Arguments
/// * `amount` - A zatoshi amount that can be converted to u64
///
/// # Returns
/// A string with the formatted ZEC amount, with the prefix "ZEC" (e.g., "ZEC 1.23456789")
///
/// # Examples
/// ```
/// # use zewif::format_zats_as_zec;
/// #
/// // 1 ZEC = 100_000_000 zatoshi
/// assert_eq!(format_zats_as_zec(100_000_000_u64), "ZEC 1.0");
///
/// // Format partial ZEC amounts
/// assert_eq!(format_zats_as_zec(123_456_789_u64), "ZEC 1.23456789");
///
/// // Trailing zeros are trimmed
/// assert_eq!(format_zats_as_zec(100_000_000_000_u64), "ZEC 1000.0");
/// ```
pub fn format_zats_as_zec(amount: impl Into<u64>) -> String {
    let amount = amount.into();
    let integer = amount / 100_000_000;
    let fraction = amount % 100_000_000;
    if fraction == 0 {
        return format!("ZEC {integer}.0");
    }
    // Format fractional part with leading zeros, then remove trailing zeros.
    let fraction_str = format!("{fraction:08}");
    let trimmed_fraction = fraction_str.trim_end_matches('0');
    format!("ZEC {integer}.{trimmed_fraction}")
}
