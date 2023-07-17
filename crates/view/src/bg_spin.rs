use stylist::yew::styled_component;
use yew::prelude::*;

use crate::theme::use_theme;

#[styled_component(BackgroundSpinner)]
pub fn background_spinner() -> Html {
    let theme = use_theme();
    html! {
        <div class={css!(
            r#"
                position: fixed;
                top: -12.5vw;
                right: -12.5vw;

                @keyframes background-spinner-rotate {
                    0% {
                        transform: rotate(0deg);
                    }
                    50% {
                        transform: rotate(-180deg);
                    }
                    100% {
                        transform: rotate(-360deg);
                    }
                }

                animation: background-spinner-rotate 20s infinite linear;

                background: conic-gradient(${bg_light_sec} 180deg, ${bg_light_prim} 180deg);

                @media (prefers-color-scheme: dark) {
                    background: conic-gradient(${bg_dark_sec} 180deg, ${bg_dark_prim} 180deg);
                }
                border-radius: 50%;

                height: 25vw;
                width: 25vw;

                padding: 30px;
            "#,
            bg_light_prim = theme.light.backgrounds.primary.clone(),
            bg_light_sec = theme.light.backgrounds.secondary,

            bg_dark_prim = theme.dark.backgrounds.primary.clone(),
            bg_dark_sec = theme.dark.backgrounds.secondary,
        )}>
            <div class={css!(
                r#"
                    background: ${bg_light_prim};

                    @media (prefers-color-scheme: dark) {
                        background: ${bg_dark_prim};
                    }

                    border-radius: 50%;

                    height: 100%;
                    width: 100%;
                "#,
                bg_light_prim = theme.light.backgrounds.primary,
                bg_dark_prim = theme.dark.backgrounds.primary,
            )}>
            </div>
        </div>
    }
}
