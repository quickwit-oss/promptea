use promtea::Schema;


fn test_basic_schema_prompts() {
    let schema: Schema = serde_yaml::from_str(include_str!("./prompts/basic.yaml")).unwrap();
    let result = schema.prompt(false).expect("Prompt");
    dbg!(result);
}

fn test_array_schema_prompts() {
    let schema: Schema = serde_yaml::from_str(include_str!("./prompts/arrays.yaml")).unwrap();
    let result = schema.prompt(false).expect("Prompt");
    dbg!(result);
}

fn test_nested_schema_prompts() {
    let schema: Schema = serde_yaml::from_str(include_str!("./prompts/nested.yaml")).unwrap();
    let result = schema.prompt(false).expect("Prompt");
    dbg!(result);
}

fn main() {
    test_basic_schema_prompts();
    test_array_schema_prompts();
    test_nested_schema_prompts();
}