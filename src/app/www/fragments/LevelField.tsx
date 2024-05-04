import { Maybe, Result, UnreachableCaseError } from "@utils";
import { Level, LevelError } from "@domain";
import { Internals, Parsed } from "../models/mod.ts";
import { Dropdown } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    internals: Internals<Maybe<Level>, LevelError>;
};

export function LevelField(props: Props) {
    const choices = Level.all().map(level => ({
        key: Level.toString(level),
        label: buildLabel(level)
    }));
    return (
        <Dropdown
            error={error(props.internals.parsed)}
            name="level"
            label={I18n.translate("level")}
            subLabel={Maybe.none()}
            choices={choices}
            required={false}
            validationUrl="/validate"
            value={props.internals.value}
        />
    );
}

function buildLabel(level: Level): string {
    switch (level) {
        case "junior": return I18n.translate("junior");
        case "mid": return I18n.translate("mid");
        case "senior": return I18n.translate("senior");
        default: throw new UnreachableCaseError(level)
    }
}

function error(parsed: Parsed<Maybe<Level>, LevelError>): Maybe<string> {
    if (parsed._type === "init") { return Maybe.none(); }
    return Result.match(parsed.result, {
        onOk: () => Maybe.none(),
        onErr: (err) => { switch (err) {
            case "not_a_level": return Maybe.some(I18n.translate("level_is_not_in_the_provided_choices"));
            default: throw new UnreachableCaseError(err)
        } }
    });
}
