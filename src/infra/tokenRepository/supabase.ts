import { z } from "zod";
import { Env, Result } from "@utils";
import { TokenRepository } from "./mod.ts";
import { Id, Token } from "@domain";

export class SupabaseTokenRepository implements TokenRepository {
    private url: string;
    private key: string;

    private constructor(url: string, key: string) {
        this.url = url;
        this.key = key;
    }

    private headers(): Record<string, string> {
        return {
            apiKey: this.key,
            Authorization: `Bearer ${this.key}`,
            "Content-Type": "application/json",
        };
    }

    static new() {
        return new SupabaseTokenRepository(
            Env.get("SUPABASE_URL"),
            Env.get("SUPABASE_KEY")
        );
    }

    async delete(token: Token): Promise<Result<Id, string>> {
        const fetchResponse = await fetch(`${this.url}tokens?token=eq.${Token.toString(token)}`, {
            method: "GET",
            headers: this.headers()
        });
        if (! fetchResponse.ok) { return Result.err("could not send request"); }

        const supabaseTokens = z
            .array(supabaseTokenSchema)
            .safeParse(await fetchResponse.json());
        if (! supabaseTokens.success) { return Result.err("could not parse json"); }

        const supabaseToken = supabaseTokens.data.pop();
        if (! supabaseToken) { return Result.err("could not find token"); }

        const salaryId = Id.tryFromString(supabaseToken.salary_id);
        if (Result.isErr(salaryId)) { return Result.err("could not convert to domain"); }
        
        const deleteResponse = await fetch(`${this.url}tokens?token=eq.${Token.toString(token)}`, {
            method: "DELETE",
            headers: this.headers()
        });
        if (! deleteResponse.ok) { return Result.err("could not send request"); }
        return salaryId;
    }

    async insert(salaryId: Id, token: Token): Promise<Result<void, string>> {
        const response = await fetch(`${this.url}tokens`, {
            method: "POST",
            headers: this.headers(),
            body: JSON.stringify(SupabaseToken.fromSalaryIdAndToken(salaryId, token))
        });
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