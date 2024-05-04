import { Result } from "@utils";

export type Token = {
    _type: "token";
    raw: string;
};

const LENGTH = 64;

export const Token = {
    generate(): Token {
        const raw = Array(LENGTH)
            .fill(0)
            .map(() => Math.floor(Math.random() * 10))
            .join("");
        return { _type: "token", raw };
    },
    toString(token: Token): string {
        return token.raw;
    },
    tryFromString(raw: string): Result<Token, "not_a_token"> {
        const notAToken = raw.length !== LENGTH
            && Number.isNaN(Number(raw));
        if (notAToken) { return Result.err("not_a_token"); }
        return Result.ok({ _type: "token", raw });
    }
}
