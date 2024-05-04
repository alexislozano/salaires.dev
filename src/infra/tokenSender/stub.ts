import { Email, Token } from "@domain";
import { TokenSender } from "./mod.ts";
import { Result } from "@utils";

export class StubTokenSender implements TokenSender {
    private error: boolean;

    private constructor(error: boolean) {
        this.error = error;
    }

    static new() {
        return new StubTokenSender(false);        
    }

    static withError() {
        return new StubTokenSender(true);
    }

    send(_token: Token, _email: Email): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        } else {
            return Promise.resolve(Result.ok(undefined));
        }
    }
}