import { Env, Result } from "@utils";
import { Captcha } from "@domain";
import { CaptchaService } from "./mod.ts";
import { z } from "zod";

export class HCaptchaService implements CaptchaService {
    private secret: string;

    private constructor(secret: string) {
        this.secret = secret;
    }

    private form(captcha: Captcha) {
        const form = new FormData();
        form.append("response", Captcha.toString(captcha));
        form.append("secret", this.secret);
        return form;
    }

    static new() {
        return new HCaptchaService(
            Env.get("HCAPTCHA_SECRET")
        );
    }

    async validate(captcha: Captcha): Promise<Result<void, string>> {
        const response = await fetch("https://hcaptcha.com/siteverify", {
            method: "POST",
            body: this.form(captcha)
        });
        if (! response.ok) { return Result.err("HCaptchaService: could not send request"); }
        
        const hCaptchaResponse = hCaptchaResponseSchema
            .safeParse(await response.json());
        if (! hCaptchaResponse.success) { return Result.err("HCaptchaService: could not parse json"); }

        if (! hCaptchaResponse.data.success) { return Result.err("HCaptchaService: could not convert to domain"); }
        return Result.ok(undefined);
    }
}

const hCaptchaResponseSchema = z.object({
    success: z.boolean()
});