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
                justifyContent: "center",
                padding: "32px"
            }}>
                { props.text }
            </div>
        </Template>
    )
}