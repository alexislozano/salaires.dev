import { Maybe } from "@utils";
import { Template } from "./Template.tsx";

type Props = {
    text: string;
};

export function TextOnly(props: Props) {
    return (
        <Template notification={Maybe.none()}>
            <div style={{
                display: "flex",
                height: "100%",
                alignItems: "center",
                justifyContent: "center"
            }}>
                { props.text }
            </div>
        </Template>
    )
}