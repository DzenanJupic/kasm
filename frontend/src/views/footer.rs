use seed::{*, prelude::*};
use crate::Msg;

pub fn view() -> Node<Msg> {
    footer![
            C!["font-monospace", "fw-light", "bg-dark"],
            div![
                C!["container", "text-center", "text-muted"],
                "This site was developed by ",
                a![
                    C!["text-reset"],
                    attrs! { 
                        At::Href => "https://github.com/DzenanJupic",
                        At::Target => "_blank" 
                    },
                    "Dzenan Jupic"
                ],
            ]
        ]
}
