use num_traits::FromPrimitive;
use seed::{*, prelude::*};

use kasm::cpu::CPU;
use kasm::instruction::Instruction;

use crate::console::ConsoleOut;
use crate::Msg;
use crate::settings::Settings;

pub fn view<IRS: Copy + UpdateEl<Msg>>(cpu: &CPU<IRS, ConsoleOut>, settings: &Settings) -> Node<Msg> {
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
                                    C![IF!(cpu.BZ() == i as usize => "table-active")],
                                    match Instruction::from_usize(*inst) {
                                        Some(inst) if settings.show_instruction_names => inst.to_string(),
                                        _ => inst.to_string()
                                    }
                                ],
                                td![
                                    C![IF!(cpu.BZ() == i as usize => "table-active")],
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
