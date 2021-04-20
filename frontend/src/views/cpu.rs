use std::borrow::Borrow;

use itertools::Itertools;
use seed::{*, prelude::*};

use crate::{Model, Msg};

pub fn view_registers<IRS: Copy + UpdateEl<Msg>>(model: &Model<IRS>) -> Node<Msg> {
    div![
            style! { St::Height => if model.settings.show_data_registers { "70%" } else { "40%" } },
            C!["row", "d-flex", "flex-column", "justify-content-center"],
            
            
            div![
                C!["row"],
                
                view_register("A", None, model.cpu.A()),
                div![
                    C!["col", "m-2", "border", "border-primary", "border-3", "text-center", "rounded"],
                    div!["BZ"],
                    div![
                        input![
                            input_ev(Ev::Input, Msg::BZChanged),
                            attrs! {
                                At::Type => "text",
                                At::Value => model.cpu.BZ(),
                            },
                            style! {
                                St::Border => "none",
                            },
                            C!["text-center"],
                        ]
                    ],
                ]
            ],   
            IF!(
                model.settings.show_data_registers =>
                model.cpu.Rx()
                    .iter()
                    .enumerate()
                    .chunks(4)
                    .borrow()
                    .into_iter()
                    .map(|row| {
                        div![
                            C!["row", "mt-2"],
                            row.map(|(i, rx)| view_register("R", Some(i), rx))
                        ]
                    })
                    .collect::<Vec<_>>()
            )
        ]
}

pub fn view_register<T: UpdateEl<Msg>>(name: &str, i: Option<usize>, value: T) -> Node<Msg> {
    div![
            C!["col", "m-2", "border", "border-primary", "border-3", "text-center", "rounded"],
            div![name, i],
            div![value],
        ]
}
