use seed::{*, prelude::*};
use crate::Msg;


pub fn view() -> Node<Msg> {
    div![
        style! { St::Height => "30%" },
        C!["flex-grow-1", "row", "d-flex", "flex-column", "justify-content-evenly"],
        
        div![
            C!["row"],
        
            view_control_panel_btn(
                "Run",
                "Resets all registers, clears the RAM, compiles the code, and steps till the end (END)",
                Msg::Run
            ),
            view_control_panel_btn(
                "Compile",
                "Compiles the code in the editor and loads it into RAM",
                Msg::Compile
            ),
            view_control_panel_btn(
                "Reset",
                "Resets all registers to 0 and clears the RAM",
                Msg::Reset
            ),
        ],
        div![
            C!["row"],
        
            view_control_panel_btn(
                "Step",
                "Executes the next instruction",
                Msg::Step
            ),
            view_control_panel_btn(
                "Step to break point",
                "Executes all instructions till the next break point (BP)",
                Msg::StepToBreakpoint
            ),
            view_control_panel_btn(
                "Step to end",
                "Executes all instructions till the end (END)",
                Msg::StepToEnd
            ),
        ]
    ]
}

fn view_control_panel_btn(name: &str, title: &str, msg: Msg) -> Node<Msg> {
    button![
        ev(Ev::Click, move |_| msg),
        C!["col", "m-2", "btn", "btn-primary"],
        attrs! { At::Title => title },
        name
    ]
}
