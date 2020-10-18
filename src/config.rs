//! This module implements config access.
use std::io;
use std::fs;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;

use ini::Ini;
use lazy_static::lazy_static;
use failure::{Error, err_msg};

use crate::constants::{CONFIG_RC_FILE_NAME};

#[derive(Debug, Clone)]
pub enum Auth {
    Key(String),
    Token(String),
}

/// Represents `glab-cli` config
pub struct Config {
    filename: PathBuf,
    // ini file
    ini: Ini,
	// Indicates whether the config is bound to the current process
	process_bound: bool,
    // Cache auth
    cached_auth: Option<Auth>,
    // Cache vcs
    cached_vcs_remote: String
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Arc<Config>>> = Mutex::new(None);
}

impl Config {
    /// Loads the CLI config from the default location and returns it.
    pub fn from_cli_config() -> Result<Self, Error> {
        let (filename, ini) = load_cli_config()?;
        Config::from_file(filename, ini)
    }

    pub fn from_file(filename: PathBuf, ini: Ini) -> Result<Self, Error> {
        Ok(Config {
            filename,
            cached_auth: get_default_auth(&ini),
            cached_vcs_remote: get_default_vcs_remote(&ini),
						ini: ini,
						process_bound: false
        })
    }

    /// Return the currently bound config as option.
    pub fn current_opt() -> Option<Arc<Config>> {
        CONFIG.lock().as_ref().cloned()
    }

    /// Return the currently bound config.
    pub fn current() -> Arc<Config> {
        Config::current_opt().expect("Config not bound yet")
    }
	
    pub fn bind_to_process(mut self) -> Arc<Config> {
        self.process_bound = true;
        {
            let mut cfg = CONFIG.lock();
            *cfg = Some(Arc::new(self));
        }
        Config::current()
    }
	
    /// Makes a copy of the config in a closure and boxes it.
    pub fn make_copy<F: FnOnce(&mut Config) -> Result<(), Error>>(
        &self,
        cb: F,
    ) -> Result<(), Error> {
        let mut new_config = self.clone();
        cb(&mut new_config)?;
        Ok(())
    }

    /// Updates the auth info
    pub fn set_auth(&mut self, auth: Auth) {
        self.cached_auth = Some(auth);

        self.ini.delete_from(Some("auth"), "api_key");
        self.ini.delete_from(Some("auth"), "token");
        match self.cached_auth {
            Some(Auth::Token(ref val)) => {
                self.ini
                    .set_to(Some("auth"), "token".into(), val.to_string());
            }
            Some(Auth::Key(ref val)) => {
                self.ini
                    .set_to(Some("auth"), "api_key".into(), val.to_string());
            }
            None => {}
        }
    }
}

impl Clone for Config {
    fn clone(&self) -> Config {
        Config {
            filename: self.filename.clone(),
            ini: self.ini.clone(),
            cached_auth: self.cached_auth.clone(),
            cached_vcs_remote: self.cached_vcs_remote.clone(),
			process_bound: false
        }
    }
}

fn load_cli_config() -> Result<(PathBuf, Ini), Error> {
    let (global_filename, mut rv) = load_global_config_file()?;

    let (path, rv) = if let Some(project_config_path) = find_project_config_file() {
        let mut f = fs::File::open(&project_config_path)?;
        let ini = Ini::read_from(&mut f)?;
        for (section, props) in ini.iter() {
            for (key, value) in props.iter() {
                rv.set_to(section, key.to_string(), value.to_owned());
            }
        }
        (project_config_path, rv)
    } else {
        (global_filename, rv)
    };
	
	Ok((path, rv))
}

fn load_global_config_file() -> Result<(PathBuf, Ini), Error> {
    let filename = find_global_config_file()?;
    match fs::File::open(&filename) {
        Ok(mut file) => match Ini::read_from(&mut file) {
            Ok(ini) => Ok((filename, ini)),
            Err(err) => Err(Error::from(err)),
        },
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Ok((filename, Ini::new()))
            } else {
                Err(
                    Error::from(err)
                    .context("Failed to load .glabclirc file from the home folder.")
                    .into()
                )
            }
        }
    }
}

fn find_global_config_file() -> Result<PathBuf, Error> {
    dirs::home_dir()
        .ok_or_else(|| err_msg("Could not find home dir"))
        .map(|mut path| {
            path.push(CONFIG_RC_FILE_NAME);
            path
        })
}

fn find_project_config_file() -> Option<PathBuf> {
    env::current_dir().ok().and_then(|mut path| loop {
        path.push(CONFIG_RC_FILE_NAME);
        if path.exists() {
            return Some(path);
        }
        path.set_file_name("glabcli.ini");
        if path.exists() {
            return Some(path);
        }
        path.pop();
        if !path.pop() {
            return None;
        }
    })
}

fn get_default_auth(ini: &Ini) -> Option<Auth> {
    if let Some(val) = ini.get_from(Some("auth"), "token") {
        Some(Auth::Token(val.to_owned()))
    } else {
        None
    }
}

fn get_default_vcs_remote(ini: &Ini) -> String {
    if let Some(remote) = ini.get_from(Some("defaults"), "vcs_remote") {
        remote.to_string()
    } else {
        "origin".to_string()
    }
}