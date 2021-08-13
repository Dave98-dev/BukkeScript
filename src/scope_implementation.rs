use crate::eval_implementation::eval;
use crate::variable_implementation::add_variables;
use crate::variable_implementation::get_name_and_value;
use std::collections::HashMap;

pub struct Scope {
    variables: HashMap<String, f32>,
    jumps: HashMap<String, usize>,
}

pub enum StatementResult{
    Output(String),
    LineJump(usize),
    NewLine,
    Nothing
}

impl Scope {
    pub fn new() -> Scope {
        let vars: HashMap<String, f32> = HashMap::new();
        let jumps: HashMap<String, usize> = HashMap::new();
        return Scope {
            variables: vars,
            jumps: jumps,
        };
    }
    pub fn execute(&mut self, s: String, line_number: usize) -> StatementResult {
        //se inizia per : ignoro
        if s.len() > 1 && s.chars().next().unwrap() == ':' {
            &self.jumps.entry(s).or_insert(line_number);
            return StatementResult::Nothing;
        }

        if s.len() == 2 && &s.trim()[..2] == "\\n"{
            return StatementResult::NewLine;
        }

        if s.len() > 2 && &s.trim()[..2] == "if" {
            match Scope::get_if_expression(&s) {
                Ok(var) => {
                    let (expression, label) = var;
                    match eval(&expression, &self.variables) {
                        Ok(result) => {
                            if result == 1.0 {
                                return StatementResult::LineJump(*self.jumps.get(label.trim()).unwrap())
                            }
                            //restituisco niente perché è un assegnamento
                            return StatementResult::Nothing;
                        }
                        Err(e) => {
                            return StatementResult::Output(e);
                        }
                    }
                }
                Err(error) => {
                    return StatementResult::Output(error);
                }
            }
        }

        //se inizia per let memorizzo una nuova variabile
        if s.len() > 4 && &s.trim()[..3] == "let" {
            match add_variables(&s, &self.variables) {
                Ok(var) => {
                    if let Some((key, value)) = var {
                        self.variables.entry(key).or_insert(value);
                    }
                }
                Err(e) => {
                    return StatementResult::Output(String::from(e));                    
                }
            }
        }
        if s.len() < 4 || &s.trim()[..3] != "let" {
            match get_name_and_value(&s) {
                //se trovo un uguale memorizzo una variabile esistente
                Ok((name, value)) => {
                    match eval(&value, &self.variables) {
                        Ok(result) => {
                            *self.variables.get_mut(&name).unwrap() = result;
                            //restituisco niente perché è un assegnamento
                            return StatementResult::Nothing;
                        }
                        Err(e) => {
                            return StatementResult::Output(e);
                        }
                    }
                }
                //altrimenti mostro il valore
                Err(_e) => match eval(&s, &self.variables) {
                    Ok(result) => {
                        return StatementResult::Output(format!("{}", result));
                    }
                    Err(e) => {
                        return StatementResult::Output(e);
                    }
                },
            }
        } else {
            return StatementResult::Nothing;
        }
    }
    
    pub fn get_if_expression(s: &str) -> Result<(String, String), String> {
        let goto_index = s.find("goto");
        match goto_index {
            Some(i) => {
                let expression = String::from(s[2..i].trim());
                let label = String::from(s[i + 4..].trim());
                return Ok((expression, label));
            }
            None => return Err(String::from("couldn't find 'goto'")),
        }
    }
}
