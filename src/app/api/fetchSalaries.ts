import { Context } from "hono";
import * as lib from "@domain";
import { SalaryRepository } from "@infra";
import { Maybe, Result } from "@utils";

export async function fetchSalaries(
    c: Context,
    salaryRepo: SalaryRepository
) {
    return Result.match(await lib.fetchSalaries({
        key: lib.Key.default(),
        direction: lib.Direction.default()
    }, salaryRepo), {
        onOk: (salaries) => c.json(salaries.map(salary => ({
            company: lib.Company.toString(salary.company),
            title: Maybe.toNullable(Maybe.map(salary.title, lib.Title.toString)),
            location: lib.Location.toString(salary.location),
            compensation: lib.Compensation.toNumber(salary.compensation),
            date: lib.SalaryDate.toDate(salary.date),
            level: Maybe.toNullable(Maybe.map(salary.level, lib.Level.toString)),
            company_xp: Maybe.toNullable(Maybe.map(salary.companyXp, lib.Xp.toNumber)),
            total_xp: Maybe.toNullable(Maybe.map(salary.totalXp, lib.Xp.toNumber)),
        }))),
        onErr: () => c.json([], 500)
    });
}
