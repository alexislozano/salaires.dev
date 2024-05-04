import { Result } from "@utils";
import { Company } from "@domain";

export type CompanyRepository = {
    fetchAll: () => Promise<Result<Company[], string>>;
};

export { InMemoryCompanyRepository } from "./inMemory.ts";
export { SupabaseCompanyRepository } from "./supabase.ts";