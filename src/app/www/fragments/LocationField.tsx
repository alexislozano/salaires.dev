import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Location, LocationError } from "@domain";
import { Internals, Parsed } from "../models/mod.ts";
import { Select } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<Location, LocationError>;
    locations: Location[];
};

export function LocationField(props: Props) {
    return (
        <Select
            error={error(props.internals.parsed)}
            name="location"
            label={I18n.translate("location")}
            options={props.locations.map(Location.toString)}
            placeholder="Paris"
            required
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function error(parsed: Parsed<Location, LocationError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.none(); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "empty": return Maybe.some(I18n.translate("should_not_be_empty"));
            default: throw new UnreachableCaseError(err)
        } }
    });
}
