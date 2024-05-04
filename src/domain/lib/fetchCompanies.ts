import { Result } from "@utils";
import { CompanyRepository } from "@infra";
import { Company } from "../models/mod.ts";

export async function fetchCompanies(companyRepository: CompanyRepository): Promise<Result<Company[], string>> {
    return Result.match(await companyRepository.fetchAll(), {
        onOk: (companies) => Result.ok(companies),
        onErr: (err) => Result.err(err)
    });
}