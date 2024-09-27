use std::io::*;
use formater::format_csv_to_dashed_list;
use dialoguer::Select;
use validator::validate_email;
use std::io::Error;

pub mod validator;
pub mod formater;

#[derive(Debug, Default)]
struct UserInput {
    github_username: String,
    email: String,
    title: String,
    description: String,
    installation: String,
    usage: String,
    credits: String,
    license: String,
    features: String,
    technologies: String,
    ways_to_contribute: String,
    tests: String,
}

impl UserInput {
    fn new() -> Self {
        Default::default()
    }

    fn set_gh_username(&mut self, username: &String) {
        self.github_username = username.to_string();
    }

    fn set_title(&mut self, title: &String) {
        self.title = title.to_string();
    }

    fn set_email(&mut self, email: &String) {
        self.email = email.to_string();
    }

    fn set_description(&mut self, description: &String) {
        self.description = description.to_string();
    }

    fn set_installation(&mut self, installation: &String) {
        self.installation = installation.to_string();
    }

    fn set_usage(&mut self, usage: &String) {
        self.usage = usage.to_string();
    }

    fn set_credits(&mut self, credits: &String) {
        self.credits = credits.to_string();
    }
    fn set_license(&mut self, license: &String) {
        self.license = license.to_string();
    }
    fn set_features(&mut self, features: &String) {
        self.features = features.to_string();
    }
    fn set_technologies(&mut self, technologies: &String) {
        self.technologies = technologies.to_string();
    }

    fn set_ways_to_contribute(&mut self, ways_to: &String) {
        self.ways_to_contribute = ways_to.to_string();
    }
    fn set_tests(&mut self, tests: &String) {
        self.tests = tests.to_string();
    }
}
fn main() {
    let mut user_input = UserInput::new();
    let result = collect_input(&mut user_input);
    println!("{:?}", &result);
}

fn collect_input(user_input: &mut UserInput) -> &UserInput {
    // let mut user_input = UserInput::new();

    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    println!("Please enter your github username to be included in your README");

    handle.read_line(&mut input);
    user_input.set_gh_username(&input);
    input.clear();

    println!("Please enter your email address to be included in your README");

    handle.read_line(&mut input);
    input = input.trim().to_string();

    println!("email {}", &input);
    while validate_email(&input) == false {
        input.clear();
        println!("Please enter a valid email address, or at least something that looks like one");
        handle.read_line(&mut input).map_err(|e| e.to_string());
    }
    

    user_input.set_email(&input);
    input.clear();

    println!("Please entire a title for your README.");
    handle.read_line(&mut input).map_err(|e| e.to_string());

    user_input.set_title(&input);
    input.clear();

    println!("Please enter a brief description explaining the what, why, and how of your project.");
    handle.read_line(&mut input).map_err(|e| e.to_string());
    user_input.set_description(&input);
    input.clear();

    println!("Please enter installation instructions for your application.");
    handle.read_line(&mut input).map_err(|e| e.to_string());
    user_input.set_installation(&input);
    input.clear();

    println!("Please enter usage instruction for your application.");
    handle.read_line(&mut input).map_err(|e| e.to_string());
    user_input.set_usage(&input);
    input.clear();

    println!("Please enter credit information.");
    handle.read_line(&mut input).map_err(|e| e.to_string());
    user_input.set_credits(&input);
    input.clear();

    let choices = &["y", "n"];
    let include_license_input = Select::new()
        .with_prompt("Would you like to include a license? ")
        .items(choices)
        .interact()
        .map_err(|e| e.to_string());
    let include_license = include_license_input == Ok(0);

    while include_license && input.is_empty() {
        let choices = vec![
            "Apache 2.0",
            "BSD 3-Clause",
            "BSD 2-Clause",
            "GPL v3",
            "GPL v2",
            "AGPL v3",
            "LGPL v3",
            "Unlicense",
            "The Do What the Fuck You Want to Public License",
            "MIT"
        ];
        input = match
            Select::new()
                .with_prompt("Please choose a license.")
                .items(&choices)
                .interact()
                .map_err(|e| e.to_string())
        {
            Ok(i) => choices[i].to_string(),
            Err(e) => String::from("butts... wait no error"),
        };
    }
    if input.len() == 0 {
        let val = String::from("n/a");
        user_input.set_license(&val);
    } else {
        user_input.set_license(&input);
    }
    input.clear();

    println!(
        "Please enter a list of features available on your web application separated by commas. (For real, now. You made the formatting change."
    );
    handle.read_line(&mut input).map_err(|e| e.to_string());
    let formatted_features = format_csv_to_dashed_list(&input);
    user_input.set_features(&formatted_features);

    input.clear();

    println!(
        "Please enter a list of technologies available on your web application separated by commas. (For real, now. You made the formatting change."
    );
    handle.read_line(&mut input).map_err(|e| e.to_string());
    let formatted_techs = format_csv_to_dashed_list(&input);
    user_input.set_technologies(&formatted_techs);
    input.clear();

    println!("Please explain how other developers can contribute to your project.");
    handle.read_line(&mut input).map_err(|e| e.to_string());
    user_input.set_ways_to_contribute(&input);

    println!("Please list tests for your application.");
    handle.read_line(&mut input);
    user_input.set_tests(&input);
    input.clear();

    user_input
}
