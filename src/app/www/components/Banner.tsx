import { DARK_BLUE, LIGHT_BLUE } from "./palette.ts";

type Props = { text: string };

export function Banner(props: Props) {
    return (
        <p style={{
            padding: "12px",
            margin: 0,
            border: `1px solid ${DARK_BLUE}`,
            borderRadius: "4px",
            backgroundColor: LIGHT_BLUE,
            color: DARK_BLUE
        }}>
            { props.text }
        </p>
    );
};
