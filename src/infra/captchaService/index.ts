import { Result } from "@utils";
import { Captcha } from "@domain";

export type CaptchaService = {
    validate: (captcha: Captcha) => Result<void, string>;
};
