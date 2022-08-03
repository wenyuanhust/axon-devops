use clap::{Arg, ArgMatches, Command};
use std::{
    error::Error as StdErr,
    process::{self, Output},
};

pub struct HardHat {}

impl HardHat {
    pub fn get_hardhat_command() -> Command<'static> {
        Command::new("hardhat")
            .about("hardhat")
            .subcommand(Command::new("clean").about("Clears the cache and deletes all artifacts"))
            .subcommand(
                Command::new("compile")
                    .about("Compiles the entire project, building all artifacts"),
            )
            .subcommand(
                Command::new("run")
                    .about("Runs a user-defined script after compiling the project")
                    .arg(Arg::new("script").index(1)),
            )
            .subcommand(Command::new("test").about("Runs mocha tests"))
    }

    pub fn exec_hardhat(matches: &ArgMatches) -> Result<(), Box<dyn StdErr>> {
        match matches.subcommand() {
            Some(("clean", _)) => {
                let output = process::Command::new("npx")
                    .arg("hardhat")
                    .arg("clean")
                    .output()
                    .expect("");
                println!("{}", HardHat::transform_output(&output));
                Ok(())
            }
            Some(("compile", _)) => {
                let output = process::Command::new("npx")
                    .arg("hardhat")
                    .arg("compile")
                    .output()
                    .expect("");
                println!("{}", HardHat::transform_output(&output));
                Ok(())
            }
            Some(("test", _)) => {
                let output = process::Command::new("npx")
                    .arg("hardhat")
                    .arg("test")
                    .output()
                    .expect("");
                println!("{}", HardHat::transform_output(&output));
                Ok(())
            }
            Some(("run", matches)) => {
                let script = matches.value_of("script").unwrap_or("scripts/deploy.js");
                println!("{}", script);
                let output = process::Command::new("npx")
                    .arg("hardhat")
                    .arg("run")
                    .arg(script)
                    .output()
                    .expect("");
                println!("{}", HardHat::transform_output(&output));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn transform_output(output: &Output) -> String {
        if output.status.success() {
            let stdout_utf8 = std::str::from_utf8(&output.stdout).unwrap();
            stdout_utf8.to_string()
        } else {
            let stderr_utf8 = std::str::from_utf8(&output.stderr).unwrap();
            stderr_utf8.to_string()
        }
    }
}
