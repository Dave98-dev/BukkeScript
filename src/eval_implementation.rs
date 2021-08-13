use std::collections::HashMap;
use std::io::stdin;

pub fn eval(s: &String, variables: &HashMap<String, f32>) -> Result<f32, String> {
    match get_first_symbol(s) {
        None => {
            let symbol = s.trim();
            let num = symbol.parse::<f32>();
            match num {
                Ok(num) => return Ok(num),
                Err(_e) => {
                    if symbol == "input()" {
                        println!("please enter a number");
                        let mut s = String::new();
                        stdin()
                            .read_line(&mut s)
                            .expect("Did not enter a correct string");
                        return Ok(s.trim().parse::<f32>().expect("numero errato"));
                    } else if variables.contains_key(symbol) {
                        return Ok(variables[symbol]);
                    } else {
                        return Err(format!("Variable '{}' not found", symbol));
                    }
                }
            }
        }
        Some(i) => {
            let s1 = &s[..i];
            let s2 = &s[i + 1..];
            let number1 = eval(&String::from(s1), variables)?;
            let number2 = eval(&String::from(s2), variables)?;
            let f = operations(s.chars().nth(i).unwrap())?;
            return Ok(f(number1, number2));
        }
    }
}

fn get_first_symbol(s: &String) -> Option<usize> {
    let symbols = ['+', '-', '*', '/', '>', '<'];

    for operation in &symbols {
        let mut index = 0;
        for character in s.chars() {
            if character == *operation {
                return Some(index);
            }
            index += 1;
        }
    }
    return None;
}

fn operations(operation: char) -> Result<fn(f32, f32) -> f32, String> {
    match operation {
        '+' => Ok(|x, y| x + y),
        '-' => Ok(|x, y| x - y),
        '*' => Ok(|x, y| x * y),
        '/' => Ok(|x, y| x / y),
        '>' => Ok(|x, y| if x > y { 1.0 } else { 0.0 }),
        '<' => Ok(|x, y| if x < y { 1.0 } else { 0.0 }),
        _ => Err(format!("operator '{}' not found", operation)),
    }
}
