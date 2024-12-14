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

Deno.test("it should return published salaries otherwise", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const salary1 = Salary.generate();
    const salary2 = Salary.generate();
    const salary3 = Salary.generate();
    salaryRepo.insert(salary1);
    salaryRepo.insert(salary2);
    salaryRepo.confirm(salary2.id)
    salaryRepo.insert(salary3);
    salaryRepo.confirm(salary3.id);
    salaryRepo.publish(salary3.id);

    const result = await fetchSalaries(order, salaryRepo);

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [Salary.publish(Salary.confirm(salary3))]);
});

Deno.test("it should return ordered salaries", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const salary1 = {
        ...Salary.generate(),
        company: { _type: "company" as const, raw: "A" }
    };
    const salary2 = {
        ...Salary.generate(),
        company: { _type: "company" as const, raw: "B" }
    };
    salaryRepo.insert(salary1);
    salaryRepo.insert(salary2);
    salaryRepo.confirm(salary1.id);
    salaryRepo.publish(salary1.id);
    salaryRepo.confirm(salary2.id);
    salaryRepo.publish(salary2.id);

    const result = await fetchSalaries(
        { key: "company", direction: "desc" },
        salaryRepo
    );

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [
        Salary.publish(Salary.confirm(salary2)),
        Salary.publish(Salary.confirm(salary1))
    ]);
});
