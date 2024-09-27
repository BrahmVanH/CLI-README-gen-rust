use crate::interface::UserInput;
use std::path::Path;
use std::fs::File;
use serde_json;

#[derive(serde::Deserialize)]
struct License {
    name: String,
    badge: String,
    link: String,
}

pub fn format_readme_markdown(user_input: &UserInput) -> Result<String, serde_json::Error> {
    let licenses_json = get_licenses_json();
    let licenses_file = match licenses_json {
        Ok(file) => file,
        Err(e) => {
            return Err(serde_json::Error::io(e));
        }
    };
    let licenses: Vec<License> = match serde_json::from_reader(licenses_file) {
        Ok(json) => json,
        Err(e) => {
            return Err(e);
        }
    };
    let mut license_badge_string = String::new();
    let mut license_link = String::new();
    for l in &licenses {
        if l.name == user_input.license.trim() {
            license_badge_string = l.badge.clone();
        }
    }
    for l in licenses {
        if l.name == user_input.license.trim() {
            license_link = l.link;
        }
    }

    let UserInput {
        github_username,
        email,
        title,
        description,
        installation,
        usage,
        credits,
        features,
        technologies,
        ways_to_contribute,
        tests,
        ..
    } = user_input;

    let string = format!(
        "# {title}
  
  {license_badge_string}

  
  ## Description 
  
  {description}
  
  ## Table of Contents

  ⋆[Installation](#Installation)
  ⋆[Usage](#Usage)
  ⋆[Credits](#Credits)
  ⋆[License](#License)
  ⋆[Features](#Features)
  ⋆[Contributions](#Contributions)
  ⋆[Test](#Contributions)

  ## Installation 

  {installation}

  ## Usage

  {usage}

  ## Credits 

  {credits}

  ## License

  {license_link}  

  ## Features

  {features}

  ## Technologies

  {technologies}

  ## Contributions

  {ways_to_contribute}

  ## Test

  {tests}

  ## Questions

  If you have any questions about the project you can reach out to me via email or GitHub with the information below. 

  >Email: {email}

  >GitHub: [{github_username}](https://github.com/{github_username})
  "
    );
    Ok(string)
}

fn get_licenses_json() -> std::io::Result<File> {
    let licenses_path = Path::new("src/licenses.json");
    let file = File::open(licenses_path)?;
    Ok(file)
}
