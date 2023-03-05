use maud::{html, Markup};

pub fn view(key: &str) -> Markup {
    html! {
        div
            class="h-captcha"
            data-sitekey=(key)
            style="
                display: flex;
                justify-content: center;"
            {}
    }
}
