use seed::prelude::{*, web_sys::Storage};
use wasm_bindgen::JsValue;
use std::num::NonZeroU64;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = localStorage)]
    static LOCAL_STORAGE: Storage;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub max_steps_between_render: NonZeroU64,
    pub continue_after_max_steps: bool,
    
    pub editor_font_size: u8,
    pub editor_start_from_line: i64,
    
    pub show_instruction_names: bool,
    pub show_data_registers: bool,
    
    pub show_help: bool,
    pub show_settings: bool,
    
    #[serde(default)]
    pub cpu_mode: CpuMode
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_steps_between_render: NonZeroU64::new(100_000).unwrap(),
            continue_after_max_steps: false,
            editor_font_size: 12,
            editor_start_from_line: 1,
            show_instruction_names: true,
            show_data_registers: true,
            show_help: false,
            show_settings: false,
            cpu_mode: CpuMode::default()
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum CpuMode {
    Integer64,
    Integer128,
    FloatingPoint64,
}

impl Default for CpuMode {
    fn default() -> Self {
        Self::Integer64
    }
}
