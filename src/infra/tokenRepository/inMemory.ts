import { TokenRepository } from "./mod.ts";
import { Id, Token } from "@domain";
import { Result } from "@utils";

export class InMemoryTokenRepository implements TokenRepository {
    private tokens: { id: Id, token: Token }[];
    private error: boolean;

    private constructor(tokens: { id: Id, token: Token }[], error: boolean) {
        this.tokens = tokens;
        this.error = error;
    }

    static new() {
        return new InMemoryTokenRepository([], false);
    }

    static withError() {
        return new InMemoryTokenRepository([], true);
    }

    delete(token: Token): Promise<Result<Id, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        const index = this.tokens
            .findIndex(t => t.token.raw === token.raw);

        if (index == -1) {
            return Promise.resolve(Result.err("token not found"));
        }

        const [{ id }] = this.tokens.splice(index, 1);

        return Promise.resolve(Result.ok(id));
    }

    insert(id: Id, token: Token): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        this.tokens.push({ id, token });

        return Promise.resolve(Result.ok(undefined));
    }
}
