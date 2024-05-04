import { CompanyRepository } from "./mod.ts";
import { Company } from "@domain";
import { Result } from "@utils";

export class InMemoryCompanyRepository implements CompanyRepository {
    private companies: Company[];
    private error: boolean;

    private constructor(companies: Company[], error: boolean) {
        this.companies = companies;
        this.error = error;
    }

    static new() {
        return new InMemoryCompanyRepository([], false);
    }

    static withError() {
        return new InMemoryCompanyRepository([], true);
    }

    insert(company: Company) {
        this.companies.push(company);
    }

    fetchAll(): Promise<Result<Company[], string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        return Promise.resolve(Result.ok(this.companies));
    }
}
