extern crate futures;
extern crate hubcaps;
extern crate tokio_core;

use std::env;
use std::error::Error;
use tokio_core::reactor::Core;

#[macro_use]
extern crate log;
extern crate env_logger;

use hubcaps::{Credentials, Github};

// Gets the value of an environment variable or prints a nicely formatted
// error message when missing. Logs yo.
fn env_or_error(input: &str) -> Result<std::string::String, Box<Error>> {
    match env::var(input) {
        Ok(val)  => return Ok(val),
        Err(e) => {
            error!("required environment variable {} has an empty value", input);
            return Err(e.into())
        },
    };
}

fn main() -> Result<(), Box<Error>> {

    // Sometimes we need to get the code to tattle on the problem.
    env_logger::init();

    // Get actions to perform via environment variables. These are all required.
    let repo = env_or_error("GITHUB_REPO")?;
    let label = env_or_error("GITHUB_ISSUE_LABEL")?;
    let token = env_or_error("GITHUB_TOKEN")?;

    // We need to convert num to a u64 for later use
    let num = env_or_error("GITHUB_ISSUE_NUMBER")?;
    let num = num.parse::<u64>()?;

    let parts: Vec<&str> = repo.split("/").collect();

    if parts.len() < 2 {
        error!("The repo does not follow the format [org|user]/[project]");
        return Err("GITHUB_REPO has improper format".into())
    }

    let mut core = Core::new()?;
    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::Token(token),
        &core.handle(),
    );

    // Add a label
    let f =  github
                .repo(parts[0], parts[1])
                .issues()
                .get(num)
                .labels()
                .add(vec![label.as_str()]);
    
    match core.run(f) {
        Ok(_) => {
            info!("{:?} label added", label);
        },
        Err(err) => {
            error!("Unable to add the label {:?}", label);
            return Err(err.into());
        }
    }

    Ok(())
}