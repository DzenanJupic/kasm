use seed::{*, prelude::*};

use crate::Msg;

#[wasm_bindgen]
extern "C" {
    fn get_code() -> Option<String>;
    fn set_editor_font_size(font_size: u8);
    fn set_editor_error(row: usize, msg: String);
    fn clear_editor_annotations();
}

#[derive(Debug)]
pub struct Editor;

impl Editor {
    pub fn get_code(&self) -> Option<String> {
        get_code()
    }
    
    #[allow(unused_mut)]
    pub fn set_font_size(&mut self, font_size: u8) {
        set_editor_font_size(font_size);
    }
    
    #[allow(unused_mut)]
    pub fn set_error(&mut self, row: usize, msg: String) {
        set_editor_error(row.saturating_sub(1), msg)
    }
    
    #[allow(unused_mut)]
    pub fn clear_errors(&mut self) {
        clear_editor_annotations()
    }
    
    pub fn view(&self) -> Node<Msg> {
        div![
            C!["col", "pe-0"],
            pre![
                id!("editor"),
                style! { St::Height => "100%" },
                
                include_str!("../../examples/hello_world.kasm")
            ]
        ]
    }
}
