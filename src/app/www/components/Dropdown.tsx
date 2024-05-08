import { Maybe } from "@utils";
import { Label } from "./Label.tsx";
import { BLACK, RED, WHITE } from "./palette.ts";

type Props = {
    error: Maybe<string>;
    name: string;
    label: string;
    subLabel: Maybe<string>;
    choices: { key: string; label: string}[];
    required: boolean;
    validationUrl: string;
    value: string;
};

export function Dropdown(props: Props) {
    const choices = [
        ...props.required ? [] : [{ key: "", label: "-" }],
        ...props.choices
    ];
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
            <select
                id={`${props.name}-select`}
                style={{
                    border: `2px solid ${borderColor(props.error)}`,
                    borderRadius: "4px",
                    fontFamily: "inherit",
                    fontSize: "inherit",
                    padding: "12px",
                    fontWeight: "bold",
                    backgroundColor: WHITE
                }}
                name={props.name}
                hx-post={props.validationUrl}
                hx-swap="none"
            >
                { choices.map(choice =>
                    <option
                        value={choice.key}
                        selected={choice.key === props.value}
                    >
                        { choice.label }
                    </option>
                )}
            </select>
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
