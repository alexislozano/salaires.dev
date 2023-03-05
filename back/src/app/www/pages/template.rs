use maud::{html, Markup, DOCTYPE};

use crate::app::www::components::notification;

use super::super::components::{link, palette};
use super::super::I18n;

pub fn view(main: Markup, notification: Option<&str>) -> Markup {
    html! {
        (DOCTYPE)
        html lang="fr" {
            (head())
            (body(main, notification))
        }
    }
}

pub fn head() -> Markup {
    html! {
        head {
            // basic meta
            meta
                charset="utf-8";
            meta
                http-equiv="X-UA-Compatible"
                content="IE-edge";
            meta
                name="viewport"
                content="width=device-width, initial-scale=1.0";
            title {
                "salaires.dev"
            }
            meta
                name="description"
                content="Partagez et comparez votre salaire anonymement";
            link
                rel="canonical"
                href="https://salaires.dev";

            // twitter meta
            meta
                property="twitter:card"
                content="summary";
            meta
                property="twitter:title"
                content="salaires.dev";
            meta
                property="twitter:description"
                content="Partagez et comparez votre salaire anonymement";
            meta
                property="twitter:image"
                content="https://salaires.dev/assets/hero.png";
            meta
                property="twitter:image-alt"
                content="Capture d'écran de salaires.dev";

            // facebook / linkedin meta
            meta
                property="og:type"
                content="website";
            meta
                property="og:title"
                content="salaires.dev";
            meta
                property="og:description"
                content="Partagez et comparez votre salaire anonymement";
            meta
                property="og:image"
                content="https://salaires.dev/assets/hero.png";
            meta
                property="og:image:secure_url"
                content="https://salaires.dev/assets/hero.png";
            meta
                property="og:image:alt"
                content="Capture d'écran de salaires.dev";
            meta
                property="og:url"
                content="https://salaires.dev";
        }
    }
}

pub fn body(main: Markup, notification: Option<&str>) -> Markup {
    html! {
        body
            style=(format!("
                    background-color: {background_color};
                    font-family: Open Sans, Helvetica, Verdana, sans-serif;
                    font-size: 14px;
                    margin: 0;
                    height: 100vh;
                    display: flex;
                    flex-direction: column;",
                background_color=palette::SAND
            ))
        {
            (header())
            main
                style="
                    overflow: auto;
                    flex-grow: 1;"
                {
                    (notification::view(notification))
                    (main)
                }
            (script())
        }
    }
}

fn header() -> Markup {
    html! {
        nav
            style=(format!("
                    background-color: {background_color};
                    border-bottom: 2px solid {border_color};
                    display: flex;
                    justify-content: space-between;
                    padding: 8px;",
                background_color=palette::PEACH,
                border_color=palette::BLACK
            ))
            {
                div
                    style="
                        display: flex;
                        gap: 8px"
                    {
                        (link::view("salaires.dev", "/"))
                        (link::view("Github", "https://github.com/alexislozano/salaires.dev"))
                    }
                (link::view(I18n::IAddMySalary.translate(), "/insert"))
            }
    }
}

fn script() -> Markup {
    html! {
        script
            src="https://unpkg.com/htmx.org@1.8.6"
            {}
        script
            src="https://hcaptcha.com/1/api.js"
            async
            defer
            {}
    }
}
