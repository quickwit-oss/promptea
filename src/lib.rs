mod constraints;
mod value;

use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::io;

use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, MultiSelect, Select, Validator};
use indexmap::IndexMap;
use inflector::Inflector;

pub use self::constraints::{
    BlankValidator, CollectionConstraints, Conditions, IfCondition, IntConstraints,
    SelectConstraints, StringConstraints,
};
pub use self::value::{PromptValue, TraitIntBounds};

static SKIP_MESSAGE: &str = "Did you mean to skip this field entirely?";

#[derive(serde::Deserialize)]
/// A prompt schema.
///
/// The schema describes how promptea should prompt the user
/// for input and validate it accordingly.
pub struct Schema {
    /// The schema fields to prompt users.
    pub fields: IndexMap<String, Field>,
}

impl Schema {
    pub fn prompt(&self, quiet: bool) -> io::Result<BTreeMap<String, serde_json::Value>> {
        let mut populated_fields = BTreeMap::new();
        for (key, field) in self.fields.iter() {
            let value = field.prompt(key, quiet, false, &mut populated_fields)?;
            populated_fields.insert(key.clone(), value);
        }
        Ok(populated_fields)
    }
}

#[derive(serde::Deserialize)]
pub struct Field {
    #[serde(default)]
    /// The display name to show as the prompt rather
    /// than the field key.
    pub display_name: Option<String>,
    #[serde(default)]
    /// An optional prompt message.
    ///
    /// If left blank this defaults to the display name or field name.
    pub prompt: Option<String>,
    #[serde(default)]
    /// The help description to display if enabled.
    pub description: String,
    #[serde(flatten)]
    /// The specific type and relevant constraints for the field.
    pub type_constraints: TypeConstraints,
    #[serde(default)]
    /// Can the value be skipped/left blank.
    pub can_skip: bool,
}

impl Field {
    pub fn prompt(
        &self,
        field_key: &str,
        quiet: bool,
        hide_title: bool,
        populated_fields: &mut BTreeMap<String, serde_json::Value>,
    ) -> io::Result<serde_json::Value> {
        if !quiet {
            if !hide_title && self.display_name.is_some() {
                let styled = Style::new()
                    .bold()
                    .underlined()
                    .for_stdout()
                    .apply_to(self.display_name.as_ref().unwrap());
                println!("\n{styled}:");
            } else if !hide_title {
                println!();
            }

            let styled = Style::new().dim().italic().for_stdout();
            for line in self.description.lines() {
                println!("  {}", styled.apply_to(line));
            }
        }

        let field_name = self
            .prompt
            .as_deref()
            .or(self.display_name.as_deref())
            .map(str::to_string)
            .unwrap_or_else(|| field_key.to_title_case());
        self.type_constraints
            .prompt(&field_name, self.can_skip, quiet, populated_fields)
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum TypeConstraints {
    /// A boolean type.
    Bool,
    /// A string type.
    String(StringConstraints),
    /// A u64 type.
    U64(IntConstraints<u64>),
    /// A u32 type.
    U32(IntConstraints<u32>),
    /// A u16 type.
    U16(IntConstraints<u16>),
    /// A u8 type.
    U8(IntConstraints<u8>),
    /// A i64 type.
    I64(IntConstraints<i64>),
    /// A i32 type.
    I32(IntConstraints<i32>),
    /// A i16 type.
    I16(IntConstraints<i16>),
    /// A i8 type.
    I8(IntConstraints<i8>),
    /// A f64 type.
    F64(IntConstraints<f64>),
    /// A f32 type.
    F32(IntConstraints<f32>),
    /// A select menu
    Select {
        #[serde(flatten)]
        constraints: SelectConstraints,
        #[serde(rename = "then", default)]
        conditions: Conditions,
    },
    /// A nested object.
    Object {
        /// The fields within the nested object.
        fields: IndexMap<String, Field>,
    },
    #[serde(rename = "string[]")]
    /// An array of string values.
    ArrayString {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: StringConstraints,
    },
    #[serde(rename = "u64[]")]
    /// An array of u64 values.
    ArrayU64 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<u64>,
    },
    #[serde(rename = "u32[]")]
    /// An array of u32 values.
    ArrayU32 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<u32>,
    },
    #[serde(rename = "u16[]")]
    /// An array of u16 values.
    ArrayU16 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<u16>,
    },
    #[serde(rename = "u8[]")]
    /// An array of u8 values.
    ArrayU8 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<u8>,
    },
    #[serde(rename = "i64[]")]
    /// An array of i64 values.
    ArrayI64 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<i64>,
    },
    #[serde(rename = "i32[]")]
    /// An array of i32 values.
    ArrayI32 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<i32>,
    },
    #[serde(rename = "i16[]")]
    /// An array of i16 values.
    ArrayI16 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<i16>,
    },
    #[serde(rename = "i8[]")]
    /// An array of i8 values.
    ArrayI8 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<i8>,
    },
    #[serde(rename = "f64[]")]
    /// An array of f64 values.
    ArrayF64 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<f64>,
    },
    #[serde(rename = "f32[]")]
    /// An array of f32 values.
    ArrayF32 {
        #[serde(flatten)]
        constraints: CollectionConstraints,
        #[serde(flatten)]
        inner_constraints: IntConstraints<f32>,
    },
}

impl TypeConstraints {
    pub fn prompt(
        &self,
        field_name: &str,
        can_skip: bool,
        quiet: bool,
        populated_fields: &mut BTreeMap<String, serde_json::Value>,
    ) -> io::Result<serde_json::Value> {
        let theme = ColorfulTheme::default();
        match self {
            TypeConstraints::Bool => bool::prompt(field_name, Some(BlankValidator), can_skip)
                .map(serde_json::Value::from),
            TypeConstraints::String(constraints) => {
                String::prompt(field_name, Some(constraints.clone()), can_skip)
                    .map(serde_json::Value::from)
            }
            TypeConstraints::U64(constraints) => {
                u64::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::U32(constraints) => {
                u32::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::U16(constraints) => {
                u16::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::U8(constraints) => {
                u8::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::I64(constraints) => {
                i64::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::I32(constraints) => {
                i32::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::I16(constraints) => {
                i16::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::I8(constraints) => {
                i8::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::F64(constraints) => {
                f64::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::F32(constraints) => {
                f32::prompt(field_name, Some(*constraints), can_skip).map(serde_json::Value::from)
            }
            TypeConstraints::Select {
                constraints,
                conditions,
            } => {
                let items = constraints
                    .items
                    .iter()
                    .map(display_value)
                    .collect::<Vec<String>>();

                if constraints.select_many {
                    let maybe_selections = MultiSelect::with_theme(&ColorfulTheme::default())
                        .with_prompt(field_name)
                        .items(&items)
                        .defaults(&[])
                        .interact_opt()?;

                    let selections = match maybe_selections {
                        None => return Ok(serde_json::Value::Null),
                        Some(selections) => selections,
                    };

                    let selections = selections
                        .into_iter()
                        .flat_map(|index| constraints.items.get(index).cloned());

                    let mut values = Vec::new();
                    for selected in selections {
                        let returned_value =
                            check_conditions(conditions, &selected, quiet, populated_fields)?;
                        values.push(returned_value.unwrap_or(selected));
                    }

                    return Ok(serde_json::Value::Array(values));
                }

                let selected_value = if can_skip {
                    Select::with_theme(&theme)
                        .with_prompt(field_name)
                        .default(0)
                        .items(&items)
                        .interact_opt()?
                        .and_then(|index| constraints.items.get(index).cloned())
                        .unwrap_or(serde_json::Value::Null)
                } else {
                    let index = Select::with_theme(&theme)
                        .with_prompt(field_name)
                        .default(0)
                        .items(&items)
                        .interact()?;
                    constraints
                        .items
                        .get(index)
                        .cloned()
                        .unwrap_or(serde_json::Value::Null)
                };

                let returned_value =
                    check_conditions(conditions, &selected_value, quiet, populated_fields)?;
                Ok(returned_value.unwrap_or(selected_value))
            }
            TypeConstraints::ArrayString {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, inner_constraints.clone()),
            TypeConstraints::ArrayU64 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayU32 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayU16 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayU8 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayI64 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayI32 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayI16 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayI8 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayF64 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::ArrayF32 {
                constraints,
                inner_constraints,
            } => array_prompter(can_skip, field_name, constraints, *inner_constraints),
            TypeConstraints::Object { fields } => {
                let mut nested_fields = serde_json::Map::new();
                for (key, field) in fields {
                    let value = field.prompt(key, quiet, true, populated_fields)?;
                    nested_fields.insert(key.clone(), value);
                }
                Ok(serde_json::Value::Object(nested_fields))
            }
        }
    }
}

fn array_prompter<'a, V, T>(
    can_skip: bool,
    field_name: &str,
    constraints: &CollectionConstraints,
    validator: V,
) -> io::Result<serde_json::Value>
where
    T: PromptValue<'a, V> + Debug,
    V: Validator<T> + Clone + 'a,
    V::Err: Display,
{
    let error_style = Style::new().red().italic().for_stdout();
    let mut values = Vec::new();
    for _ in 0..constraints.max_items {
        let maybe_value = T::prompt(field_name, Some(validator.clone()), true)?;

        match maybe_value {
            Some(value) => values.push(value.into()),
            None => {
                if values.len() < constraints.min_items {
                    let msg = format!(
                        "This field requires a minimum of {} values to be provided. {}",
                        constraints.min_items,
                        if can_skip { SKIP_MESSAGE } else { "" }
                    );

                    println!("{}", error_style.apply_to(msg));
                    if can_skip {
                        let skip = Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt("Skip this field?")
                            .interact()?;

                        if skip {
                            break;
                        }
                    }

                    continue;
                }

                break;
            }
        }
    }

    Ok(serde_json::Value::from(values))
}

fn check_conditions(
    conditions: &Conditions,
    selected: &serde_json::Value,
    quiet: bool,
    populated_fields: &mut BTreeMap<String, serde_json::Value>,
) -> io::Result<Option<serde_json::Value>> {
    let mut return_value = None;
    for condition in conditions.if_conditions.iter() {
        if &condition.picked != selected {
            continue;
        }

        let mut object = serde_json::Map::new();
        for (key, field) in condition.fields.iter() {
            let value = field.prompt(key, quiet, false, populated_fields)?;

            if conditions.insert_at_root {
                populated_fields.insert(key.clone(), value);
            } else {
                object.insert(key.clone(), value);
            }
        }

        if !conditions.insert_at_root {
            return_value = Some(serde_json::Value::Object(object));
        }

        break;
    }

    Ok(return_value)
}

fn display_value(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(int) => int.to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(a) => format!("{a:?}"),
        serde_json::Value::Object(o) => format!("{o:?}"),
    }
}
