import { z } from "zod";
import { SupabaseRepository } from "../utils/mod.ts";
import { Result } from "@utils";
import { TokenRepository } from "./mod.ts";
import { Id, IdError, Token } from "@domain";

const SERVICE = "SupabaseTokenRepository";

export class SupabaseTokenRepository implements TokenRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseTokenRepository(repo);
    }

    async delete(token: Token): Promise<Result<Id, string>> {
        const salaryId = await this.repo.fetchOne({
            url: `tokens?token=eq.${Token.toString(token)}`,
            schema: supabaseTokenSchema,
            convert: SupabaseToken.tryToId,
            service: SERVICE
        })
        if (Result.isErr(salaryId)) { return salaryId; }
        
        const deleteResult = await this.repo.delete({
            url: `tokens?token=eq.${Token.toString(token)}`,
            service: SERVICE
        });
        if (Result.isErr(deleteResult)) { return deleteResult; }

        return salaryId;
    }

    insert(salaryId: Id, token: Token): Promise<Result<void, string>> {
        return this.repo.insert({
            url: "tokens",
            body: SupabaseToken.fromSalaryIdAndToken(salaryId, token),
            service: SERVICE
        });
    }
}

const supabaseTokenSchema = z.object({
    salary_id: z.string(),
    token: z.string(),
});
type SupabaseToken = z.infer<typeof supabaseTokenSchema>;

const SupabaseToken = {
    fromSalaryIdAndToken(salaryId: Id, token: Token): SupabaseToken {
        return {
            salary_id: Id.toString(salaryId),
            token: Token.toString(token),
        };
    },
    tryToId(token: SupabaseToken): Result<Id, IdError> {
        return Id.tryFromString(token.salary_id)
    }
}