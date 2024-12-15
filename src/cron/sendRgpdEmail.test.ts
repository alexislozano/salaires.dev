import { assert } from "assert";
import { InMemorySalaryRepository, StubRgpdNotifier } from "@infra";
import { Result } from "@utils";
import { sendRgpdEmail } from "./sendRgpdEmail.ts";
import { Salary, SalaryDate } from "@domain";

const TODAY = new Date("2024-01-15");
const SEVEN_DAYS_AGO = new Date("2024-01-08");
const EIGHT_DAYS_AGO = new Date("2024-01-07");
const DAYS_BEFORE_DELETION = 7;

Deno.test("it should return an error when the salaries could not be fetched", async () => {
    const salaryRepo = InMemorySalaryRepository.withError();
    const rgpdNotifier = StubRgpdNotifier.new();

    const result = await sendRgpdEmail(salaryRepo, rgpdNotifier, TODAY, DAYS_BEFORE_DELETION);

    assert(Result.isErr(result));
});

Deno.test("it should do nothing when there are no expired salaries", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const rgpdNotifier = StubRgpdNotifier.new();
    const salary1 = Salary.generate({ date: SalaryDate.generate({ raw: EIGHT_DAYS_AGO }) });
    const salary2 = Salary.generate({ date: SalaryDate.generate({ raw: SEVEN_DAYS_AGO }) });
    const salary3 = Salary.generate({ date: SalaryDate.generate({ raw: SEVEN_DAYS_AGO }) });
    salaryRepo.insert(salary1);
    salaryRepo.insert(salary2);
    salaryRepo.insert(salary3);
    salaryRepo.confirm(salary1.id);
    salaryRepo.publish(salary1.id);
    salaryRepo.confirm(salary2.id);
    
    const result = await sendRgpdEmail(salaryRepo, rgpdNotifier, TODAY, DAYS_BEFORE_DELETION);

    assert(Result.isOk(result));
    assert(Result.unwrap(result) === "no_expired_emails");
});

Deno.test("it should return an error when the email could not be sent", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const rgpdNotifier = StubRgpdNotifier.withError();
    const salary = Salary.generate({ date: SalaryDate.generate({ raw: EIGHT_DAYS_AGO }) })
    salaryRepo.insert(salary);

    const result = await sendRgpdEmail(salaryRepo, rgpdNotifier, TODAY, DAYS_BEFORE_DELETION);

    assert(Result.isErr(result));
});

Deno.test("it should return ok otherwise", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const rgpdNotifier = StubRgpdNotifier.new();
    const salary = Salary.generate({ date: SalaryDate.generate({ raw: EIGHT_DAYS_AGO }) })
    salaryRepo.insert(salary);

    const result = await sendRgpdEmail(salaryRepo, rgpdNotifier, TODAY, DAYS_BEFORE_DELETION);

    assert(Result.isOk(result));
    assert(Result.unwrap(result) === undefined);
});