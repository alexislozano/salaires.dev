import { Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { CompanyRepository } from "./mod.ts";
import { Company, CompanyError } from "@domain";
import { z } from "zod";

export class SupabaseCompanyRepository implements CompanyRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseCompanyRepository(repo);
    }

    async fetchAll(): Promise<Result<Company[], string>> {
        const response = await this.repo.fetch("companies?select=*&order=company");
        if (! response.ok) { return Result.err("could not send request"); }
        
        const supabaseCompanies = z
            .array(supabaseCompanySchema)
            .safeParse(await response.json());
        if (! supabaseCompanies.success) { return Result.err("could not parse json"); }

        const companies: Company[] = [];
        for (const supabaseCompany of supabaseCompanies.data) {
            const company = SupabaseCompany.tryToCompany(supabaseCompany);
            if (Result.isErr(company)) { return Result.err("could not convert to domain"); }
            companies.push(Result.unwrap(company));
        }

        return Result.ok(companies);
    }
}

const supabaseCompanySchema = z.object({
    company: z.string()
});
type SupabaseCompany = z.infer<typeof supabaseCompanySchema>;

const SupabaseCompany = {
    tryToCompany(company: SupabaseCompany): Result<Company, CompanyError> {
        return Company.tryFromString(company.company);
    }
}