import { BLACK, YELLOW } from "./palette.ts";

type Props = {
    label: string;
    url: string;
};

export function ButtonLink(props: Props) {
    return (
        <a
            href={props.url}
            style={{
                border: `2px solid ${BLACK}`,
                borderRadius: "4px",
                fontWeight: "bold",
                backgroundColor: YELLOW,
                padding: "12px 16px",
                textAlign: "center",
                color: BLACK,
                textDecoration: "none",
                lineHeight: 1,
                boxSizing: "border-box",
                whiteSpace: "nowrap"
            }}
        >
            { props.label }
        </a>
    );
};
