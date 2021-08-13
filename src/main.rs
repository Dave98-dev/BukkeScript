mod scope_implementation;
mod eval_implementation;
mod variable_implementation;
use crate::scope_implementation::Scope;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    
    let mut scope: Scope = Scope::new();

    let args:Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(&args[1]) {

        let linee:Vec<String> = lines
            .flat_map(|l| match l { Ok(s) => return vec![s], Err(_) => return vec![]})
            .map(|l| String::from(l.trim()))
            .filter(|l| l != "")
            .collect();

        let mut i = 0;
        loop{
            if i >= linee.len(){
                break;
            }


            let line = &linee[i];
            let mut s = line.clone();
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            if let Some(result) = scope.execute(s, i){
                if result.output.trim() != ""{
                    if result.output.trim() == "999"{
                        println!()
                    }else{
                        print!("{}", result.output);
                    }
                }
                i = result.riga;
            }else{
                i += 1;
            }

        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
