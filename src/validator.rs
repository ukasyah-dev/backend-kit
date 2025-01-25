use validator::Validate;

use crate::error;

pub fn validate<T: Validate>(value: &T) -> Result<(), error::Error> {
    if let Err(err) = value.validate() {
        if let Some((field, errors)) = err.field_errors().iter().next() {
            let message = errors.first().map(|e| e.to_string()).unwrap_or_default();

            return Err(error::invalid_argument_with_message(
                format!("{}: {}", field, message).as_str(),
            ));
        }
    }
    Ok(())
}
