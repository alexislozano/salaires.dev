import { InMemoryRepository } from "../utils/mod.ts";
import { CompanyRepository } from "./mod.ts";
import { Company } from "@domain";
import { Result } from "@utils";

export class InMemoryCompanyRepository implements CompanyRepository {
    private repo: InMemoryRepository<Company>;

    private constructor(companies: Company[], error: boolean) {
        this.repo = new InMemoryRepository(companies, error);
    }

    static new() {
        return new InMemoryCompanyRepository([], false);
    }

    static withError() {
        return new InMemoryCompanyRepository([], true);
    }

    insert(company: Company) {
        this.repo.insert(company);
    }

    fetchAll(): Promise<Result<Company[], string>> {
        return this.repo.fetchAll();
    }
}
