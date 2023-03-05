use maud::{html, Markup};

pub fn view(key: &str) -> Markup {
    html! {
        div
            class="h-captcha"
            data-sitekey=(key)
            data-callback="onSuccess"
            style="
                display: flex;
                justify-content: center;"
            hx-trigger="success"
            hx-post="/validate"
            hx-swap="none"
            {}
        script {
            "function onSuccess() { htmx.trigger('.h-captcha', 'success') }"
        }
    }
}
