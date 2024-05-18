import { Context } from "hono";
import { CompanyRepository, LocationRepository, TitleRepository } from "@infra";
import { parseForm } from "./utils/mod.ts";
import { ParsedForm, ValidatedForm, formSchema } from "../models/mod.ts";
import { Maybe, Result } from "@utils";
import { Notification, Submit } from "../components/mod.ts";
import { I18n } from "../i18n.ts";
import { CompanyField, CompanyXpField, CompensationField, EmailField, LevelField, LocationField, RemoteField, TitleField, TotalXpField } from "../fragments/mod.ts";
import { fetchCompanies, fetchLocations, fetchTitles } from "@domain";

export async function post(
    c: Context,
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository
) {
    const name = c.req.header("HX-Trigger-Name");

    const unparsedForm = await parseForm(c, formSchema);
    const parsedForm = ParsedForm.fromUnparsedForm(unparsedForm);
    const disabled = Result.isErr(ValidatedForm.tryFromParsedForm(parsedForm));

    const field = await generateField(
        name,
        parsedForm,
        companyRepo,
        locationRepo,
        titleRepo
    );

    return c.html([field, Submit({
        disabled,
        label: I18n.translate("send")
    })].join(""));
}

async function generateField(
    name: string | undefined,
    parsedForm: ParsedForm,
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository
) {
    switch (name) {
        case "email": return EmailField({ internals: parsedForm.email });
        case "company": return Result.match(await fetchCompanies(companyRepo), {
            onOk: (companies) => CompanyField({ internals: parsedForm.company, companies }),
            onErr: () => buildErrorNotification()
        });
        case "title": return Result.match(await fetchTitles(titleRepo), {
            onOk: (titles) => TitleField({ internals: parsedForm.title, titles }),
            onErr: () => buildErrorNotification()
        });
        case "level": return LevelField({ internals: parsedForm.level });
        case "location": return Result.match(await fetchLocations(locationRepo), {
            onOk: (locations) => LocationField({ internals: parsedForm.location, locations }),
            onErr: () => buildErrorNotification()
        });
        case "compensation": return CompensationField({ internals: parsedForm.compensation });
        case "companyXp": return CompanyXpField({ internals: parsedForm.companyXp });
        case "totalXp": return TotalXpField({ internals: parsedForm.totalXp });
        case "remoteVariant":
        case "remoteDayCount":
        case "remoteBase":
        case "remoteLocation": return RemoteField({ internals: parsedForm.remote });
        default: return "";
    }
}

function buildErrorNotification() {
    return Notification({
        notification: Maybe.some(I18n.translate("validation_error"))
    });
}