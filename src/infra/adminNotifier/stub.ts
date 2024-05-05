import { AdminNotifier } from "./mod.ts";
import { Result } from "@utils";

export class StubAdminNotifier implements AdminNotifier {
    private error: boolean;

    private constructor(error: boolean) {
        this.error = error;
    }

    static new() {
        return new StubAdminNotifier(false);        
    }

    static withError() {
        return new StubAdminNotifier(true);
    }

    notify(): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        } else {
            return Promise.resolve(Result.ok(undefined));
        }
    }
}