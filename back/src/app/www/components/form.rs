use maud::{html, Markup};

pub fn view(title: &str, post_url: &str, elements: Vec<Markup>) -> Markup {
    html! {
        form
            id="form"
            style="
                max-width: 500px;
                height: fit-content;
                display: flex;
                flex-direction: column;
                gap: 16px;
                padding: 32px;
                margin: auto;"
            method="post"
            action=(post_url)
            {
                h2
                    style="
                        font-size: 32px;
                        padding: 16px 32px;
                        margin: 0;
                        font-weight: normal;
                        text-align: center;"
                    {
                        (title)
                    }
                @for element in &elements {
                    (element)
                }
            }
    }
}
