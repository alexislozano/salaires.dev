import { Email, Token } from "@domain";
import { Result } from "@utils";

export type TokenSender = {
    send: (token: Token, email: Email) => Result<void, string>;
};
