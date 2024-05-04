import { html } from "hono/helper";

type Props = {
    hCaptchaKey: string;
    validationUrl: string;
};

export function HCaptcha(props: Props) {
    return (
        <>
            <div
                style={{
                    display: "flex",
                    justifyContent: "center"
                }}
                className="h-captcha"
                data-sitekey={props.hCaptchaKey}
                data-callback="onSuccess"
                hx-trigger="success"
                hx-post={props.validationUrl}
                hx-swap="none"
            ></div>
            {html`
                <script>
                    function onSuccess() {
                        htmx.trigger(\"h-captcha", "success")
                    }
                </script>
            `}
        </>
    );
};
