import { CaptchaService } from "./mod.ts";
import { Captcha } from "@domain";
import { Result } from "@utils";

export class StubCaptchaService implements CaptchaService {
    private error: boolean;

    private constructor(error: boolean) {
        this.error = error;
    }

    static new() {
        return new StubCaptchaService(false);        
    }

    static withError() {
        return new StubCaptchaService(true);
    }

    validate(_: Captcha): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        } else {
            return Promise.resolve(Result.ok(undefined));
        }
    }
};
