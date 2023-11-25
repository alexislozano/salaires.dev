import { Maybe } from "@utils";
import { DARK_BLUE, LIGHT_BLUE } from "./palette.ts";

type Props = {
    notification: Maybe<string>;
    deletionUrl: string
};

export const Notification = (props: Props) => {
    return (
        <aside 
            id="notification"
            style={{
                display: display(props.notification),
                position: "absolute",
                zIndex: 1,
                width: "100%",
                backgroundColor: LIGHT_BLUE,
                borderBottom: `2px solid ${DARK_BLUE}`,
                color: DARK_BLUE,
                padding: 8,
                boxSizing: "border-box",
                fontWeight: "bold",
                textAlign: "center"
            }}
            hx-swap-oob="true"
            hx-delete={props.deletionUrl}
            hx-trigger={trigger(props.notification)}
        >
            { Maybe.match(props.notification, {
                onSome: (n) => n,
                onNone: () => ""
            }) }
        </aside>
    );
};

function display(notification: Maybe<string>) {
    return Maybe.match(notification, {
        onSome: () => "block",
        onNone: () => "none"
    });
}

function trigger(notification: Maybe<string>) {
    return Maybe.match(notification, {
        onSome: () => "load delay:2s",
        onNone: () => "none"
    });
}
