// PyO3 wrapper functions for utility functions
use pyo3::prelude::*;
use crate::classes::utils::MtgjsonUtils;

/// Convert snake_case to camelCase
#[pyfunction]
pub fn to_camel_case(snake_str: &str) -> String {
    to_camel_case(snake_str)
}

/// Make filename Windows-safe
#[pyfunction]
pub fn make_windows_safe_filename(filename: &str) -> String {
    MtgjsonUtils::make_windows_safe_filename(filename)
}

/// Clean card number for sorting
#[pyfunction]
pub fn clean_card_number(number: &str) -> (String, usize) {
    let (num, len) = MtgjsonUtils::clean_card_number(number);
    (num.to_string(), len)
}