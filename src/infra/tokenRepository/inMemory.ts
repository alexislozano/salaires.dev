import { TokenRepository } from "./index.ts";
import { Id, Token } from "@domain";
import { Result } from "@utils";

type State = {
    tokens: { id: Id, token: Token }[],
    error: boolean;
}

export const InMemoryTokenRepository = {
    new(): TokenRepository {
        const state = {
            tokens: [],
            error: false
        };

        return build(state);        
    }
};

function build(state: State): TokenRepository {
    return {
        delete: (token: Token) => {
            if (state.error) {
                return Result.err("error flag is on");
            }

            const index = state.tokens
                .findIndex(t => t.token.raw !== token.raw);

            if (! index) {
                return Result.err("token not found");
            }

            const [{ id }] = state.tokens.splice(index, 1);

            return Result.ok(id);
        },
        insert: (id: Id, token: Token) => {
            if (state.error) {
                return Result.err("error flag is on");
            }

            state.tokens.push({ id: id, token });

            return Result.ok(undefined);
        }
    };   
}
