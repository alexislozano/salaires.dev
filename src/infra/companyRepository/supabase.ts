import { Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { CompanyRepository } from "./mod.ts";
import { Company, CompanyError } from "@domain";
import { z } from "zod";

const SERVICE = "SupabaseCompanyRepository";

export class SupabaseCompanyRepository implements CompanyRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseCompanyRepository(repo);
    }

    fetchAll(): Promise<Result<Company[], string>> {
        return this.repo.fetchAll({
            url: "companies?select=*&order=company",
            schema: supabaseCompanySchema,
            convert: SupabaseCompany.tryToCompany,
            service: SERVICE
        });
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