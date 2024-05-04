import { Result } from "@utils";
import { Captcha } from "@domain";

export type CaptchaService = {
    validate: (captcha: Captcha) => Promise<Result<void, string>>;
};

export { StubCaptchaService } from "./stub.ts";
export { HCaptchaService } from "./hcaptcha.ts";