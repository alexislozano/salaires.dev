import { DARK_BLUE, LIGHT_BLUE } from "./palette.ts";

type Props = { text: string };

export const Banner = (props: Props) => {
    return (
        <p style={{
            padding: 12,
            margin: 0,
            border: `1px solid ${DARK_BLUE}`,
            borderRadius: 4,
            backgroundColor: LIGHT_BLUE,
            color: DARK_BLUE
        }}>
            { props.text }
        </p>
    );
};
