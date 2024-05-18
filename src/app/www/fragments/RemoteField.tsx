import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Remote, RemoteError } from "@domain";
import { Internals, Parsed } from "../models/mod.ts";
import { Label } from "../components/mod.ts";
import { BLACK, RED, WHITE } from "../components/palette.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<{ variant: string, dayCount: string, base: string, location: string }, Maybe<Remote>, RemoteError>
};

export function RemoteField(props: Props) {
    const validationUrl = "/validate";
    const error = buildError(props.internals.parsed);
    return (
        <label
            id="remote"
            style={{
                display: "flex",
                flexDirection: "column",
                gap: "4px",
                width: "100%"
            }}
            hx-swap-oob="true"
        >
            <Label
                required={false}
                label={I18n.translate("remote_terms")}
                subLabel={Maybe.none()}
            />
            <select
                style={{
                    border: `2px solid ${borderColor(error)}`,
                    borderRadius: "4px",
                    fontFamily: "inherit",
                    fontSize: "inherit",
                    padding: "12px",
                    fontWeight: "bold",
                    backgroundColor: WHITE
                }}
                name="remoteVariant"
                hx-post={validationUrl}
                hx-swap="none"
            >
                <option
                    value=""
                    selected={props.internals.value.variant === ""}
                >
                    { I18n.translate("undefined") }
                </option>
                <option
                    value="none"
                    selected={props.internals.value.variant === "none"}
                >
                    { I18n.translate("no_remote") }
                </option>
                <option
                    value="full"
                    selected={props.internals.value.variant === "full"}
                >
                    { I18n.translate("full_remote") }
                </option>
                <option
                    value="partial"
                    selected={props.internals.value.variant === "partial"}
                >
                    { I18n.translate("partial_remote") }
                </option>
            </select>
            { props.internals.value.variant === "partial" &&
                <div style={{
                    display: "flex",
                    alignItems: "baseline", 
                    gap: "8px"
                }}>
                    <input
                        style={{
                            width: "32px",
                            border: `2px solid ${borderColor(error)}`,
                            borderRadius: "4px",
                            fontFamily: "inherit",
                            fontSize: "inherit",
                            padding: "12px",
                            fontWeight: "bold"
                        }}
                        name="remoteDayCount"
                        input-mode="numeric"
                        value={props.internals.value.dayCount}
                        aria-label={I18n.translate("remote_day_count")}
                        placeholder="3"
                        hx-post={validationUrl}
                        hx-swap="none"
                    />
                    <span>{ I18n.translate("days_per") }</span>
                    <select
                        style={{
                            border: `2px solid ${borderColor(error)}`,
                            borderRadius: "4px",
                            fontFamily: "inherit",
                            fontSize: "inherit",
                            padding: "12px",
                            fontWeight: "bold",
                            backgroundColor: WHITE
                        }}
                        name="remoteBase"
                        aria-label={I18n.translate("remote_base")}
                        hx-post={validationUrl}
                        hx-swap="none"
                    >
                        <option
                            value="week"
                            selected={props.internals.value.base === "week"}
                        >
                            { I18n.translate("week") }
                        </option>
                        <option
                            value="month"
                            selected={props.internals.value.base === "month"}
                        >
                            { I18n.translate("month") }
                        </option>
                    </select>
                    <span>{ I18n.translate("in") }</span>
                    <select
                        style={{
                            border: `2px solid ${borderColor(error)}`,
                            borderRadius: "4px",
                            fontFamily: "inherit",
                            fontSize: "inherit",
                            padding: "12px",
                            fontWeight: "bold",
                            backgroundColor: WHITE
                        }}
                        name="remoteLocation"
                        aria-label={I18n.translate("remote_location")}
                        hx-post={validationUrl}
                        hx-swap="none"
                    >
                        <option
                            value="remote"
                            selected={props.internals.value.location === "remote"}
                        >
                            { I18n.translate("remote") }
                        </option>
                        <option
                            value="office"
                            selected={props.internals.value.location === "office"}
                        >
                            { I18n.translate("office") }
                        </option>
                    </select>
                </div>
            }
            { Maybe.isSome(error) &&
                <span style={{
                    color: RED
                }}>
                    { Maybe.unwrap(error) }
                </span>
            }
        </label>
    );
}

function buildError(parsed: Parsed<Maybe<Remote>, RemoteError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.none(); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "not_a_remote_variant": return Maybe.some(I18n.translate("remote_variant_is_not_in_the_provided_choices"));
            case "remote_day_count_is_negative": return Maybe.some(I18n.translate("remote_day_count_should_be_positive"));
            case "remote_day_count_is_not_a_number": return Maybe.some(I18n.translate("remote_day_count_should_be_a_number"));
            case "remote_day_count_is_not_an_integer": return Maybe.some(I18n.translate("remote_day_count_should_be_an_integer"));
            case "remote_base_is_not_a_week_or_a_month": return Maybe.some(I18n.translate("remote_base_should_be_a_week_or_a_month"));
            case "remote_day_count_should_be_between_1_and_7": return Maybe.some(I18n.translate("remote_day_count_should_be_between_1_and_7"));
            case "remote_day_count_should_be_between_1_and_31": return Maybe.some(I18n.translate("remote_day_count_should_be_between_1_and_31"));
            case "remote_location_is_not_remote_or_office": return Maybe.some(I18n.translate("remote_location_should_be_remote_or_office"));
            default: throw new UnreachableCaseError(err);
        } }
    });
}

function borderColor(error: Maybe<string>): string {
    return Maybe.match(error, {
        onSome: () => RED,
        onNone: () => BLACK
    });
}
