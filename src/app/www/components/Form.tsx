import { PropsWithChildren } from "react";

type Props = {
    title: string;
    postUrl: string;
};

const Form = (props: PropsWithChildren<Props>) => {
    return (
        <form
            id="form"
            style={{
                maxWidth: 500,
                height: "fit-content",
                display: "flex",
                flexDirection: "column",
                gap: 16,
                padding: 32,
                margin: "auto"
            }}
            method="post"
            action={props.postUrl}
        >
            <h2 style={{
                fontSize: 32,
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