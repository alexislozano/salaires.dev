import { Xp, XpError } from "@domain";
import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Internals, Parsed } from "../models/mod.ts";
import { Input } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<string, Maybe<Xp>, XpError>
};

export function TotalXpField(props: Props) {
    return (
        <Input
            error={error(props.internals.parsed)}
            name="totalXp"
            inputMode="numeric"
            label={I18n.translate("total_xp")}
            subLabel={Maybe.some(I18n.translate("in_years"))}
            placeholder="10"
            required={false}
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function error(parsed: Parsed<Maybe<Xp>, XpError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.none(); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "negative": return Maybe.some(I18n.translate("should_be_positive"));
            case "not_a_number": return Maybe.some(I18n.translate("should_be_a_number"));
            case "not_an_integer": return Maybe.some(I18n.translate("should_be_an_integer"));
            default: throw new UnreachableCaseError(err)
        } }
    });
}
