use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        // available if we implement try_from, as value is vec<string> it understands what we are trying to call
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            pwd,
            config,
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<String>) -> Result<Self> {
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }
        let term = value.get(0).expect("Expect to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "Op Add expects 2 arguments but got {}",
                    value.len() - 1
                ));
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }
        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "Op rm expects 2 arguments but got {}",
                    value.len() - 1
                ));
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }
        if value.len() > 1 {
            return Err(anyhow!(
                "Op print expects 0 or 1 arguments but got {}",
                value.len() - 1
            ));
        }
        return Ok(Operation::Print(Some(value[0].clone())));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let loc = std::env::current_dir().context("unable to get xdg_config_home")?;
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");
    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = pwd {
        return Ok(v);
    }
    return Ok(std::env::current_dir().context("unable to get current director")?);
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::{config::Operation, opts::Opts};
    use anyhow::{Ok, Result};

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            pwd: None,
            config: None,
        }
        .try_into()?;
        assert_eq!(opts.operation, Operation::Print(None));
        Ok(())
    }
    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["hi".to_string()],
            pwd: None,
            config: None,
        }
        .try_into()?;
        assert_eq!(opts.operation, Operation::Print(Some(String::from("hi"))));
        Ok(())
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["add".to_string(), "hi".to_string(), "boo".to_string()],
            pwd: None,
            config: None,
        }
        .try_into()?;
        assert_eq!(
            opts.operation,
            Operation::Add(String::from("hi"), "boo".to_string())
        );
        Ok(())
    }

    #[test]
    fn test_remove_key_value() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["rm".to_string(), "hi".to_string()],
            pwd: None,
            config: None,
        }
        .try_into()?;
        assert_eq!(opts.operation, Operation::Remove(String::from("hi")));
        Ok(())
    }
}
