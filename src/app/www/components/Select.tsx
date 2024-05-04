import { Maybe } from "@utils";
import { Label } from "./Label.tsx";
import { BLACK, RED, WHITE } from "./palette.ts";

type Props = {
    error: Maybe<string>;
    name: string;
    label: string;
    options: string[];
    placeholder: string;
    required: boolean;
    validationUrl: string;
    value: string;
};

export function Select(props: Props) {
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
                subLabel={Maybe.none()}
            />
            <input
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
                list={`${props.name}-list`}
                placeholder={props.placeholder}
                value={props.value}
                name={props.name}
                autoComplete="off"
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
            <datalist id={`${props.name}-list`}>
                { props.options.map(option =>
                    <option value={option}></option>
                ) }
            </datalist>
        </label>
    );
};

function borderColor(error: Maybe<string>): string {
    return Maybe.match(error, {
        onSome: () => RED,
        onNone: () => BLACK
    });
}
