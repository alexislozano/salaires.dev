import { Result } from "@utils";

export type Captcha = {
    _type: "captcha";
    raw: string;
};

export const Captcha = {
    toString(captcha: Captcha): string {
        return captcha.raw;
    },
    tryFromString(raw: string): Result<Captcha, "empty"> {
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "captcha", raw });
    }
};
