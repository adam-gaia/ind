use crate::config::Config;
use anyhow::{bail, Result};
use commandstream::CommandStream;
use log::debug;

pub struct App<'a> {
    stdout_ind: String,
    stderr_ind: String,
    command: &'a [String],
}

impl<'a> App<'a> {
    pub fn with_default_base(command: &'a [String]) -> Result<Self> {
        Ok(App {
            command,
            stdout_ind: String::from("  "),
            stderr_ind: String::from("  "),
        })
    }

    pub fn from_config(config: &Config, command: &'a [String]) -> Result<Self> {
        let stdout_ind = if let Some(level) = config.level {
            " ".repeat(level)
        } else if let Some(string) = &config.string {
            string.to_string()
        } else {
            unreachable!("Cannot set both 'level' and 'string' config/arg values");
        };
        let stderr_ind = if let Some(err_level) = config.err_level {
            " ".repeat(err_level)
        } else if let Some(err_string) = &config.err_string {
            err_string.to_string()
        } else {
            unreachable!("Cannot set both 'level' and 'string' config/arg values");
        };
        Ok(App {
            command,
            stdout_ind,
            stderr_ind,
        })
    }
}

impl<'a> CommandStream<'_> for App<'a> {
    fn command(&self) -> &[String] {
        &self.command
    }

    fn handle_stdout(&self, line: &str) -> Result<()> {
        println!("{}{}", self.stdout_ind, line);
        Ok(())
    }

    fn handle_stderr(&self, line: &str) -> Result<()> {
        eprintln!("{}{}", self.stderr_ind, line);
        Ok(())
    }
}
