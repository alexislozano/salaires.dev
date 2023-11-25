import { CaptchaService } from "./index.ts";
import { Captcha } from "@domain";
import { Result } from "@utils";

type State = {
    error: boolean;
}

export const StubCaptchaService = {
    new(): CaptchaService {
        return build({ error: false });        
    },
    withError(): CaptchaService {
        return build({ error: true });
    }
};

function build(state: State): CaptchaService {
    return {
        validate: (_: Captcha) => {
            if (state.error) {
                return Result.err("error flag is on");
            } else {
                return Result.ok(undefined);
            }
        }
    };   
}
