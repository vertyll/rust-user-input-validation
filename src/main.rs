use regex::Regex;
use std::io;
use std::str::FromStr;

struct User {
    name: String,
    email: String,
    age: u32,
}

impl User {
    fn new(name: impl Into<String>, email: impl Into<String>, age: u32) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            age,
        }
    }
}

struct ValidationMethods;

impl ValidationMethods {
    fn validate_name(name: &str) -> bool {
        !name.chars().any(|c| c.is_numeric())
    }

    fn validate_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
        email_regex.is_match(email)
    }

    fn not_empty(value: &str) -> bool {
        !value.is_empty()
    }
}

struct Validator {
    validations: Vec<fn(&str) -> bool>,
}

impl Validator {
    fn new(validations: Vec<fn(&str) -> bool>) -> Self {
        Self { validations }
    }

    fn validate(&self, input: &str) -> bool {
        self.validations.iter().all(|validation| validation(input))
    }
}

macro_rules! validator_factory {
    ($($validation:ident),*) => {
        Validator::new(vec![$(ValidationMethods::$validation),*])
    };
}

fn main() {
    let name: String = read_input("Enter name:", &validator_factory!(not_empty, validate_name));
    let email: String = read_input(
        "Enter email:",
        &validator_factory!(not_empty, validate_email),
    );
    let age: u32 = read_input("Enter age:", &validator_factory!(not_empty));

    let user = User::new(name, email, age);

    println!(
        "Name: {}, Email: {}, Age: {}",
        user.name, user.email, user.age
    );
}

fn read_input<T>(prompt: &str, validator: &Validator) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let stdin = io::stdin();

    loop {
        println!("{}", prompt);

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read input");

        let input = buffer.trim();

        if let Ok(value) = input.parse::<T>() {
            if validator.validate(input) {
                return value;
            } else {
                println!("Invalid input, please try again.");
            }
        } else {
            println!("Failed to convert value, please try again.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;
    use std::io::Cursor;

    #[test]
    fn test_validate_name() {
        assert!(ValidationMethods::validate_name("John"));
        assert!(!ValidationMethods::validate_name("John123"));
    }

    #[test]
    fn test_validate_email() {
        assert!(ValidationMethods::validate_email("test@example.com"));
        assert!(!ValidationMethods::validate_email("invalid-email"));
    }

    #[test]
    fn test_not_empty() {
        assert!(ValidationMethods::not_empty("not empty"));
        assert!(!ValidationMethods::not_empty(""));
    }

    #[test]
    fn test_validator() {
        let validator = validator_factory!(not_empty, validate_name);
        assert!(validator.validate("John"));
        assert!(!validator.validate("John123"));
        assert!(!validator.validate(""));
    }

    #[test]
    fn test_read_input() {
        let input = b"John\n";
        let mut cursor = Cursor::new(&input[..]);

        let validator = validator_factory!(not_empty, validate_name);
        let result: String = read_input_with_cursor("Enter name:", &validator, &mut cursor);
        assert_eq!(result, "John");
    }

    fn read_input_with_cursor<T>(
        prompt: &str,
        validator: &Validator,
        cursor: &mut Cursor<&[u8]>,
    ) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        loop {
            println!("{}", prompt);

            let mut buffer = String::new();
            cursor.read_line(&mut buffer).expect("Failed to read input");

            let input = buffer.trim();

            if let Ok(value) = input.parse::<T>() {
                if validator.validate(input) {
                    return value;
                } else {
                    println!("Invalid input, please try again.");
                }
            } else {
                println!("Failed to convert value, please try again.");
            }
        }
    }
}
