import { Context } from "hono";
import { CaptchaService, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository, TokenRepository, TokenSender } from "@infra";
import { Env, Maybe, Result } from "@utils";
import { Company, Location, Title, fetchCompanies, fetchLocations, fetchTitles, insertSalary } from "@domain";
import { Insert, TextOnly } from "../pages/mod.ts";
import { I18n } from "../i18n.ts";
import { ParsedForm, ValidatedForm, formSchema } from "../models/mod.ts";
import { parseForm } from "./utils/mod.ts";

export async function get(
    c: Context,
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository
) {
    return c.html(Result.match(await fetch(companyRepo, locationRepo, titleRepo), {
        onOk: (data) => Insert({
            form: ParsedForm.init(),
            notification: Maybe.none(),
            ...data
        }),
        onErr: () => TextOnly({ text: I18n.translate("form_fetching_error") })
    }));
}

export async function post(
    c: Context,
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository,
    salaryRepo: SalaryRepository,
    tokenRepo: TokenRepository,
    captchaService: CaptchaService,
    tokenSender: TokenSender
) {
    const dataResult = await fetch(companyRepo, locationRepo, titleRepo);
    if (Result.isErr(dataResult)) {
        console.log(Result.unwrapErr(dataResult));
        return c.html(TextOnly({
            text: I18n.translate("salary_inserting_error")
        }));
    }
    const data = Result.unwrap(dataResult);

    const unparsedForm = await parseForm(c, formSchema);
    const parsedForm = ParsedForm.fromUnparsedForm(unparsedForm);
    const validatedFormResult = ValidatedForm.tryFromParsedForm(parsedForm);
    if (Result.isErr(validatedFormResult)) {
        console.log(Result.unwrapErr(validatedFormResult));
        return c.html(Insert({
            form: parsedForm,
            notification: Maybe.some(I18n.translate("salary_inserting_error")),
            ...data
        }
    )); }
    const validatedForm = Result.unwrap(validatedFormResult);
    
    const salary = ValidatedForm.toSalary(validatedForm);
    const captcha = ValidatedForm.toCaptcha(validatedForm);

    return c.html(Result.match(await insertSalary(captchaService, salaryRepo, tokenRepo, tokenSender, captcha, salary), {
        onOk: () => Insert({
            form: ParsedForm.init(),
            notification: Maybe.some(I18n.translate("salary_inserted")),
            ...data
        }),
        onErr: (error) => {
            console.log(error);
            return Insert({
                form: parsedForm,
                notification: Maybe.some(I18n.translate("salary_inserting_error")),
                ...data
            });
        }
    }));
}

async function fetch(
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository
): Promise<Result<{
    hCaptchaKey: string;
    companies: Company[];
    locations: Location[];
    titles: Title[];
}, void>> {
    const hCaptchaKey = Env.get("HCAPTCHA_KEY");

    const [companiesResult, locationsResult, titlesResult] = await Promise.all([
        fetchCompanies(companyRepo),
        fetchLocations(locationRepo),
        fetchTitles(titleRepo)
    ]);
    if (Result.isErr(companiesResult)) { return Result.err(undefined) }
    if (Result.isErr(locationsResult)) { return Result.err(undefined); }
    if (Result.isErr(titlesResult)) { return Result.err(undefined); }

    return Result.ok({
        hCaptchaKey,
        companies: Result.unwrap(companiesResult),
        locations: Result.unwrap(locationsResult),
        titles: Result.unwrap(titlesResult)
    });
}