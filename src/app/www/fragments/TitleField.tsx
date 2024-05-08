import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Title, TitleError } from "@domain";
import { Internals, Parsed } from "../models/mod.ts";
import { Select } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<Maybe<Title>, TitleError>;
    titles: Title[];
};

export function TitleField(props: Props) {
    return (
        <Select
            error={error(props.internals.parsed)}
            name="title"
            label={I18n.translate("title")}
            options={props.titles.map(Title.toString)}
            placeholder="Software Engineer"
            required
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function error(parsed: Parsed<Maybe<Title>, TitleError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.none(); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "empty": return Maybe.some(I18n.translate("should_not_be_empty"));
            default: throw new UnreachableCaseError(err)
        } }
    });
}