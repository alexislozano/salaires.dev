import { Order } from "@domain";
import { BLACK, LIGHT_SAND, SAND } from "./palette.ts";
import { UnreachableCaseError } from "@utils";

export type Column<K> = {
    key: K;
    label: string;
    subLabel: string;
    sortUrl: string;
    pushedUrl: string;
};

type Extract<K> = {
    extract: (key: K) => string;
};

type Props<T extends Extract<K>, K> = {
    items: T[],
    columns: Column<K>[],
    order: Order<K>
};

export const Table = <T extends Extract<K>, K>(props: Props<T, K>) => {
    return (
        <table
            id="table"
            style={{
                borderSpacing: 0,
                width: "100%"
            }}
            hx-swap-oob="true"
        >
            <Head
                columns={props.columns}
                order={props.order}
            />
            <Body
                items={props.items}
                columns={props.columns}
            />
        </table>
    );
}

type HeadProps<K> = Pick<Props<never, K>, "columns" | "order">;

export function Head<K>(props: HeadProps<K>) {
    return (
        <thead
            style={{
                position: "sticky",
                top: 0
            }}
        >
            <tr>
                { props.columns.map(column =>
                    <th
                        style={{
                            padding: 0,
                            backgroundColor: SAND,
                            borderBottom: "2px solid"
                        }}
                    >
                        <button
                            id={`${column.key}-button`}
                            style={{
                                height: "48px",
                                width: "100%",
                                border: 0,
                                backgroundColor: "transparent",
                                padding: "8px 16px",
                                textAlign: "start",
                                cursor: "pointer",
                                fontFamily: "inherit",
                                whiteSpace: "nowrap",
                                display: "flex",
                                flexDirection: "column",
                                color: BLACK
                            }}
                            hx-get={column.sortUrl}
                            hx-swap="none"
                            hx-push-url={column.pushedUrl}
                        >
                            <span style={{fontWeight: "bold"}}>
                                { column.key === props.order.key
                                    ? `${column.label} ${buildArrow(props.order)}`
                                    : column.label
                                }
                            </span>
                            <span style={{
                                fontSize: "12px",
                                fontWeight: "normal"
                            }}>
                                { column.subLabel }
                            </span>
                        </button>
                    </th> 
                )}
            </tr>
        </thead>
    );
}

function buildArrow<K>(order: Order<K>): string {
    switch (order.direction) {
        case "asc": return "↑";
        case "desc": return "↓";
        default: throw new UnreachableCaseError(order.direction);
    }
}

type BodyProps<T extends Extract<K>, K> = Pick<Props<T, K>, "items" | "columns">;

export function Body<T extends Extract<K>, K>(props: BodyProps<T, K>) {
    return (
        <tbody>
            { props.items.map((item, index) =>
                <tr style={{ backgroundColor: backgroundColor(index) }}>
                    { props.columns.map(column =>
                        <td style={{
                            height: "48px",
                            padding: "0 16px",
                            whiteSpace: "nowrap"
                        }}>
                            { item.extract(column.key) }
                        </td>
                    )}
                </tr>
            )}
        </tbody>
    )
}

function backgroundColor(index: number): string {
    if (index % 2 === 0) {
        return LIGHT_SAND;
    } else {
        return SAND;
    }
}
