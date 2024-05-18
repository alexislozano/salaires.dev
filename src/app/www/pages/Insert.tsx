import { Company, Location, Title } from "@domain";
import { ParsedForm } from "../models/mod.ts";
import { Banner, Form, HCaptcha, Submit } from "../components/mod.ts";
import { CompanyField, CompanyXpField, CompensationField, EmailField, LevelField, LocationField, RemoteField, TitleField, TotalXpField } from "../fragments/mod.ts";
import { Template } from "./Template.tsx";
import { Maybe } from "@utils";
import { I18n } from "../i18n.ts";

type Props = {
    form: ParsedForm;
    hCaptchaKey: string;
    companies: Company[];
    locations: Location[];
    titles: Title[];
    notification: Maybe<string>;
};

export function Insert(props: Props) {
    return (
        <Template notification={props.notification}>
            <Form
                title={I18n.translate("i_add_my_salary")}
                postUrl="/insert"
            >
                <Banner text={I18n.translate("email_explanation")} />
                <EmailField internals={props.form.email} />
                <CompanyField internals={props.form.company} companies={props.companies} />
                <TitleField internals={props.form.title} titles={props.titles} />
                <LevelField internals={props.form.level} />
                <LocationField internals={props.form.location} locations={props.locations} />
                <CompensationField internals={props.form.compensation} />
                <CompanyXpField internals={props.form.companyXp} />
                <TotalXpField internals={props.form.totalXp} />
                <RemoteField internals={props.form.remote} />
                <HCaptcha hCaptchaKey={props.hCaptchaKey} validationUrl="/validate" />
                <Submit disabled label={I18n.translate("send")} />
            </Form>
        </Template>
    )
}