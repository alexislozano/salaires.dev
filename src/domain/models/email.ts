import { Result } from "@utils";

export type Email = {
    _type: "email";
    raw: string;
};

export type EmailError =
    | "not_an_email"
    | "not_a_pro_email"
    ;

const FORBIDDEN_DOMAINS = [
    "gmail",
    "yahoo",
    "hotmail",
    "aol",
    "wanadoo",
    "msn",
    "live",
    "free",
    "outlook",
    "laposte",
    "protonmail",
    "yopmail",
    "minutemail",
];

export const Email = {
    generate(): Email {
        return {
            _type: "email",
            raw: "bonjour@salaires.dev"
        };
    },
    toString(email: Email): string {
        return email.raw;
    },
    tryFromString(email: string, { admin = false } = {}): Result<Email, EmailError> {
        const raw = email.trim();
        if (! raw.includes("@")) { return Result.err("not_an_email"); }
        if (admin) { return Result.ok({ _type: "email", raw }); }
        if (FORBIDDEN_DOMAINS.some(domain => raw.includes(domain))) { return Result.err("not_a_pro_email"); }
        return Result.ok({ _type: "email", raw });
    }
};
