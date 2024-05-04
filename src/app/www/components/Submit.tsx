import { BLACK, GREY, YELLOW, WHITE } from "./palette.ts";

type Props = {
    disabled: boolean;
    label: string;
}

export function Submit(props: Props) {
    return (
        <input
            id="form-submit"
            style={{
                border: `2px solid ${BLACK}`,
                borderRadius: "4px",
                padding: "12px",
                width: "100%",
                fontWeight: "bold",
                fontFamily: "inherit",
                fontSize: "inherit",
                cursor: cursor(props.disabled),
                color: color(props.disabled),
                borderColor: borderColor(props.disabled),
                backgroundColor: backgroundColor(props.disabled)
            }}
            disabled={props.disabled}
            type="submit"
            value={props.label}
            hx-swap-oob="true"
        />
    )
}

function backgroundColor(disabled: boolean): string {
    if (disabled) {
        return WHITE;
    } else {
        return YELLOW;
    }
}

function borderColor(disabled: boolean): string {
    if (disabled) {
        return GREY;
    } else {
        return BLACK;
    }
}

function color(disabled: boolean): string {
    if (disabled) {
        return GREY;
    } else {
        return BLACK;
    }
}

function cursor(disabled: boolean): string {
    if (disabled) {
        return "not-allowed";
    } else {
        return "pointer";
    }
}
