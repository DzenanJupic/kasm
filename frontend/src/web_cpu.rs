use seed::{*, prelude::*};
use std::io::Write;
use num_traits::FromPrimitive;

use kasm::{Error, URS, instruction::Instruction};
use kasm::cpu::{CPU, ExecResult};

use crate::settings::Settings;
use crate::console::ConsoleOut;
use crate::Msg;

#[derive(Clone)]
pub enum CPUMsg {
    Step,
    StepToEnd,
    StepToBreakpoint,
    ResetRegisters,
    BZChanged(String),
}

pub fn update_cpu(
    msg: CPUMsg,
    cpu: &mut CPU<ConsoleOut>,
    settings: &Settings,
    orders: &mut impl Orders<Msg>
) {
    match msg {
        CPUMsg::Step => { 
            let res = cpu.step();
            handle_step_to_res(
                res,
                CPUMsg::Step,
                cpu.stdout(),
                orders
            );
        }
        CPUMsg::StepToEnd => {
            let res = cpu.step_to_end(settings.max_steps_between_render);
            handle_step_to_res(
                res,
                CPUMsg::StepToEnd,
                cpu.stdout(),
                orders
            );
        }
        CPUMsg::StepToBreakpoint => {
            let res = cpu.step_to_breakpoint(settings.max_steps_between_render);
            handle_step_to_res(
                res,
                CPUMsg::StepToBreakpoint,
                cpu.stdout(),
                orders
            );
        }
        CPUMsg::ResetRegisters => {
            cpu.reset_registers();
        }
        CPUMsg::BZChanged(bz) => {
            if let Ok(bz) = bz.parse::<URS>() {
                cpu.set_BZ(bz);
            } else if bz.is_empty() {
                cpu.set_BZ(0);
            }
        }
    }
}

fn handle_step_to_res(
    res: Result<ExecResult, Error>,
    not_finished_msg: CPUMsg,
    console: &ConsoleOut,
    orders: &mut impl Orders<Msg>
) {
    match res {
        Ok(ExecResult::NotFinished) => {
            orders
                .force_render_now()
                .send_msg(Msg::CPU(not_finished_msg));
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

pub fn view_ram(cpu: &CPU<ConsoleOut>, settings: &Settings) -> Node<CPUMsg> {
    div![
        id!("ram-table"),
        C!["col", "p-0", "bg-secondary",  "overflow-auto", "position-relative"],
        
        table![
            C!["table", "table-dark", "table-striped", "table-sm", "text-center", "position-absolute"],
            
            thead![
                tr![
                    th!["#"],
                    th!["Instruction"],
                    th!["Argument"],
                ]
            ],
            tbody![
                IF!(!cpu.ram().is_empty() =>
                    cpu
                        .ram()
                        .iter()
                        .enumerate()
                        .map(|(i, (inst, val))| {
                            tr![
                                th![i],
                                td![
                                    C![IF!(cpu.BZ() == i as u64 => "table-active")],
                                    match Instruction::from_u64(*inst) {
                                        Some(inst) if settings.show_instruction_names => inst.to_string(),
                                        _ => inst.to_string()
                                    }
                                ],
                                td![
                                    C![IF!(cpu.BZ() == i as u64 => "table-active")],
                                    val
                                ],
                            ]
                        })
                        .collect::<Vec<_>>()
                ),
                IF!(cpu.ram().is_empty() =>
                    tr![
                        th!["0"],
                        td!["-"],
                        td!["-"],
                    ]
                )
            ]
        ]
    ]
}
