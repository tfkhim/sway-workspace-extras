/*
 * This file is part of sway-workspace-extras
 *
 * Copyright (c) 2022 Thomas Himmelstoss
 *
 * This software is subject to the MIT license. You should have
 * received a copy of the license along with this program.
 */

use std::process::ExitCode;
use std::process::Termination;

use clap::Error as ClapError;
use swayipc::Error as SwayIpcError;
use swayipc::Fallible;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Cli(#[from] ClapError),
    #[error(transparent)]
    SwayIpc(#[from] SwayIpcError),
    #[error("One or more Sway commands failed{}", format_command_errors(.0))]
    SwayCommand(Vec<SwayIpcError>),
}

fn format_command_errors(errors: &[SwayIpcError]) -> String {
    let mut result = String::new();
    for (index, error) in errors.iter().enumerate() {
        let error_number = index + 1;
        result.push_str(&format!("\n{error_number}. {error}"));
    }
    result
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        match self {
            Error::Cli(error) => {
                let _ = error.print();
            }
            _ => {
                eprintln!("{self}");
            }
        };
        ExitCode::FAILURE
    }
}

pub trait CommandErrorConversion {
    fn convert_errors(self) -> Result<(), Error>;
}

impl CommandErrorConversion for Fallible<Vec<Fallible<()>>> {
    fn convert_errors(self) -> Result<(), Error> {
        self.map_err(Error::from).and_then(|results| {
            let errors: Vec<_> = results.into_iter().filter_map(Result::err).collect();
            if errors.is_empty() {
                Ok(())
            } else {
                Err(Error::SwayCommand(errors))
            }
        })
    }
}
