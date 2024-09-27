use interface::UserInput;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json;
use markdown_gen::markdown::Markdown;

pub mod interface;
pub mod validator;
pub mod formater;
pub mod readme_generator;

fn main() -> Result<(), serde_json::Error> {
    let mut user_input = UserInput::new();
    let output = match interface::collect_input(&mut user_input) {
        Ok(user_input) => user_input,
        Err(e) => {
            return Err(serde_json::Error::io(e));
        }
    };
    let markdown_string: String = match readme_generator::format_readme_markdown(output) {
        Ok(string) => string,
        Err(e) => { String::new() }
    };
    let output_path = Path::new("dist/README.md");
    let mut file = match File::create(&output_path) {
        Ok(file) => { file }
        Err(e) => {
            return Err(serde_json::Error::io(e));
        }
    };
    file.write_all(&markdown_string.as_bytes());
    println!("README Successfully written. Check dist/ folder for result.");
    Ok(())
}
