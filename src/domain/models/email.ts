import { Result } from "@utils";

export type Email = {
    _type: "email";
    raw: string;
};

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
    toString(email: Email): string {
        return email.raw;
    },
    tryFromString(raw: string): Result<Email, "not_an_email" | "not_a_pro_email"> {
        if (! raw.includes("@")) { return Result.err("not_an_email"); }
        if (FORBIDDEN_DOMAINS.some(domain => raw.includes(domain))) { return Result.err("not_a_pro_email"); }
        return Result.ok({ _type: "email", raw });
    }
};
