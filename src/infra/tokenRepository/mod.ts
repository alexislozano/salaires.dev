import { Id, Token } from "@domain";
import { Result } from "@utils";

export type TokenRepository = {
    delete: (token: Token) => Promise<Result<Id, string>>;
    insert: (salaryId: Id, token: Token) => Promise<Result<void, string>>;
};

export { InMemoryTokenRepository } from "./inMemory.ts";
export { SupabaseTokenRepository } from "./supabase.ts";