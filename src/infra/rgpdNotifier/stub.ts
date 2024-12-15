import { RgpdNotifier } from "./mod.ts";
import { Result } from "@utils";

export class StubRgpdNotifier implements RgpdNotifier {
    private error: boolean;

    private constructor(error: boolean) {
        this.error = error;
    }

    static new() {
        return new StubRgpdNotifier(false);        
    }

    static withError() {
        return new StubRgpdNotifier(true);
    }

    notify(): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        } else {
            return Promise.resolve(Result.ok(undefined));
        }
    }
}