use maud::{html, Markup};

use super::template;

pub fn view(text: &str) -> Markup {
    let main = html! {
        div
            style="
                display: flex;
                height: 100%;
                align-items: center;
                justify-content: center;"
            {
                (text)
            }
    };

    template::view(main, None)
}
