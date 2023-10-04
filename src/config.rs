use std::env::{self, VarError};

pub struct EnvVars {
    pub ignore_case: Result<String, VarError>,
}

impl EnvVars {
    pub fn fetch() -> EnvVars {
        EnvVars {
            ignore_case: env::var("IGNORE_CASE"),
        }
    }

    #[cfg(test)]
    pub fn new(ignore_case: Option<String>) -> EnvVars {
        EnvVars {
            ignore_case: ignore_case.ok_or(VarError::NotPresent),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>, envs: EnvVars) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: envs.ignore_case.is_ok(),
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::{Config, EnvVars};

    fn args(args_string: &str) -> Vec<String> {
        let mut args = vec![String::from("path/to/binary")];
        for config in args_string.split_whitespace() {
            args.push(String::from(config));
        }

        args
    }

    fn empty_envs() -> EnvVars {
        EnvVars::new(None)
    }

    #[test]
    fn build_not_enough_arguments() {
        let args = args("foo");
        let result = Config::build(args, empty_envs());
        assert!(result.is_err_and(|msg| msg == "not enough arguments"));
    }

    #[test]
    fn build_query() {
        let query = String::from("foo");
        let args = args(&format!("{query} path"));
        let result = Config::build(args, empty_envs());

        assert_eq!(result.unwrap().query, query)
    }

    #[test]
    fn build_path() {
        let path = String::from("foo");
        let args = args(&format!("query {path}"));
        let result = Config::build(args, empty_envs());

        assert_eq!(result.unwrap().file_path, path)
    }

    #[test]
    fn build_with_unset_env_ignore_case() {
        let args = args("query path");
        let envs = EnvVars::new(None);
        let result = Config::build(args, envs);

        assert_eq!(result.unwrap().ignore_case, false)
    }

    #[test]
    fn build_with_env_ignore_case_set_to_anything() {
        let args = args("query path");
        let envs = EnvVars::new(Some("anything".to_string()));
        let result = Config::build(args, envs);

        assert_eq!(result.unwrap().ignore_case, true)
    }
}
