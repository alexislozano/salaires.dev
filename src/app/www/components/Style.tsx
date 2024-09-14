import { html } from "hono/html";
import { Maybe } from "@utils";
import { BLACK, RED, WHITE } from "./palette.ts";

type Props = {
    error: Maybe<string>;
    id: string,
    additionalStyle?: [string, string][],
};

export function Style(props: Props) {
    return html`
        <style
            id="${props.id}-style"
            hx-swap-oob="true"
        >
            #${props.id} {
                border: 2px solid ${borderColor(props.error)};
                border-radius: 4px;
                font-family: inherit;
                font-size: inherit;
                padding: 12px;
                font-weight: bold;
                background-color: ${WHITE};
                ${additionalStyle(props.additionalStyle)}
            }
        </style>
    `
}

function borderColor(error: Maybe<string>): string {
    return Maybe.match(error, {
        onSome: () => RED,
        onNone: () => BLACK
    });
}

function additionalStyle(style?: [string, string][]): string {
    if (! style) { return ""; }
    return style
        .map(([key, value]) => `${key}: ${value};`)
        .join("\n");
}