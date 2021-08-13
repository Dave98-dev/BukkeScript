use std::collections::HashMap;
use crate::eval_implementation::eval;

pub fn add_variables(
    s: &String,
    variables: &HashMap<String, f32>,
) -> Result<Option<(String, f32)>, String> {
    if &s.trim()[..3] == "let" {
        let (name, value) = get_name_and_value(&s[3..])?;

        let value_evalued = eval(&value, variables)?;
        return Ok(Some((name, value_evalued)));
    }
    return Ok(None);
}

pub fn get_name_and_value(s: &str) -> Result<(String, String), String> {
    let eq_index = s.find('=');
    match eq_index {
        Some(i) => {
            let var_value = String::from(s[i + 1..].trim());
            let var_name = String::from(s[..i - 1].trim());
            return Ok((var_name, var_value));
        }
        None => return Err(String::from("couldn't find '='")),
    }
}
