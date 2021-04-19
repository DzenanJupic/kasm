#![allow(non_snake_case)]
use std::io::Write;
use seed::{*, prelude::*};

use console::{ConsoleOut};
use kasm::{cpu::CPU, RAM, Error};

use crate::editor::Editor;
use crate::settings::{Settings, CpuMode};
use std::num::NonZeroU64;

mod console;
mod editor;
mod settings;

mod helpers;
mod views;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

pub struct Model {
    cpu: CPU<ConsoleOut>,
    console: ConsoleOut,
    editor: Editor,
    settings: Settings,
}

#[derive(Clone)]
pub enum Msg {
    Run,
    Reset,
    Compile,

    Step,
    StepToEnd,
    StepToBreakpoint,
    ResetRegisters,
    BZChanged(String),

    ToggleShowInstructionNames,
    ToggleShowDataRegisters,
    ToggleShowHelp,
    ToggleShowSettings,
    ToggleContinueAfterMaxSteps,

    SetEditorFontSize(String),
    SetMaxStepsBetweenRender(String),
    
    ClearConsole,

    SetError {
        line: usize,
        msg: String
    },
    ClearErrors
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

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Run => {
            orders
                .send_msg(Msg::ClearConsole)
                .send_msg(Msg::ResetRegisters)
                .send_msg(Msg::ClearErrors)
                .send_msg(Msg::Compile)
                .send_msg(Msg::StepToEnd);
        }
        Msg::Reset => {
            orders
                .send_msg(Msg::ClearConsole)
                .send_msg(Msg::ResetRegisters)
                .send_msg(Msg::ClearErrors);
        }
        Msg::Compile => {
            if let Some(ref code) = model.editor.get_code() {
                match kasm::lexer::Document::from_str(code) {
                    Ok(doc) => {
                        *model.cpu.ram_mut() = doc.as_ram();
                    }
                    Err(err) => {
                        writeln!(model.console, "{}", err)
                            .expect("Writing to console will never fail");
                        
                        if let Error::ParsingFailed { line, err, .. } |
                        Error::InvalidTokenArrangement { line, err } = err {
                            model.editor.set_error(line, format!("{}", err));
                        }
                    }
                }
            }
        }
        
        Msg::Step => helpers::handle_step_to_res(
            model.cpu.step(),
            &model.settings,
            Msg::Step,
            &model.console,
            orders
        ),
        Msg::StepToEnd => helpers::handle_step_to_res(
            model.cpu.step_to_end(model.settings.max_steps_between_render), 
            &model.settings,
            Msg::StepToEnd,
            &model.console,
            orders
        ),
        Msg::StepToBreakpoint => helpers::handle_step_to_res(
            model.cpu.step_to_breakpoint(model.settings.max_steps_between_render),
            &model.settings,
            Msg::Step,
            &model.console,
            orders
        ),
        Msg::ResetRegisters => model.cpu.reset_registers(),
        Msg::BZChanged(s) => helpers::parse_from_str_into(&s, model.cpu.BZ_mut()),
        
        Msg::ToggleShowInstructionNames => model.settings.show_instruction_names ^= true,
        Msg::ToggleShowDataRegisters => model.settings.show_data_registers ^= true,
        Msg::ToggleShowHelp => model.settings.show_help ^= true,
        Msg::ToggleShowSettings => model.settings.show_settings ^= true,
        Msg::ToggleContinueAfterMaxSteps => model.settings.continue_after_max_steps ^= true,
        Msg::SetEditorFontSize(s) => helpers::parse_from_str_into(&s, &mut model.settings.editor_font_size),
        Msg::SetMaxStepsBetweenRender(s) => helpers::parse_from_str_into_or(&s, &mut model.settings.max_steps_between_render, NonZeroU64::new(1).unwrap()),
        
        Msg::ClearConsole => model.console.clear(),
        Msg::SetError { line, msg } => model.editor.set_error(line, msg),
        Msg::ClearErrors => model.editor.clear_errors()
    }
    
    // fixme
    let _ = model.settings.save_to_storage();
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["d-flex", "flex-column", "vh-100"],
        
        views::help::view(model),
        views::settings::view(&model.settings),

        views::header::view(),
        views::main::view(&model),
        views::footer::view(),
    ]
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    
    let cpu_mode = Settings::from_storage()
        .unwrap_or_else(|_| {
            let settings = Settings::default();
            let _ = settings.save_to_storage();
            settings
        })
        .cpu_mode;
    
    match cpu_mode {
        CpuMode::Integer64 => {
            
        }
        CpuMode::Integer128 => {}
        CpuMode::FloatingPoint64 => {}
    }
    
    seed::App::start(
        "app",
        init,
        update,
        view,
    );
}
