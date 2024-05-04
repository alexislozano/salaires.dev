import { Email, EmailError } from "@domain";
import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Internals, Parsed } from "../models/mod.ts";
import { Input } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<Email, EmailError>
};

export function EmailField(props: Props) {
    return (
        <Input
            error={error(props.internals.parsed)}
            name="email"
            inputMode="email"
            label={I18n.translate("email")}
            subLabel={Maybe.none()}
            placeholder="moi@exemple.fr"
            required
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function error(parsed: Parsed<Email, EmailError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.some(""); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "not_an_email": return Maybe.some(I18n.translate("email_should_contain_an_at"));
            case "not_a_pro_email": return Maybe.some(I18n.translate("email_should_be_pro"));
            default: throw new UnreachableCaseError(err)
        } }
    });
}
