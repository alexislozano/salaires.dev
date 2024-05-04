import { Result } from "@utils";

export type Captcha = {
    _type: "captcha";
    raw: string;
};

export type CaptchaError = "empty";

export const Captcha = {
    generate(): Captcha {
        return {
            _type: "captcha",
            raw: "123456"
        };
    },
    toString(captcha: Captcha): string {
        return captcha.raw;
    },
    tryFromString(captcha: string): Result<Captcha, CaptchaError> {
        const raw = captcha.trim();
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "captcha", raw });
    }
};
