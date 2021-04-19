use seed::{*, prelude::{*, web_sys::Storage}};
use wasm_bindgen::JsValue;

use crate::{Model, Msg};
use crate::model_views::view_popup;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = localStorage)]
    static LOCAL_STORAGE: Storage;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub max_steps_in_a_row: u64,
    pub max_steps_between_render: u64,
    
    pub editor_font_size: u8,
    pub editor_start_from_line: i64,
    
    pub show_instruction_names: bool,
    pub show_data_registers: bool,
    
    pub show_help: bool,
    pub show_settings: bool,
}

#[derive(Clone)]
pub enum SettingMsg {
    ToggleShowInstructionNames,
    ToggleShowDataRegisters,
    ToggleShowHelp,
    ToggleShowSettings,

    SetEditorFontSize(String),
    SetMaxStepsBetweenRender(String),
    SetMaxStepsInRow(String)
}

impl Settings {
    pub fn from_storage() -> Result<Self, JsValue> {
        if let Some(settings) = LOCAL_STORAGE.get_item("settings")? {
            let settings = serde_json::from_str::<Self>(&settings)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            return Ok(settings);
        }

        Err(JsValue::NULL)
    }

    pub fn save_to_storage(&self) -> Result<(), JsValue> {
        LOCAL_STORAGE.set_item(
            "settings",
            &serde_json::to_string(self)
                .expect("Serializing Settings will never fail"),
        )
    }
    
    pub fn update(msg: SettingMsg, settings: &mut Settings) {
        match msg {
            SettingMsg::ToggleShowInstructionNames => {
                settings.show_instruction_names = !settings.show_instruction_names;
            }
            SettingMsg::ToggleShowDataRegisters => {
                settings.show_data_registers = !settings.show_data_registers;
            }
            SettingMsg::ToggleShowHelp => {
                settings.show_help = !settings.show_help;
            }
            SettingMsg::ToggleShowSettings => {
                settings.show_settings = !settings.show_settings;
            }
            SettingMsg::SetEditorFontSize(font_size) => {
                crate::parse_into(&font_size, &mut settings.editor_font_size);
            }
            SettingMsg::SetMaxStepsBetweenRender(max_steps) => {
                crate::parse_into(&max_steps, &mut settings.max_steps_between_render);
            }
            SettingMsg::SetMaxStepsInRow(max_steps) => {
                crate::parse_into(&max_steps, &mut settings.max_steps_in_a_row);
            }
        }
    }

    pub fn view(&self) -> Node<Msg> {
        view_popup(
            self,
            |settings| settings.show_settings,
            || Msg::Setting(SettingMsg::ToggleShowSettings),
            div![
                C!["container-fluid"],
            
                div![
                    C!["border-bottom", "border-secondary", "fs-1", "fw-bold", "text-center"],
                    "Settings"
                ],
                Self::view_setting_switch(
                    "showInstructionNamesSwitch",
                    "Show instruction names instead of instruction codes in RAM",
                    "Show Instruction Names",
                    Msg::Setting(SettingMsg::ToggleShowInstructionNames),
                    self.show_instruction_names
                ),
                Self::view_setting_switch(
                    "showDataRegisters",
                    "Show data registers in the right panel",
                    "Show Data registers",
                    Msg::Setting(SettingMsg::ToggleShowDataRegisters),
                    self.show_data_registers
                ),
                Self::view_setting_input(
                    "setEditorFontSize",
                    "The font size of the code editor",
                    "editor font size",
                    |fs| Msg::Setting(SettingMsg::SetEditorFontSize(fs)),
                    "number",
                    self.editor_font_size
                ),
                Self::view_setting_input(
                    "setMaxStepIterations",
                    "The maximum amount of instructions executed when stepping to end or to the next breakpoint",
                    "max CPU steps in a row [empty for unlimited]",
                    |si| Msg::Setting(SettingMsg::SetMaxStepsInRow(si)),
                    "number",
                    self.max_steps_in_a_row.to_string()
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

}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_steps_in_a_row: 1_000_000,
            max_steps_between_render: 100_000,
            editor_font_size: 12,
            editor_start_from_line: 1,
            show_instruction_names: true,
            show_data_registers: true,
            show_help: false,
            show_settings: false,
        }
    }
}
