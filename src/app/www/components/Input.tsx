import { Maybe } from "@utils";
import { BLACK, GREY, RED } from "./palette.ts";
import { Label } from "./Label.tsx";

type Props = {
    error: Maybe<string>;
    name: string;
    inputMode: string;
    label: string;
    subLabel: Maybe<string>;
    placeholder: string;
    required: boolean;
    validationUrl: string;
    value: string;
};

export function Input(props: Props) {
    return (
        <label
            id={props.name} 
            style={{
                display: "flex",
                flexDirection: "column",
                gap: "4px",
                width: "100%"
            }}
            hx-swap-oob="true"
        >
            <Label
                required={props.required}
                label={props.label}
                subLabel={props.subLabel}
            />
            <input
                id={`${name}-input`}
                style={{
                    border: `2px solid ${borderColor(props.error)}`,
                    borderRadius: "4px",
                    fontFamily: "inherit",
                    fontSize: "inherit",
                    padding: "12px",
                    fontWeight: "bold"
                }}
                input-mode={props.inputMode}
                required={props.required}
                value={props.value}
                placeholder={props.placeholder}
                name={props.name}
                hx-post={props.validationUrl}
                hx-swap="none"
            />
            { Maybe.isSome(props.error) &&
                <span style={{
                    color: RED
                }}>
                    { Maybe.unwrap(props.error) }
                </span>
            }
        </label>
    );
};

function borderColor(error: Maybe<string>): string {
    return Maybe.match(error, {
        onSome: () => RED,
        onNone: () => BLACK
    });
}
