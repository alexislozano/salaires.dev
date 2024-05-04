import { PropsWithChildren } from "hono/jsx";

type Props = PropsWithChildren<{
    title: string;
    postUrl: string;
}>;

export function Form(props: Props) {
    return (
        <form
            id="form"
            style={{
                maxWidth: "500px",
                height: "fit-content",
                display: "flex",
                flexDirection: "column",
                gap: "16px",
                padding: "32px",
                margin: "auto"
            }}
            method="post"
            action={props.postUrl}
        >
            <h2 style={{
                fontSize: "32px",
                padding: "16px 32px",
                margin: 0,
                fontWeight: "normal",
                textAlign: "center"
            }}>
                { props.title }
            </h2>
            { props.children }
        </form>
    );
}