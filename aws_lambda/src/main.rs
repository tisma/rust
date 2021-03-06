#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use serde_derive::{Serialize, Deserialize};
use lambda::{lambda, Context, error::HandlerError};

fn validate_serial_length(serial_number: &str) -> bool {
    serial_number.chars().count() >= 6
}

fn validate_serial_alphanumeric(serial_number: &str) -> bool {
    serial_number.chars().all(char::is_alphanumeric)
}

fn validate_serial_unique(serial_number: &str) -> bool {
    let existing_serial_numbers = vec!["serial1", "serial2", "serial3"];
    !existing_serial_numbers.contains(&serial_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_length_of_four_characters_as_invalid() {
        let test_serial = "1234";
        let validation_result = validate_serial_length(test_serial);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_length_of_six_characters_as_valid() {
        let test_serial = "i23456";
        let validation_result = validate_serial_length(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_length_of_ten_characters_as_valid() {
        let test_serial = "i234567891";
        let validation_result = validate_serial_length(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_numbers_as_valid() {
        let test_serial = "234567891";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_az_characters_as_valid() {
        let test_serial = "abcd1234";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_unicode_characters_as_valid() {
        let test_serial = "абвгдћжњђЋ1234";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(true, validation_result);
    }

    #[test]
    fn validates_string_with_special_characters_as_valid() {
        let test_serial = "abcd!1234";
        let validation_result = validate_serial_alphanumeric(test_serial);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_existing_serial1_as_invalid() {
        let test_serial = "serial1";
        let validation_result = validate_serial_unique(test_serial);
        assert_eq!(false, validation_result);
    }

    #[test]
    fn validates_new_serial4_as_valid() {
        let test_serial = "serial4";
        let validation_result = validate_serial_unique(test_serial);
        assert_eq!(true, validation_result);
    }
}

enum ValidationError {
    InvalidFormat,
    AlreadyExists
}

impl ValidationError {
    fn value(&self) -> String {
        match *self {
            ValidationError::InvalidFormat => String::from("invalid_format"),
            ValidationError::AlreadyExists => String::from("already_exists"),

        }
    }
}

fn validate_serial(serial_number: &str) -> ValidationResult {
    let mut result = ValidationResult { is_valid: true, errors: Vec::new() };

    if !validate_serial_length(serial_number) {
        result.is_valid = false;
        result.errors.push(ValidationError::InvalidFormat.value());
    }

    if !validate_serial_alphanumeric(serial_number) {
        result.is_valid = false;
        result.errors.push(ValidationError::InvalidFormat.value());
    }

    if !validate_serial_unique(serial_number) {
        result.is_valid = false;
        result.errors.push(ValidationError::AlreadyExists.value());
    }

    return result;
}

#[derive(Serialize, Deserialize)]
struct ValidationResult {
    #[serde(rename = "isValid")]
    is_valid: bool,
    errors: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct ValidationEvent {
    #[serde(rename = "serialNumber")]
    serial_number: String
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(validation_handler);
    Ok(())
}

fn validation_handler(event: ValidationEvent, ctx: Context) -> Result<ValidationResult, HandlerError> {
    Ok(validate_serial(event.serial_number.as_str()))
}

