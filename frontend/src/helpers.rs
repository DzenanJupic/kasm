use std::io::Write;
use std::str::FromStr;

use seed::prelude::*;

use kasm::{cpu::ExecResult, Error};

use crate::{console::ConsoleOut, Msg};
use crate::settings::Settings;

pub fn parse_from_str_into<T: FromStr + Default>(value: &str, into: &mut T) {
    parse_from_str_into_or(value, into, T::default())
}

pub fn parse_from_str_into_or<T: FromStr>(value: &str, into: &mut T, default: T) {
    if let Ok(value) = value.parse::<T>() {
        *into = value;
    } else if value.is_empty() {
        *into = default;
    }
}

pub fn handle_step_to_res(
    res: Result<ExecResult, Error>,
    settings: &Settings,
    not_finished_msg: Msg,
    console: &ConsoleOut,
    orders: &mut impl Orders<Msg>,
) {
    match res {
        Ok(ExecResult::NotFinished) if settings.continue_after_max_steps => {
            orders.perform_cmd(seed::prelude::cmds::timeout(1, move || not_finished_msg));
        }
        Ok(ExecResult::NotFinished) => {
            let err = Error::TooManySteps(settings.max_steps_between_render.get());
            writeln!(console.clone(), "{}", err)
                .expect("ConsoleOut will never fail");
        }
        Ok(ExecResult::Print(ref text)) => {
            console.write_str(text);
        }
        Ok(_) => {}
        Err(err) => {
            writeln!(console.clone(), "{}", err)
                .expect("ConsoleOut will never fail");
        }
    }
}
