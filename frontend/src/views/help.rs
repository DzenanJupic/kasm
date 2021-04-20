use seed::{*, prelude::*};
use strum::VariantNames;

use crate::{Model, Msg};

pub fn view<IRS>(model: &Model<IRS>) -> Node<Msg> {
    crate::views::popup::view(
        model,
        |model| model.settings.show_help,
        || Msg::ToggleShowHelp,
        div![
            C!["container-fluid"],
        
            div![
                C!["border-bottom", "border-secondary", "fs-1", "fw-bold", "text-center"],
                "Help"
            ],
            view_help_table(
                "Instruction Code",
                "Instruction Name",
                kasm::instruction::Instruction::VARIANTS.iter()
            ),
            view_help_table(
                "Interrupt Code",
                "Interrupt Name",
                kasm::interrupt::Interrupt::VARIANTS.iter()
            )
        ],
    )
}

fn view_help_table(th_lhs: &str, th_rhs: &str, rows: impl Iterator<Item=impl UpdateEl<Msg>>) -> Node<Msg> {
    table![
        C!["table", "table-light", "table-striped", "text-center"],
        
        thead![
            tr![
                th![th_lhs],
                th![th_rhs],
            ]
        ],
        tbody![
            rows
                .enumerate()
                .map(|(i, val)| {
                    tr![
                        td![i],
                        td![val],
                    ]
                })
        ]
    ]
}
