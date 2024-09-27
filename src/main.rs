use interface::UserInput;
use std::fs::File;
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
        Err(e) => {
            eprintln!("Error generating markdown: {:?}", e);
            String::new()
        }
    };
    println!("markdown string {:?}", &markdown_string);
    let output_path = Path::new("dist/README.md");
    let file = match File::create(output_path) {
        Ok(file) => { file }
        Err(e) => {
            return Err(serde_json::Error::io(e));
        }
    };
    let mut md = Markdown::new(file);
    let string: &str = &markdown_string;
    match md.write(string) {
        Ok(_) => {
            println!("check /dist for readme");
            Ok(())
        }
        Err(e) => Err(serde_json::Error::io(e)),
    }
    //  {
    //     Ok(file) => {
    //         println!("check /dist for readme");
    //         file
    //     }
    //     Err(e) => {
    //         return Err(serde_json::Error::io(e));
    //     }
    // };
    // md_file.write_all(BufWriter::new(markdown_string));
}
