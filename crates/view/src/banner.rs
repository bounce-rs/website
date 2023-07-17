use stylist::yew::styled_component;
use yew::prelude::*;

use yew_feather::Book;
use yew_feather::Github;
use yew_feather::Package;

use crate::theme::use_theme;

#[styled_component(Cargo)]
fn cargo() -> Html {
    html! {
        <div class={css!(
            r#"
                border-radius: 0.2rem;

                background-color: rgb(53, 54, 60);
                color: white;
            "#,
        )}>
            <pre class={css!(r#"
                font-size: 1.3rem;
                padding-left: 2rem;
                padding-right: 2rem;
            "#)}>
                <code class={css!(r#"
                    user-select: none;
                    -webkit-user-select: none;
                "#)}>
                    {"> "}
                </code>
                <code>
                    {"cargo add bounce"}
                </code>
            </pre>
        </div>
    }
}

#[styled_component(Actions)]
fn actions() -> Html {
    let theme = use_theme();

    html! {
        <div class={css!(r#"

            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: center;

            @media (min-width: 700px) {
                height: 50px;
            }

            @media (max-width: 700px) {
                flex-direction: column;
            }

            padding-left: 1rem;
            padding-right: 1rem;
        "#)}>
            <a href="https://github.com/bounce-rs/bounce" class={css!(r#"
                text-decoration: none;
            "#)}>
                <div class={css!(r#"
                    padding-left: 30px;
                    padding-right: 30px;

                    height: 50px;
                    border-radius: 0.3rem;
                    background-color: rgb(46, 51, 58);

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    min-width: 10rem;
                    cursor: pointer;

                    color: white;

                    user-select: none;
                    -webkit-user-select: none;

                    box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0);
                    transition: box-shadow 0.2s ease-out, transform 0.2s ease-out;

                    transform: translate(0px, 0px);

                    :hover {
                        box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.5);
                        transform: translate(0px, -3px);
                    }
                "#)}>
                    <Github size="24" />
                    <div class={css!(r#"width: 0.5rem;"#)} />
                    {"GitHub"}
                </div>
            </a>
            <div class={css!(r#"
                width: 15px;
                height: 15px;
            "#)} />
            <a href="https://book.bounce-rs.org" class={css!(r#"
                text-decoration: none;
            "#)}>
                <div class={css!(r#"
                    padding-left: 30px;
                    padding-right: 30px;

                    height: 50px;
                    border-radius: 0.3rem;
                    background-color: rgb(113, 173, 255);

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    min-width: 10rem;
                    cursor: pointer;

                    color: white;

                    user-select: none;
                    -webkit-user-select: none;

                    box-shadow: 0 3px 10px 0 rgba(113, 173, 255, 0);
                    transition: box-shadow 0.2s ease-out, transform 0.2s ease-out;

                    transform: translate(0px, 0px);

                    :hover {
                        box-shadow: 0 3px 10px 0 rgba(113, 173, 255, 0.7);
                        transform: translate(0px, -3px);
                    }

                    @media (prefers-color-scheme: dark) {
                        background-color: rgb(40, 70, 111);

                        box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0);
                        transition: box-shadow 0.2s ease-out, transform 0.2s ease-out;

                        transform: translate(0px, 0px);

                        :hover {
                            box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.5);
                            transform: translate(0px, -3px);
                        }
                    }
                "#)}>
                    <Book size="24" />
                    <div class={css!(r#"width: 0.5rem;"#)} />
                    {"Book"}
                </div>
            </a>
            <div class={css!(r#"
                width: 15px;
                height: 15px;
            "#)} />
            <a href="https://docs.rs/bounce" class={css!(r#"
                text-decoration: none;
            "#)}>
                <div class={css!(
                    r#"
                        padding-left: 30px;
                        padding-right: 30px;

                        height: 50px;
                        border-radius: 0.3rem;
                        background-color: rgb(231, 195, 179);

                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        justify-content: center;

                        min-width: 10rem;
                        cursor: pointer;

                        user-select: none;
                        -webkit-user-select: none;

                        box-shadow: 0 3px 10px 0 rgba(231, 195, 179, 0);
                        transition: box-shadow 0.2s ease-out, transform 0.2s ease-out;

                        transform: translate(0px, 0px);

                        :hover {
                            box-shadow: 0 3px 10px 0 rgba(231, 195, 179, 0.7);
                            transform: translate(0px, -3px);
                        }

                        color: ${text_light};

                        @media (prefers-color-scheme: dark) {
                            background-color: rgb(66, 49, 47);

                            color: ${text_dark};

                            box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0);
                            transition: box-shadow 0.2s ease-out, transform 0.2s ease-out;

                            transform: translate(0px, 0px);

                            :hover {
                                box-shadow: 0 3px 10px 0 rgba(0, 0, 0, 0.5);
                                transform: translate(0px, -3px);
                            }
                        }
                    "#,
                    text_light = theme.light.texts.primary,
                    text_dark = theme.dark.texts.primary,
                )}>
                    <Package size="24" />
                    <div class={css!(r#"width: 0.5rem;"#)} />
                    {"API Docs"}
                </div>
            </a>
        </div>
    }
}

#[styled_component(Banner)]
pub fn banner() -> Html {
    html! {
        <div class={css!(r#"
            min-height: 100vh;
            width: 100%;

            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        "#)}>
            <div class={css!(r#"
                min-height: 100%;

                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            "#)}>
                <div class={css!(r#"
                    font-size: 3rem;
                "#)}>
                    {"Bounce"}
                </div>
                <div class={css!(r#"
                    height: 20px;
                "#)} />
                <div class={css!(r#"
                    font-size: 1.3rem;
                    padding-left: 1rem;
                    padding-right: 1rem;
                    text-align: center;
                "#)}>
                    {"The uncomplicated "}
                    <span class={css!(r#"color: rgb(95, 149, 95);"#)}>
                        {"Yew"}
                    </span>
                    {" State management library."}
                </div>

                <div class={css!(r#"
                    height: 40px;
                "#)} />

                <Cargo />

                <div class={css!(r#"
                    height: 60px;
                "#)} />

                <Actions />
            </div>
        </div>
    }
}
