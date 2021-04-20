use seed::{*, prelude::*};
use strum::VariantNames;

use crate::Msg;
use crate::settings::CpuMode;
use crate::settings::Settings;

pub fn view(settings: &Settings) -> Node<Msg> {
    crate::views::popup::view(
        settings,
        |settings| settings.show_settings,
        || Msg::ToggleShowSettings,
        div![
                C!["container-fluid"],
            
                div![
                    C!["border-bottom", "border-secondary", "fs-1", "fw-bold", "text-center"],
                    "Settings"
                ],
                view_setting_switch(
                    "showInstructionNamesSwitch",
                    "Show instruction names instead of instruction codes in RAM",
                    "Show Instruction Names",
                    Msg::ToggleShowInstructionNames,
                    settings.show_instruction_names
                ),
                view_setting_switch(
                    "showDataRegisters",
                    "Show data registers in the right panel",
                    "Show Data registers",
                    Msg::ToggleShowDataRegisters,
                    settings.show_data_registers
                ),
                view_setting_input(
                    "setEditorFontSize",
                    "The font size of the code editor",
                    "Editor font size",
                    Msg::SetEditorFontSize,
                    "number",
                    settings.editor_font_size
                ),
                view_setting_input(
                    "setMaxStepsBetweenRender",
                    "The maximum amount of instructions executed between rendering the page",
                    "Max CPU steps between render",
                    Msg::SetMaxStepsBetweenRender,
                    "number",
                    settings
                        .max_steps_between_render
                        .to_string()
                ),
                view_setting_switch(
                    "continueAfterMaxSteps",
                    "Continue to step after the CPU hit the max steps between rendering",
                    "Continue after max CPU steps (allows endless loops)",
                    Msg::ToggleContinueAfterMaxSteps,
                    settings.continue_after_max_steps
                ),
                view_setting_select(
                    "setCpuMode",
                    "The datatype that is used to represent the A register and the data registers",
                    "CPU mode (changes lead to a page reload)",
                    Msg::SetCpuMode,
                    CpuMode::VARIANTS,
                    settings.cpu_mode.into()
                ),
            ]
    )
}

fn view_setting_switch(id: &str, title: &str, label: &str, msg: Msg, value: bool) -> Node<Msg> {
    div![
        C!["input-group", "row", "mx-auto", "my-1"],
        
        label![
            C!["input-group-text", "w-50"],
            attrs! {
                At::For => id,
                At::Title => title,
            },
            
            label
        ],
        
        div![
            C![
                "form-control", "form-switch", "d-flex", "justify-content-center",
                "align-items-center"
             ],
            
            input![
                id!(id),
                C!["form-check-input"],
                attrs! {
                    At::Checked => value.as_at_value(),
                    At::Type => "checkbox",
                },
                ev(Ev::Change, move |_| msg)
            ],
        ]
    ]
}

fn view_setting_input(
    id: &str,
    title: &str,
    label: &str,
    msg: fn(String) -> Msg,
    input_type: &str,
    value: impl std::fmt::Display,
) -> Node<Msg> {
    div![
        C!["input-group", "row", "mx-auto", "my-1"],
        
        label![
            C!["input-group-text", "w-50"],
            attrs! {
                At::For => id,
                At::Title => title,
            },
            
            label
        ],
        input![
            id!(id),
            input_ev(Ev::Input, msg),
            C!["form-control", "w-50", "text-center"],
            attrs! {
                At::Type => input_type,
                At::Value => value,
            },
        ]
    ]
}

fn view_setting_select(
    id: &str,
    title: &str,
    label: &str,
    msg: fn(String) -> Msg,
    options: &[&str],
    value: &str,
) -> Node<Msg> {
    div![
        C!["input-group", "row", "mx-auto", "my-1"],
        
        label![
            C!["input-group-text", "w-50"],
            attrs! {
                At::For => id,
                At::Title => title,
            },
            
            label
        ],
        select![
            id!(id),
            input_ev(Ev::Input, msg),
            C!["form-select", "w-50"],
            style! { St::TextAlignLast => "center" },
            
            options
                .iter()
                .map(|option| {
                    option![
                        C!["text-center"],
                        attrs! {
                            At::Value => option
                            At::Selected => (*option == value).as_at_value()
                        },
                        option
                    ]    
                })
        ]
    ]
}
