use seed::{*, prelude::*};
use crate::Msg;
use crate::console::ConsoleOut;
use kasm::cpu::CPU;
use kasm::instruction::Instruction;
use crate::settings::Settings;
use num_traits::FromPrimitive;

pub fn view(cpu: &CPU<ConsoleOut>, settings: &Settings) -> Node<Msg> {
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
