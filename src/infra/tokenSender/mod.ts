import { Email, Token } from "@domain";
import { Result } from "@utils";

export type TokenSender = {
    send: (token: Token, email: Email) => Promise<Result<void, string>>;
};

export { StubTokenSender } from "./stub.ts";
export { EmailTokenSender } from "./email.ts";