use bounce::helmet::{Helmet, HelmetBridge};
use bounce::BounceRoot;
use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;

use crate::banner::Banner;
use crate::bg_spin::BackgroundSpinner;
use crate::theme::{use_theme, ThemeSet, THEME_SET};

#[function_component(GlobalStyles)]
fn global_styles() -> Html {
    let theme = use_theme();

    html! {
        <Global css={css!(
            r#"
                html, body {
                    font-family: sans-serif;
                    margin: 0;

                    font-size: 15px;

                    background-color: ${light_bg};
                    color: ${light_text};
                }

                @media (prefers-color-scheme: dark) {
                    html, body {
                        background-color: ${dark_bg};
                        color: ${dark_text};
                    }
                }
            "#,
            light_bg = theme.light.backgrounds.primary,
            dark_bg = theme.dark.backgrounds.primary,

            light_text = theme.light.texts.primary,
            dark_text = theme.dark.texts.primary,
        )} />
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BounceRoot>
            <HelmetBridge />
            <Helmet>
                <meta charset="utf-8" />
                <title>{"Bounce - The uncomplicated Yew State management library"}</title>
                <meta name="viewport" content="width=device-width" />
                <meta name="title" content="Bounce - The uncomplicated Yew State management library" />
                <meta name="description" content="Bounce - The uncomplicated Yew State management library" />
                <meta name="keywords" content="webassembly, rust, yew, state management, bounce" />
            </Helmet>
            <ContextProvider<ThemeSet> context={THEME_SET.clone()}>
                <GlobalStyles />
                <Banner />
                <BackgroundSpinner />
            </ContextProvider<ThemeSet>>
        </BounceRoot>
    }
}
