use clap::{App, Arg, ArgMatches};
use failure::Error;

use crate::config::{Auth, Config};
use crate::utils::ui::prompt;

pub fn make_app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
  return app.about("Login to Gitlab")
    .arg(
      Arg::with_name("global")
        .short("g")
        .long("global")
        .help("Store authentication token globally rather than locally.")
    )
    .arg(
      Arg::with_name("host")
        .short("h")
        .long("host")
        .help("Host of Gitlab")
    );
}

pub fn execute<'a>(_matches: &ArgMatches<'a>) -> Result<(), Error> {
  let config = Config::current();
  let mut token;

  loop {
    token = prompt("Enter your Gitlab auth token")?;
    config.make_copy(|cfg| {
      cfg.set_auth(Auth::Token(token.to_string()));
      Ok(())
    })?;
    println!("{}", token);
  }
}