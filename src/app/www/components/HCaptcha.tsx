type Props = {
    key: string;
    validationUrl: string;
};

export const HCaptcha = (props: Props) => {
    const script = `function onSuccess() {
        htmx.trigger('.h-captcha', 'success')
    }`;

    return (
        <>
            <div
                style={{
                    display: "flex",
                    justifyContent: "center"
                }}
                className="h-captcha"
                data-sitekey={props.key}
                data-callback="onSuccess"
                hx-trigger="success"
                hx-post={props.validationUrl}
                hx-swap="none"
            ></div>
            <script>{ script }</script>
        </>
    );
};
