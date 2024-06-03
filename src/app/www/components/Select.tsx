import { Maybe } from "@utils";
import { Error } from "./Error.tsx";
import { Label } from "./Label.tsx";
import { Style } from "./Style.tsx";

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
                subLabel={Maybe.none()}
            />
            <input
                id={`${props.name}-select`}
                list={`${props.name}-list`}
                placeholder={props.placeholder}
                value={props.value}
                name={props.name}
                autoComplete="off"
                hx-post={props.validationUrl}
                hx-swap="none"
            />
            <Error
                error={props.error}
                name={props.name}
            />
            <Style
                error={props.error}
                id={`${props.name}-select`}
            />
            <datalist id={`${props.name}-list`}>
                { props.options.map(option =>
                    <option value={option}></option>
                ) }
            </datalist>
        </label>
    );
};
