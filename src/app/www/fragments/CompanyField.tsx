import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Company, CompanyError } from "@domain";
import { Internals, Parsed } from "../models/mod.ts";
import { Select } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<Company, CompanyError>;
    companies: Company[];
};

export function CompanyField(props: Props) {
    return (
        <Select
            error={error(props.internals.parsed)}
            name="company"
            label={I18n.translate("company")}
            options={props.companies.map(Company.toString)}
            placeholder="Google"
            required
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function error(parsed: Parsed<Company, CompanyError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.none(); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "empty": return Maybe.some(I18n.translate("should_not_be_empty"));
            default: throw new UnreachableCaseError(err)
        } }
    });
}
