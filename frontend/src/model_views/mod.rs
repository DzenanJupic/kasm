use seed::{*, prelude::*};
use crate::{Msg, Model, settings::SettingMsg, web_cpu, control_panel};

pub fn view_header() -> Node<Msg> {
    header![
        C!["navbar", "navbar-dark", "bg-dark"],
        
        div![
            C!["container-fluid"],
            
            a![
                C!["navbar-brand"],
                "KASM - A Klett asm emulator"
            ],
            
            div![
                a![
                    C!["btn", "btn-secondary"],
                    attrs! {
                        At::Href => "https://github.com/DzenanJupic/kasm",
                        At::Target => "_blank", 
                    },
                    raw!(
                        r##"
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
                          <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                        </svg>
                        "##
                    )
                ],
                button![
                    C!["btn", "btn-secondary", "ms-3"],
                    ev(Ev::Click, |_| Msg::Setting(SettingMsg::ToggleShowSettings)),
                    raw!(
                        r##"
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-gear" viewBox="0 0 16 16">
                          <path d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492zM5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0z"/>
                          <path d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52l-.094-.319zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115l.094-.319z"/>
                        </svg>
                        "##
                    )
                ],
                button![
                    C!["btn", "btn-secondary", "ms-3"],
                    ev(Ev::Click, |_| Msg::Setting(SettingMsg::ToggleShowHelp)),
                    raw!(
                        r##"
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-question" viewBox="0 0 16 16">
                          <path d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"/>
                        </svg>
                        "##
                    )
                ]
            ]
        ]
    ]
}

pub fn view_main(model: &Model) -> Node<Msg> {
    main![
        C!["flex-grow-1", "d-flex", "flex-column"],
        div![
            C!["row", "flex-grow-1"],
            
            model.editor.view(),
            web_cpu::view_ram(&model.cpu, &model.settings)
                .map_msg(Msg::CPU),
            div![
                C!["col-6", "d-flex", "flex-column"],
                
                control_panel::view_control_panel(),
                model.view_registers(),
            ]
        ],
        model.console.view(),            
    ]
}

pub fn view_footer() -> Node<Msg> {
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

pub fn view_popup<M>(
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
