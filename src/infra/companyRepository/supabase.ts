import { Env, Result } from "@utils";
import { CompanyRepository } from "./mod.ts";
import { Company, CompanyError } from "@domain";
import { z } from "zod";

export class SupabaseCompanyRepository implements CompanyRepository {
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
        return new SupabaseCompanyRepository(
            Env.get("SUPABASE_URL"),
            Env.get("SUPABASE_KEY")
        );
    }

    async fetchAll(): Promise<Result<Company[], string>> {
        const response = await fetch(`${this.url}companies?select=*&order=company`, {
            method: "GET",
            headers: this.headers()
        });
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