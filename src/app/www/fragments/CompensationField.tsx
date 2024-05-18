import { Compensation, CompensationError } from "@domain";
import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Internals, Parsed } from "../models/mod.ts";
import { Input } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<string, Compensation, CompensationError>
};

export function CompensationField(props: Props) {
    return (
        <Input
            error={error(props.internals.parsed)}
            name="compensation"
            inputMode="numeric"
            label={I18n.translate("compensation")}
            subLabel={Maybe.some(I18n.translate("compensation_help"))}
            placeholder="40000"
            required={true}
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function error(parsed: Parsed<Compensation, CompensationError>): Maybe<string> {
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
