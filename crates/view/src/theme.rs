use once_cell::sync::Lazy;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Background {
    pub primary: String,
    pub secondary: String,
}

#[derive(PartialEq, Clone)]
pub struct Text {
    pub primary: String,
    pub secondary: String,
}

#[derive(PartialEq, Clone)]
pub struct Theme {
    pub backgrounds: Background,
    pub texts: Text,
}

#[derive(PartialEq, Clone)]
pub struct ThemeSet {
    pub light: Theme,
    pub dark: Theme,
}

pub static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| ThemeSet {
    light: Theme {
        backgrounds: Background {
            primary: "white".into(),
            secondary: "rgb(230, 230, 230)".into(),
        },
        texts: Text {
            primary: "black".into(),
            secondary: "rgb(100, 100, 100)".into(),
        },
    },
    dark: Theme {
        backgrounds: Background {
            primary: "rgb(25, 25, 25)".into(),
            secondary: "rgb(50, 50, 50)".into(),
        },
        texts: Text {
            primary: "white".into(),
            secondary: "rgb(200, 200, 200)".into(),
        },
    },
});

#[hook]
pub fn use_theme() -> ThemeSet {
    use_context::<ThemeSet>().expect("failed to read theme.")
}
