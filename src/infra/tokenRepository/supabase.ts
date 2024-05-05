import { z } from "zod";
import { SupabaseRepository } from "../utils/mod.ts";
import { Result } from "@utils";
import { TokenRepository } from "./mod.ts";
import { Id, Token } from "@domain";

export class SupabaseTokenRepository implements TokenRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseTokenRepository(repo);
    }

    async delete(token: Token): Promise<Result<Id, string>> {
        const fetchResponse = await this.repo.fetch(`tokens?token=eq.${Token.toString(token)}`);
        if (! fetchResponse.ok) { return Result.err("could not send request"); }

        const supabaseTokens = z
            .array(supabaseTokenSchema)
            .safeParse(await fetchResponse.json());
        if (! supabaseTokens.success) { return Result.err("could not parse json"); }

        const supabaseToken = supabaseTokens.data.pop();
        if (! supabaseToken) { return Result.err("could not find token"); }

        const salaryId = Id.tryFromString(supabaseToken.salary_id);
        if (Result.isErr(salaryId)) { return Result.err("could not convert to domain"); }
        
        const deleteResponse = await this.repo.delete(`tokens?token=eq.${Token.toString(token)}`);
        if (! deleteResponse.ok) { return Result.err("could not send request"); }
        return salaryId;
    }

    async insert(salaryId: Id, token: Token): Promise<Result<void, string>> {
        const response = await this.repo.post("tokens", SupabaseToken.fromSalaryIdAndToken(salaryId, token));
        if (! response.ok) { return Result.err("could not send request"); }
        return Result.ok(undefined);
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
    }
}