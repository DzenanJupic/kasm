use seed::{*, prelude::*};

use crate::Msg;

#[wasm_bindgen]
extern "C" {
    fn get_editor_value() -> Option<String>;
    fn set_editor_font_size(font_size: u8);
    fn set_editor_error(row: i64, msg: String);
    fn clear_editor_annotations();
}

pub struct Editor;

impl Editor {
    pub fn get_value(&self) -> Option<String> {
        get_editor_value()
    }
    
    #[allow(unused_mut)]
    pub fn set_font_size(&mut self, font_size: u8) {
        set_editor_font_size(font_size);
    }
    
    #[allow(unused_mut)]
    pub fn set_error(&mut self, row: i64, msg: String) {
        set_editor_error(row, msg)
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

#[derive(Clone)]
pub enum EditorMsg {
    SetError {
        line: i64,
        msg: String
    },
    ClearErrors
}

pub fn update_editor(msg: EditorMsg, editor: &mut Editor) {
    match msg {
        EditorMsg::SetError { line, msg } => {
            editor.set_error(line, msg);
        }
        EditorMsg::ClearErrors => {
            editor.clear_errors();
        }
    }
}
