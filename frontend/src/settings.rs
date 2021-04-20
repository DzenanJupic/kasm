use std::num::NonZeroUsize;

use seed::prelude::{*, web_sys::{Storage, Window}};
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = localStorage)]
    static LOCAL_STORAGE: Storage;

    #[wasm_bindgen(js_name = window)]
    static WINDOW: Window;
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub max_steps_between_render: NonZeroUsize,
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

    pub fn save_to_storage_and_reload(&self) -> Result<(), JsValue> {
        self.save_to_storage()?;
        WINDOW.location().reload_with_forceget(false)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_steps_between_render: NonZeroUsize::new(100_000).unwrap(),
            continue_after_max_steps: false,
            editor_font_size: 12,
            editor_start_from_line: 1,
            show_instruction_names: true,
            show_data_registers: true,
            show_help: false,
            show_settings: false,
            cpu_mode: CpuMode::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, strum::EnumString, derive_more::Display, strum::EnumVariantNames, strum::IntoStaticStr)]
pub enum CpuMode {
    Integer64,
    // Integer128,
    FloatingPoint64,
}

impl Default for CpuMode {
    fn default() -> Self {
        Self::Integer64
    }
}
