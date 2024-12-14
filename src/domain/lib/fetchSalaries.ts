import { Result } from "@utils";
import { SalaryRepository } from "@infra";
import { Key, Order, PublishedSalary } from "../models/mod.ts";

export async function fetchSalaries(
    order: Order<Key>,
    salaryRepository: SalaryRepository
): Promise<Result<PublishedSalary[], string>> {
    return Result.match(await salaryRepository.fetchAll(order), {
        onOk: (salaries) => Result.ok(salaries),
        onErr: (err) => Result.err(err)
    });
}