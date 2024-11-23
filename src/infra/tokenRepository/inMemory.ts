import { InMemoryRepository } from "../utils/mod.ts";
import { TokenRepository } from "./mod.ts";
import { Id, Token } from "@domain";
import { Result } from "@utils";

export class InMemoryTokenRepository implements TokenRepository {
    private repo: InMemoryRepository<{ id: Id, token: Token }>;

    private constructor(tokens: { id: Id, token: Token }[], error: boolean) {
        this.repo = new InMemoryRepository(tokens, error);
    }

    static new() {
        return new InMemoryTokenRepository([], false);
    }

    static withError() {
        return new InMemoryTokenRepository([], true);
    }

    async delete(token: Token): Promise<Result<Id, string>> {
        const deleteResult = await this.repo.delete({ filter: t => t.token.raw === token.raw });
        
        return Result.map(
            deleteResult,
            ({ id }) => id
        );
    }

    insert(id: Id, token: Token): Promise<Result<void, string>> {
        return this.repo.insert({ id, token });
    }
}
