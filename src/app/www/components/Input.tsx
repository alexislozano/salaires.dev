import { Maybe } from "@utils";
import { Error } from "./Error.tsx";
import { Label } from "./Label.tsx";
import { Style } from "./Style.tsx";

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
            <input
                id={`${props.name}-input`}
                input-mode={props.inputMode}
                required={props.required}
                value={props.value}
                placeholder={props.placeholder}
                name={props.name}
                hx-post={props.validationUrl}
                hx-swap="none"
            />
            <Error
                error={props.error}
                name={props.name}
            />
            <Style
                error={props.error}
                id={`${props.name}-input`}
            />
        </label>
    );
};
