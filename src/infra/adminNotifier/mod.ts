import { Result } from "@utils";

export type AdminNotifier = {
    notify: () => Promise<Result<void, string>>;
};

export { StubAdminNotifier } from "./stub.ts";
export { EmailAdminNotifier } from "./email.ts";