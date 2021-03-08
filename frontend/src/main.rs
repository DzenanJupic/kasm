use std::borrow::Borrow;
use std::io::Write;

use itertools::Itertools;
use num_traits::FromPrimitive;
use seed::{*, prelude::*};
use strum::VariantNames;
use wasm_bindgen::prelude::*;

use console::ConsoleOut;
use kasm::{cpu::{CPU, ExecResult}, Error, lexer::Document, RAM, Result};
use kasm::instruction::Instruction;

mod console;

const MAX_STEPS: u64 = 100_000;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn get_editor_value() -> Option<String>;
}

struct Model {
    cpu: CPU<ConsoleOut>,
    console: ConsoleOut,

    show_instruction_names: bool,
    show_help: bool,
}

#[derive(Clone, Copy)]
enum Msg {
    Step,
    StepToBreakPoint,
    StepToEnd,

    Reset,
    Compile,

    ShowInstructionNames,
    ShowInstructionCodes,

    ShowHelp,
    CloseHelp,
}

impl Model {
    fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Self {
        let console = ConsoleOut::default();
        let cpu = CPU::new(RAM::default(), console.clone());

        Self {
            cpu,
            console,

            show_instruction_names: false,
            show_help: false,
        }
    }

    fn update(msg: Msg, model: &mut Self, _orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Step => {
                match model.cpu.step() {
                    Ok(res) => {
                        if let ExecResult::Print(text) = res {
                            writeln!(model.console, "{}", text).unwrap();
                        }
                    }
                    Err(e) => {
                        writeln!(model.console, "{}", e).unwrap();
                    }
                }
            }
            Msg::StepToBreakPoint => {
                if let Err(e) = model.cpu.step_to_breakpoint(MAX_STEPS).map(|_| ExecResult::None) {
                    writeln!(model.console, "{}", e).unwrap();
                }
            }
            Msg::StepToEnd => {
                if let Err(e) = model.cpu.step_to_end(MAX_STEPS).map(|_| ExecResult::None) {
                    writeln!(model.console, "{}", e).unwrap();
                }
            }

            Msg::Reset => {
                model.console = ConsoleOut::default();
                model.cpu = CPU::new(RAM::default(), model.console.clone());
            }
            Msg::Compile => {
                let code = get_editor_value();

                if let Some(code) = code {
                    match Document::from_str(&code) {
                        Ok(doc) => {
                            let ram = doc.as_ram();
                            model.cpu.set_ram(ram);
                        }
                        Err(err) => {
                            writeln!(model.console, "{}", err).unwrap();
                        }
                    }
                }
            }

            Msg::ShowInstructionNames => model.show_instruction_names = true,
            Msg::ShowInstructionCodes => model.show_instruction_names = false,

            Msg::ShowHelp => model.show_help = true,
            Msg::CloseHelp => model.show_help = false,
        }
    }

    fn view(&self) -> Vec<Node<Msg>> {
        nodes![
            self.view_help(),

            Self::view_header(),
            self.view_main(),
            Self::view_footer(),
        ]
    }

    fn view_header() -> Node<Msg> {
        header![
            C!["fixed-top", "navbar", "navbar-dark", "bg-dark"],
            
            div![
                C!["container-fluid"],
                
                a![
                    C!["navbar-brand"],
                    "KASM - A Klett asm emulator"
                ],
                
                div![
                    a![
                        C!["btn", "btn-secondary"],
                        attrs! {
                            At::Href => "https://github.com/DzenanJupic/kasm",
                            At::Target => "_blank", 
                        },
                        raw!(
                            r##"
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
                              <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                            </svg>
                            "##
                        )
                    ],
                    button![
                        C!["btn", "btn-secondary", "ms-3"],
                        ev(Ev::Click, |_| Msg::ShowHelp),
                        raw!(
                            r##"
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-question" viewBox="0 0 16 16">
                              <path d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"/>
                            </svg>
                            "##
                        )
                    ]
                ]
            ]
        ]
    }

    fn view_main(&self) -> Node<Msg> {
        main![
            C!["vh-100", "d-flex", "flex-column"],
            div![
                C!["row", "flex-grow-1"],
                
                self.view_code_editor(),
                self.view_ram(),
                div![
                    C!["col"],
                    
                    self.view_control_panel(),
                    self.view_registers(),
                ]
            ],
            self.view_console(),            
        ]
    }

    fn view_code_editor(&self) -> Node<Msg> {
        div![
            C!["col", "p-0"],
            pre![
                id!("editor"),
                style! { St::Height => "100%" },
                
                include_str!("../../examples/hello_world.kasm")
            ]
        ]
    }

    fn view_ram(&self) -> Node<Msg> {
        div![
            id!("ram-table"),
            C!["col", "p-0", "bg-secondary",  "overflow-auto"],
            
            table![
                C!["table", "table-dark", "table-striped", "table-sm", "text-center"],
                
                thead![
                    tr![
                        th!["#"],
                        th!["Instruction"],
                        th!["Argument"],
                    ]
                ],
                tbody![
                    if !self.cpu.ram().is_empty() {
                        self.cpu
                            .ram()
                            .iter()
                            .enumerate()
                            .map(|(i, (inst, val))| {
                                tr![
                                    th![i],
                                    td![
                                        C![IF!(self.cpu.BZ() == i as u64 => "table-active")],
                                        match Instruction::from_u64(*inst) {
                                            Some(inst) if self.show_instruction_names => inst.to_string(),
                                            _ => inst.to_string()
                                        }
                                    ],
                                    td![
                                        C![IF!(self.cpu.BZ() == i as u64 => "table-active")],
                                        val
                                    ],
                                ]
                            })
                            .collect::<Vec<_>>()
                    } else {
                        nodes![]
                    },
                    if self.cpu.ram().is_empty() {
                        tr![
                            th!["0"],
                            td!["-"],
                            td!["-"],
                        ]
                    } else {
                        empty![]
                    }
                ]
            ]
            
        ]
    }

    fn view_control_panel(&self) -> Node<Msg> {
        div![
            C!["row", "h-50", "d-flex", "flex-column", "justify-content-evenly"],
            
            div![
                C!["row"],
            
                Self::view_control_panel_btn("Compile", Msg::Compile),
                Self::view_control_panel_btn("Reset", Msg::Reset),
            ],
            div![
                C!["row"],
            
                Self::view_control_panel_btn("Step", Msg::Step),
                Self::view_control_panel_btn("Step to break point", Msg::StepToBreakPoint),
                Self::view_control_panel_btn("Step to end", Msg::StepToEnd),
            ],
            div![
                C!["row"],
            
                Self::view_control_panel_btn("Show Instruction names", Msg::ShowInstructionNames),
                Self::view_control_panel_btn("Show Instruction codes", Msg::ShowInstructionCodes),
            ]
        ]
    }

    fn view_control_panel_btn(name: &str, msg: Msg) -> Node<Msg> {
        button![
            ev(Ev::Click, move |_| msg),
            C!["col", "m-2", "btn", "btn-primary"],
            name
        ]
    }

    fn view_registers(&self) -> Node<Msg> {
        div![
            C!["row", "p-5", "h-50", "d-flex", "flex-column", "justify-content-center"],
            
            
            div![
                C!["row"],
                
                Self::view_register("A", None, self.cpu.A()),
                div![C!["col", "m-2"]],
                div![C!["col", "m-2"]],
                Self::view_register("BZ", None, self.cpu.BZ()),
            ],    
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
            
        ]
    }

    fn view_register<T: UpdateEl<Msg>>(name: &str, i: Option<usize>, value: T) -> Node<Msg> {
        div![
            C!["col", "m-2", "border", "border-primary", "border-3", "text-center", "rounded"],
            div![name, i],
            div![value],
        ]
    }

    fn view_console(&self) -> Node<Msg> {
        div![
            id!("console"),
            pre![
                style! {
                    St::Background => "#000",
                    St::WhiteSpace => "pre-wrap",
                    St::Height => "10em",
                    
                },
                C!["m-0", "px-3", "py-2", "text-white", "overflow-auto"],
                self.console
                    .read()
                    .lines()
                    .map(|line| String::from("> ") + line + "\n")
                    .collect::<String>()
            ]
        ]
    }

    fn view_footer() -> Node<Msg> {
        footer![
            C!["fixed-bottom", "font-monospace", "fw-light", "bg-dark"],
            div![
                C!["container", "text-center", "text-muted"],
                "This site was developed by ",
                a![
                    C!["text-reset"],
                    attrs! { 
                        At::Href => "https://github.com/DzenanJupic",
                        At::Target => "_blank" 
                    },
                    "Dzenan Jupic"
                ],
            ]
        ]
    }

    fn view_help(&self) -> Node<Msg> {
        div![
            style! { St::ZIndex => "2000" },
            ev(Ev::Click, |_| Msg::CloseHelp),
            C!["position-absolute", "vh-100", "vw-100", IF!(!self.show_help => "d-none")],
            
            div![
                style! { St::ZIndex => "2001" },
                ev(Ev::Click, |e| e.stop_propagation()),
                C![
                    "position-absolute", "top-50", "start-50", "translate-middle", "w-50",
                    "shadow-lg", "rounded", "bg-white"
                ],
                
                div![
                    C!["border-bottom", "border-secondary", "fs-1", "fw-bold", "text-center"],
                    "Help"
                ],
                div![
                    C!["row"],
                
                    div![
                        C!["col", "border-right", "border-secondary"],
                        
                        table![
                            C!["table", "table-light", "table-striped", "text-center"],
                            
                            thead![
                                tr![
                                    th!["Instruction Code"],
                                    th!["Instruction Name"]
                                ]
                            ],
                            tbody![
                                kasm::instruction::Instruction::VARIANTS
                                    .iter()
                                    .enumerate()
                                    .map(|(i, inst)| {
                                        tr![
                                            td![i],
                                            td![inst],
                                        ]
                                    })
                            ]
                        ]
                        
                    ],
                    div![
                        C!["col", "border-right", "border-secondary"],
                        
                        table![
                            C!["table", "table-light", "table-striped", "text-center"],
                            
                            thead![
                                tr![
                                    th!["Interrupt Code"],
                                    th!["Interrupt Name"],
                                ]
                            ],
                            tbody![
                                kasm::interrupt::Interrupt::VARIANTS
                                    .iter()
                                    .enumerate()
                                    .map(|(i, inst)| {
                                        tr![
                                            td![i],
                                            td![inst],
                                        ]
                                    })
                            ]
                        ]
                        
                    ],
                ]
                
            ]
        ]
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    seed::App::start(
        "app",
        Model::init,
        Model::update,
        Model::view,
    );
}
