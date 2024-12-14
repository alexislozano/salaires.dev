import { z } from "zod";
import { Maybe, Result } from "@utils";
import { Captcha, Company, CompanyError, Compensation, CompensationError, Email, EmailError, Id, Level, LevelError, Location, LocationError, Remote, RemoteError, SalaryDate, Title, TitleError, WaitingSalary, Xp, XpError } from "@domain";

export type Parsed<T, E> =
    | { _type: "init" }
    | { _type: "computed", result: Result<T, E> }
    ;

export type Internals<V, T, E> = {
    value: V;
    parsed: Parsed<T, E>;
};

export const Internals = {
    init<V, T, E>(value: V): Internals<V, T, E> {
        return {
            value,
            parsed: { _type: "init" }
        };
    },
    computed<V, T, E>(value: V, result: Result<T, E>): Internals<V, T, E> {
        return {
            value,
            parsed: { _type: "computed", result }
        };
    },
    extract<V, T, E>(internals: Internals<V, T, E>): Result<T, ""> {
        if (internals.parsed._type === "init") { return Result.err(""); }
        if (Result.isErr(internals.parsed.result)) { return Result.err(""); }
        return internals.parsed.result;
    }
};

export const formSchema = z.object({
    email: z.string(),
    company: z.string(),
    title: z.string(),
    location: z.string(),
    compensation: z.string(),
    level: z.string(),
    companyXp: z.string(),
    totalXp: z.string(),
    remoteVariant: z.string(),
    remoteDayCount: z.optional(z.string()),
    remoteBase: z.optional(z.string()),
    remoteLocation: z.optional(z.string()),
    "h-captcha-response": z.string(),
});

export type ParsedForm = {
    _type: "parsed_form",
    email: Internals<string, Email, EmailError>,
    company: Internals<string, Company, CompanyError>,
    title: Internals<string, Maybe<Title>, TitleError>,
    level: Internals<string, Maybe<Level>, LevelError>,
    location: Internals<string, Location, LocationError>,
    compensation: Internals<string, Compensation, CompensationError>,
    companyXp: Internals<string, Maybe<Xp>, XpError>,
    totalXp: Internals<string, Maybe<Xp>, XpError>,
    remote: Internals<{ variant: string, dayCount: string, base: string, location: string }, Maybe<Remote>, RemoteError>,
    captcha: Maybe<Captcha>,
};

export const ParsedForm = {
    init(): ParsedForm {
        return {
            _type: "parsed_form",
            email: Internals.init(""),
            company: Internals.init(""),
            title: Internals.init(""),
            level: Internals.init(""),
            location: Internals.init(""),
            compensation: Internals.init(""),
            companyXp: Internals.init(""),
            totalXp: Internals.init(""),
            remote: Internals.init({ variant: "", dayCount: "", base: "", location: "" }),
            captcha: Maybe.none()
        };
    },
    fromUnparsedForm(form: z.infer<typeof formSchema>): ParsedForm {
        const remoteForm = {
            variant: form.remoteVariant,
            dayCount: form.remoteDayCount ?? "",
            base: form.remoteBase ?? "",
            location: form.remoteLocation ?? ""
        };
        return {
            _type: "parsed_form",
            email: Internals.computed(form.email, Email.tryFromString(form.email)),
            company: Internals.computed(form.company, Company.tryFromString(form.company)),
            title: Internals.computed(form.title, form.title.length === 0
                ? Result.ok(Maybe.none())
                : Result.map(Title.tryFromString(form.title), Maybe.some)
            ),
            level: Internals.computed(form.level, form.level.length === 0
                ? Result.ok(Maybe.none())
                : Result.map(Level.tryFromString(form.level), Maybe.some)
            ),
            location: Internals.computed(form.location, Location.tryFromString(form.location)),
            compensation: Internals.computed(form.compensation, Compensation.tryFromString(form.compensation)),
            companyXp: Internals.computed(form.companyXp, form.companyXp.length === 0
                ? Result.ok(Maybe.none())
                : Result.map(Xp.tryFromString(form.companyXp), Maybe.some)
            ),
            totalXp: Internals.computed(form.totalXp, form.totalXp.length === 0
                ? Result.ok(Maybe.none())
                : Result.map(Xp.tryFromString(form.totalXp), Maybe.some)
            ),
            remote: Internals.computed(remoteForm, form.remoteVariant.length === 0
                ? Result.ok(Maybe.none())
                : Result.map(Remote.tryFromForm(remoteForm), Maybe.some)
            ),
            captcha: Result.match(Captcha.tryFromString(form["h-captcha-response"]), {
                onOk: (captcha) => Maybe.some(captcha),
                onErr: () => Maybe.none()
            })
        }
    }
};

export type ValidatedForm = {
    _type: "validated_form",
    email: Email,
    company: Company,
    title: Maybe<Title>,
    level: Maybe<Level>,
    location: Location,
    compensation: Compensation,
    companyXp: Maybe<Xp>,
    totalXp: Maybe<Xp>,
    remote: Maybe<Remote>,
    captcha: Captcha,
};

export const ValidatedForm = {
    tryFromParsedForm(form: ParsedForm): Result<ValidatedForm, ""> {
        const email = Internals.extract(form.email);
        const company = Internals.extract(form.company);
        const title = Internals.extract(form.title);
        const level = Internals.extract(form.level);
        const location = Internals.extract(form.location);
        const compensation = Internals.extract(form.compensation);
        const companyXp = Internals.extract(form.companyXp);
        const totalXp = Internals.extract(form.totalXp);
        const remote = Internals.extract(form.remote);
        const captcha = form.captcha;

        if (
            Result.isErr(email)
            || Result.isErr(company)
            || Result.isErr(title)
            || Result.isErr(level)
            || Result.isErr(location)
            || Result.isErr(compensation)
            || Result.isErr(companyXp)
            || Result.isErr(totalXp)
            || Result.isErr(remote)
            || Maybe.isNone(captcha)
        ) { return Result.err(""); }

        return Result.ok({
            _type: "validated_form",
            email: Result.unwrap(email),
            company: Result.unwrap(company),
            title: Result.unwrap(title),
            level: Result.unwrap(level),
            location: Result.unwrap(location),
            compensation: Result.unwrap(compensation),
            companyXp: Result.unwrap(companyXp),
            totalXp: Result.unwrap(totalXp),
            remote: Result.unwrap(remote),
            captcha: Maybe.unwrap(captcha)
        });
    },
    toSalary(form: ValidatedForm): WaitingSalary {
        return {
            id: Id.generate(),
            email: form.email,
            company: form.company,
            title: form.title,
            location: form.location,
            compensation: form.compensation,
            date: SalaryDate.fromDate(new Date()),
            level: form.level,
            companyXp: form.companyXp,
            totalXp: form.totalXp,
            remote: form.remote,
            status: "waiting"
        };
    },
    toCaptcha(form: ValidatedForm): Captcha {
        return form.captcha;
    }
}
