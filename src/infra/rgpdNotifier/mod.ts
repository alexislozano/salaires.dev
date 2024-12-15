import { Result } from "@utils";

export type RgpdNotifier = {
    notify: () => Promise<Result<void, string>>;
};

export { StubRgpdNotifier } from "./stub.ts";
export { EmailRgpdNotifier } from "./email.ts";