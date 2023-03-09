use dialoguer::Validator;
use indexmap::IndexMap;
use regex::Regex;

use crate::value::TraitIntBounds;
use crate::Field;

#[derive(serde::Deserialize, Clone, Copy)]
/// The constraints for collection types (array, set, hashmap, etc...)
pub struct CollectionConstraints {
    #[serde(default)]
    /// The minimum number of the items.
    pub min_items: usize,
    #[serde(default = "<usize as TraitIntBounds>::max")]
    /// The maximum number of the items.
    pub max_items: usize,
}

impl Default for CollectionConstraints {
    fn default() -> Self {
        Self {
            min_items: 0,
            max_items: usize::MAX,
        }
    }
}

#[derive(serde::Deserialize, Clone, Default)]
/// The constraints for the select type.
pub struct SelectConstraints {
    #[serde(default)]
    /// Can several items be selected at once.
    pub select_many: bool,
    /// The items that can be selected.
    pub items: Vec<serde_json::Value>,
}

#[derive(serde::Deserialize, Default)]
/// Prompts which are triggered on value selection.
pub struct Conditions {
    #[serde(default)]
    /// Should the object be inserted at the root of the data or not.
    ///
    /// If this is false (default) it will be inserted in a nested fashion.
    pub insert_at_root: bool,

    #[serde(rename = "if")]
    /// If conditions if a value is picked.
    ///
    /// Not triggered when value skipped.
    pub if_conditions: Vec<IfCondition>,
}

#[derive(serde::Deserialize)]
pub struct IfCondition {
    /// The trigger value.
    pub picked: serde_json::Value,
    /// The prompt fields to trigger.
    pub fields: IndexMap<String, Field>,
}

#[derive(serde::Deserialize, Clone)]
/// The constraints for string types.
pub struct StringConstraints {
    #[serde(default)]
    /// The minimum length of the string.
    pub min_length: usize,
    #[serde(default = "<usize as TraitIntBounds>::max")]
    /// The maximum length of the string.
    pub max_length: usize,
    #[serde(default)]
    /// The required regex match.
    pub regex: Option<String>,
}

impl Default for StringConstraints {
    fn default() -> Self {
        Self {
            min_length: 0,
            max_length: usize::MAX,
            regex: None,
        }
    }
}

impl Validator<String> for StringConstraints {
    type Err = String;

    fn validate(&mut self, input: &String) -> Result<(), Self::Err> {
        if input.len() < self.min_length {
            return Err(format!(
                "Value {input:?} does not meet the minimum required length ({})",
                self.max_length
            ));
        }

        if input.len() > self.max_length {
            return Err(format!(
                "Value {input:?} exceeds the maximum allowed length ({})",
                self.max_length
            ));
        }

        if let Some(re) = self.regex.as_ref() {
            let regex =
                Regex::new(re).map_err(|e| format!("Failed to build regex validator: {e}"))?;

            if !regex.is_match(input) {
                return Err(format!(
                    "Value {input:?} does not match regex pattern: {re:?}"
                ));
            }
        }

        Ok(())
    }
}

#[derive(serde::Deserialize, Clone, Copy)]
/// The constraints for integer types.
pub struct IntConstraints<T: TraitIntBounds + Clone + Copy> {
    #[serde(default)]
    /// The minimum value allowed.
    pub min: T,
    #[serde(default = "<T as TraitIntBounds>::max")]
    /// The maximum value allowed.
    pub max: T,
}

impl<T: TraitIntBounds + Clone + Copy> Default for IntConstraints<T> {
    fn default() -> Self {
        Self {
            min: T::min(),
            max: T::max(),
        }
    }
}

impl<T: TraitIntBounds + Clone + Copy> Validator<T> for IntConstraints<T> {
    type Err = String;

    fn validate(&mut self, input: &T) -> Result<(), Self::Err> {
        if input < &self.min {
            return Err(format!("Value {input:?} cannot be less than {}", self.min));
        }

        if input > &self.max {
            return Err(format!("Value {input:?} must be less than {}", self.max));
        }

        Ok(())
    }
}

/// A no-op validator.
pub struct BlankValidator;
impl<T> Validator<T> for BlankValidator {
    type Err = String;

    fn validate(&mut self, _input: &T) -> Result<(), Self::Err> {
        Ok(())
    }
}
