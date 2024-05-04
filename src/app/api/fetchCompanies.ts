import { Context } from "hono";
import * as lib from "@domain";
import { CompanyRepository } from "@infra";
import { Result } from "@utils";

export async function fetchCompanies(
    c: Context,
    companyRepo: CompanyRepository
) {
    return Result.match(await lib.fetchCompanies(companyRepo), {
        onOk: (companies) => c.json(companies.map(company => ({
            company: lib.Company.toString(company)
        }))),
        onErr: () => c.json([], 500)
    });
}
