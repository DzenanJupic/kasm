use seed::{*, prelude::*};

use crate::{Model, Msg};

pub fn view<IRS: Copy + UpdateEl<Msg>>(model: &Model<IRS>) -> Node<Msg> {
    main![
        C!["flex-grow-1", "d-flex", "flex-column"],
        div![
            C!["row", "flex-grow-1"],
            
            model.editor.view(),
            crate::views::ram::view(&model.cpu, &model.settings),
            div![
                C!["col-6", "d-flex", "flex-column"],
                
                crate::views::control_panel::view(),
                crate::views::cpu::view_registers(model),
            ]
        ],
        model.console.view(),            
    ]
}
