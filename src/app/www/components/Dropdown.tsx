import { Maybe } from "@utils";
import { Error } from "./Error.tsx";
import { Label } from "./Label.tsx";
import { Style } from "./Style.tsx";

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
            style={{
                display: "flex",
                flexDirection: "column",
                gap: "4px",
                width: "100%"
            }}
        >
            <Label
                required={props.required}
                label={props.label}
                subLabel={props.subLabel}
            />
            <select
                id={`${props.name}-select`}
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
            <Error
                error={props.error}
                name={props.name}
            />
            <Style
                error={props.error}
                id={`${props.name}-select`}
            />
        </label>
    );
};
