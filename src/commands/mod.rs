use std::env;
use std::process;
use failure::Error;
use clap::{App, AppSettings, ArgMatches};

use crate::constants::{VERSION, AUTHOR};
use crate::config::{Config};

const APP_NAME: &str = "glab-cli";

const ABOUT: &str = "
Glab CLI is a command line utility for Gitlab written in rust.
Note: This is not a official CLI
";

macro_rules! each_subcommand {
  ($mac:ident) => {
    $mac!(login);
  }
}

pub mod login;

fn execute_command(matches: &ArgMatches<'_>) -> Result<(), Error> {
  macro_rules! execute_subcommand {
    ($name:ident) => {{
      let cmd = stringify!($name).replace("_", "-");
      if let Some(sub_matches) = matches.subcommand_matches(&cmd) {
        let rv = $name::execute(&sub_matches)?;
        return Ok(rv);
      }
    }};
  }
  each_subcommand!(execute_subcommand);
  unreachable!();
}

fn add_commands<'a, 'b>(mut app: App<'a, 'b>) -> App<'a, 'b> {
  macro_rules! add_subcommand {
    ($name:ident) => {{
      let cmd = $name::make_app(App::new(stringify!($name).replace("_", "-").as_str()));
      app = app.subcommand(cmd);
    }};
  }

  each_subcommand!(add_subcommand);
  app
}

fn execute(args: &[String]) -> Result<(), Error> {
  let mut config = Config::from_cli_config()?;
  let mut app = App::new(APP_NAME)
                  .version(VERSION)
                  .author(AUTHOR)
                  .about(ABOUT)
                  .max_term_width(100)
                  .setting(AppSettings::VersionlessSubcommands)
                  .setting(AppSettings::SubcommandRequiredElseHelp);
  app = add_commands(app);
  
	// bind the config to the process and fetch an immutable reference to it
  config.bind_to_process();
	
  let matches = app.get_matches_from_safe(&args[..])?;
  execute_command(&matches)
}

pub fn main() {
  let result = execute(&env::args().collect::<Vec<String>>());
  let status_code = match result {
    Ok(()) => 0,
    Err(_err) => 1
  };
  process::exit(status_code);
}