import { Result } from "@utils";
import { SalaryRepository } from "@infra";
import { Salary } from "../models/index.ts";

export function fetchSalaries(salaryRepository: SalaryRepository): Result<Salary[], string> {
    return Result.match(salaryRepository.fetchAll(), {
        onOk: (salaries) => Result.ok(salaries),
        onErr: (err) => Result.err(err)
    });
}