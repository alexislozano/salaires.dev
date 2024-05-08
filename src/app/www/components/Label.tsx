import { Maybe } from "@utils";
import { I18n } from "../i18n.ts";
import { GREY } from "./palette.ts";

type Props = {
    required: boolean;
    label: string;
    subLabel: Maybe<string>;
};

export function Label(props: Props) {
    return (
        <span style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center"
        }}>
            <span style={{
                fontWeight: "bold"
            }}>
                { props.label }
                { ! props.required &&
                    <span style={{
                        color: GREY
                    }}> - { I18n.translate("optional") }</span> 
                }
            </span>
            { Maybe.isSome(props.subLabel) && 
                <span style={{
                    fontSize: "12px"
                }}>
                    { Maybe.unwrap(props.subLabel) }
                </span>
            }
        </span>
    );
};
