use seed::{*, prelude::*};
use crate::Msg;

pub fn view<M>(
    model: &M,
    cond: fn(&M) -> bool,
    on_close: fn() -> Msg,
    element: impl UpdateEl<Msg>,
) -> Node<Msg> {
    div![
            C![IF!(!cond(model) => "d-none")],
        
            view_overlay(on_close),
            div![
                style! {
                    St::ZIndex => "2001",
                    St::Height => "80vh",
                    St::Width => "60vw",
                    St::OverflowY => "auto",
                    St::OverflowX => "hidden",
                },
                C![
                    "position-absolute", "top-50", "start-50", "translate-middle",
                    "shadow-lg", "rounded", "bg-white"
                ],
                
                element
            ]
        ]
}

pub fn view_overlay(
    on_close: fn() -> Msg,
) -> Node<Msg> {
    div![
            style! { 
                St::ZIndex => "2000",
                St::Background => "#000",
                St::Opacity => "0.5",
            },
            ev(Ev::Click, move |_| on_close()),
            C!["position-absolute", "vh-100", "vw-100"],
        ]
}
