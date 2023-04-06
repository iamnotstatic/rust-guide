use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
enum RgbError {
    #[error("Hex colors must begin with a hash")]
    MissingHash,
    #[error("Failed to parse hex digit: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Invalid hex color length (must be 6)")]
    LengthError,
}

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

impl TryFrom<&str> for Rgb {
    type Error = RgbError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if !hex.starts_with('#') {
            return Err(RgbError::MissingHash);
        }

        if hex.len() != 7 {
            return Err(RgbError::LengthError);
        }

        let (r, g, b) = (
            u8::from_str_radix(&hex[1..=2], 16)?,
            u8::from_str_radix(&hex[3..=4], 16)?,
            u8::from_str_radix(&hex[5..=6], 16)?,
        );

        Ok(Self(r, g, b))
    }
}

// impl From<std::num::ParseIntError> for RgbError {
//     fn from(err: std::num::ParseIntError) -> Self {
//         Self::ParseError(err)
//     }
// }

fn main() {
    println!("Hello !World");
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert_eq!(
            Rgb::try_from("#0011f").is_err(),
            true,
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert_eq!(
            Rgb::try_from("#0011ffa").is_err(),
            true,
            "should be an error when too many color components are provided"
        );
    }
}
