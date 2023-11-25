import { Id, Token } from "@domain";
import { Result } from "@utils";

export type TokenRepository = {
    delete: (token: Token) => Result<Id, string>;
    insert: (salaryId: Id, token: Token) => Result<void, string>;
};
