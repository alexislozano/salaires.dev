import { assert, assertEquals } from "assert";
import { InMemorySalaryRepository } from "@infra";
import { Result } from "@utils";
import { Direction, Key, Salary } from "@domain";
import { fetchSalaries } from "./fetchSalaries.ts";

const order = { key: Key.default(), direction: Direction.default() };

Deno.test("it should return an error when an error occurs", async () => {
    const salaryRepo = InMemorySalaryRepository.withError();

    const result = await fetchSalaries(order, salaryRepo);

    assert(Result.isErr(result));
});

Deno.test("it should return all salaries otherwise", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const salary = Salary.generate();
    salaryRepo.insert(salary);

    const result = await fetchSalaries(order, salaryRepo);

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [salary]);
});

Deno.test("it should return ordered salaries", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const salary1: Salary = {
        ...Salary.generate(),
        company: { _type: "company", raw: "A" }
    };
    const salary2: Salary = {
        ...Salary.generate(),
        company: { _type: "company", raw: "B" }
    };
    salaryRepo.insert(salary1);
    salaryRepo.insert(salary2);

    const result = await fetchSalaries(
        { key: "company", direction: "desc" },
        salaryRepo
    );

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [salary2, salary1]);
});
