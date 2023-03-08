use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

use dialoguer::theme::Theme;
use dialoguer::{Input, Validator};

/// A value which can prompt a user for a value.
///
/// This includes basic validation to ensure
/// the type is correct and can apply
/// additional validation if needed.
pub trait PromptValue<'a, V>
where
    Self: Into<serde_json::Value>,
    V: Validator<Self> + 'a,
    V::Err: Display,
{
    fn prompt(
        theme: &dyn Theme,
        field_name: impl Display,
        validator: Option<V>,
        can_skip: bool,
    ) -> io::Result<Option<Self>>;
}

impl<'a, V> PromptValue<'a, V> for String
where
    V: Validator<Self> + 'a,
    V::Err: Display,
{
    fn prompt(
        theme: &dyn Theme,
        field_name: impl Display,
        mut validator: Option<V>,
        can_skip: bool,
    ) -> io::Result<Option<Self>> {
        Input::with_theme(theme)
            .with_prompt(field_name.to_string())
            .validate_with(|input: &String| -> Result<(), String> {
                if can_skip && input.is_empty() {
                    return Ok(());
                }

                if let Some(validator) = validator.as_mut() {
                    validator.validate(input).map_err(|e| e.to_string())
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .map(|input| {
                if can_skip && input.is_empty() {
                    None
                } else {
                    Some(input)
                }
            })
    }
}

pub trait TraitIntBounds: PartialOrd + Debug + Display {
    fn max() -> Self;
    fn min() -> Self;
}

impl TraitIntBounds for usize {
    fn max() -> Self {
        Self::MAX
    }

    fn min() -> Self {
        Self::MIN
    }
}

fn maybe_parse_value<V>(can_skip: bool, input: String) -> Option<V>
where
    V: FromStr,
    V::Err: Debug,
{
    if can_skip && input.is_empty() {
        return None;
    } else {
        Some(input.parse::<V>().unwrap())
    }
}

macro_rules! parse_primitives {
    ($tp:ty, $msg:expr) => {
        impl TraitIntBounds for $tp {
            fn max() -> Self {
                <$tp>::MAX
            }

            fn min() -> Self {
                <$tp>::MIN
            }
        }

        impl<'a, V> PromptValue<'a, V> for $tp
        where
            V: Validator<Self> + 'a,
            V::Err: Display,
        {
            fn prompt(
                theme: &dyn Theme,
                field_name: impl Display,
                mut validator: Option<V>,
                can_skip: bool,
            ) -> io::Result<Option<Self>> {
                Input::with_theme(theme)
                    .with_prompt(field_name.to_string())
                    .validate_with(|input: &String| -> Result<(), String> {
                        if can_skip && input.is_empty() {
                            return Ok(());
                        }

                        let value = input
                            .parse::<Self>()
                            .map_err(|_| format!("Value ({input}) {}.", $msg))?;

                        if let Some(validator) = validator.as_mut() {
                            validator.validate(&value).map_err(|e| e.to_string())
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()
                    .map(|input| maybe_parse_value(can_skip, input))
            }
        }
    };
}

parse_primitives!(u64, "is not a valid positive number.");
parse_primitives!(u32, "is not a valid positive 32-bit number.");
parse_primitives!(u16, "is not a valid positive 16-bit number.");
parse_primitives!(u8, "is not a valid positive 8-bit number.");
parse_primitives!(i64, "is not a valid number.");
parse_primitives!(i32, "is not a valid 32-bit number.");
parse_primitives!(i16, "is not a valid 16-bit number.");
parse_primitives!(i8, "is not a valid 8-bit number.");
parse_primitives!(f64, "is not a valid float.");
parse_primitives!(f32, "is not a valid 32-bit float.");
