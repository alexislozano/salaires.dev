import { Result } from "@utils";
import { CompanyRepository } from "@infra";
import { Company } from "../models/index.ts";

export function fetchCompanies(companyRepository: CompanyRepository): Result<Company[], string> {
    return Result.match(companyRepository.fetchAll(), {
        onOk: (companies) => Result.ok(companies),
        onErr: (err) => Result.err(err)
    });
}