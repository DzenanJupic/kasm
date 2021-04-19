#![feature(or_patterns)]
#![allow(non_snake_case)]

use std::borrow::Borrow;
use std::io::Write;
use std::str::FromStr;

use itertools::Itertools;
use num_traits::FromPrimitive;
use seed::{*, prelude::*};
use seed::prelude::web_sys::Storage;
use strum::VariantNames;

use console::{ConsoleMsg, ConsoleOut};
use editor::EditorMsg;
use kasm::{cpu::{CPU, ExecResult}, Error, lexer::Document, RAM, URS};
use kasm::instruction::Instruction;
use settings::SettingMsg;
use web_cpu::CPUMsg;

use crate::editor::Editor;
use crate::settings::Settings;

mod console;
mod control_panel;
mod editor;
mod settings;
mod web_cpu;
mod model_views;
mod help;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

struct Model {
    cpu: CPU<ConsoleOut>,
    console: ConsoleOut,
    editor: Editor,
    settings: Settings,
}

#[derive(Clone)]
enum Msg {
    Run,
    Reset,
    Compile,

    CPU(CPUMsg),
    Setting(SettingMsg),
    Console(ConsoleMsg),
    Editor(EditorMsg),
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    let console = ConsoleOut::default();
    let cpu = CPU::new(RAM::default(), console.clone());
    let settings = Settings::from_storage().unwrap_or(Settings::default());

    Editor.set_font_size(settings.editor_font_size);

    Model {
        cpu,
        console,
        editor: Editor,
        settings,
    }
}

fn parse_into<T: FromStr + Default>(value: &str, into: &mut T) {
    if let Ok(value) = value.parse::<T>() {
        *into = value;
    } else if value.is_empty() {
        *into = T::default();
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Run => {
            orders
                .send_msg(Msg::Reset)
                .send_msg(Msg::Compile)
                .send_msg(Msg::CPU(CPUMsg::StepToEnd));
        }
        Msg::Reset => {
            orders
                .send_msg(Msg::Console(ConsoleMsg::Clear))
                .send_msg(Msg::CPU(CPUMsg::ResetRegisters))
                .send_msg(Msg::Editor(EditorMsg::ClearErrors));
        }
        Msg::Compile => {
            
        }
        Msg::CPU(cpu_msg) => {
            web_cpu::update_cpu(
                cpu_msg,
                &mut model.cpu,
                &model.settings,
                orders
            )
        }
        Msg::Setting(setting_msg) => {
            Settings::update(
                setting_msg,
                &mut model.settings,
            )
        }
        Msg::Console(console_msg) => {
            ConsoleOut::update(
                console_msg,
                &mut model.console,
            )
        }
        Msg::Editor(editor_msg) => {
            editor::update_editor(
                editor_msg,
                &mut model.editor,
            )
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["d-flex", "flex-column", "vh-100"],
        
        help::view_help(model),
        model.settings.view(),

        model_views::view_header(),
        model_views::view_main(model),
        model_views::view_footer(),
    ]
}

impl Model {
    
    fn view_registers(&self) -> Node<Msg> {
        div![
            style! { St::Height => if self.settings.show_data_registers { "70%" } else { "40%" } },
            C!["row", "d-flex", "flex-column", "justify-content-center"],
            
            
            div![
                C!["row"],
                
                Self::view_register("A", None, self.cpu.A()),
                div![
                    C!["col", "m-2", "border", "border-primary", "border-3", "text-center", "rounded"],
                    div!["BZ"],
                    div![
                        input![
                            input_ev(Ev::Input, |s| Msg::CPU(CPUMsg::BZChanged(s))),
                            attrs! {
                                At::Type => "text",
                                At::Value => self.cpu.BZ(),
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
                self.settings.show_data_registers =>
                self.cpu.Rx()
                    .iter()
                    .enumerate()
                    .chunks(4)
                    .borrow()
                    .into_iter()
                    .map(|row| {
                        div![
                            C!["row", "mt-2"],
                            row.map(|(i, rx)| Self::view_register("R", Some(i), rx))
                        ]
                    })
                    .collect::<Vec<_>>()
            )
        ]
    }

    fn view_register<T: UpdateEl<Msg>>(name: &str, i: Option<usize>, value: T) -> Node<Msg> {
        div![
            C!["col", "m-2", "border", "border-primary", "border-3", "text-center", "rounded"],
            div![name, i],
            div![value],
        ]
    }

    

    
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    seed::App::start(
        "app",
        init,
        update,
        view,
    );
}
