import { Maybe } from "@utils";
import { RED } from "./palette.ts";

type Props = {
    error: Maybe<string>;
    name: string,
};

export function Error(props: Props) {
    return (
        <span
            id={`${props.name}-error`}
            style={{
                color: RED
            }}
            hx-swap-oob="true"
        >
            { Maybe.isSome(props.error) && Maybe.unwrap(props.error) }
        </span>
    )
}
