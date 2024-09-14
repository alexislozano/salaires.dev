import { assert, assertEquals } from "assert";
import { InMemoryCompanyRepository } from "@infra";
import { fetchCompanies } from "./fetchCompanies.ts";
import { Result } from "@utils";
import { Company } from "@domain";

Deno.test("it should return an error when an error occurs", async () => {
    const companyRepo = InMemoryCompanyRepository.withError();

    const result = await fetchCompanies(companyRepo);

    assert(Result.isErr(result));
});

Deno.test("it should return all companies otherwise", async () => {
    const company = Company.generate();
    const companyRepo = InMemoryCompanyRepository.new();
    companyRepo.insert(company);

    const result = await fetchCompanies(companyRepo);

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [company]);
});
