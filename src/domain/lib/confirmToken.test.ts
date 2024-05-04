import { assert } from "asserts";
import { InMemorySalaryRepository, InMemoryTokenRepository } from "@infra";
import { confirmToken } from "./confirmToken.ts";
import { Result } from "@utils";
import { Salary, Token } from "@domain";

const token = Token.generate();
const salary = Salary.generate();

Deno.test("it should return an error when the token could not be found", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    salaryRepo.insert(salary);
    const tokenRepo = InMemoryTokenRepository.new();

    const result = await confirmToken(tokenRepo, salaryRepo, token);

    assert(Result.isErr(result));
});

Deno.test("it should return an error when the token could not be deleted", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    salaryRepo.insert(salary);
    const tokenRepo = InMemoryTokenRepository.withError();

    const result = await confirmToken(tokenRepo, salaryRepo, token);

    assert(Result.isErr(result));
});

Deno.test("it should return an error when the salary could not be found", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    const tokenRepo = InMemoryTokenRepository.new();
    tokenRepo.insert(salary.id, token);

    const result = await confirmToken(tokenRepo, salaryRepo, token);

    assert(Result.isErr(result));
});

Deno.test("it should return an error when the salary could not be confirmed", async () => {
    const salaryRepo = InMemorySalaryRepository.withError();
    const tokenRepo = InMemoryTokenRepository.new();
    tokenRepo.insert(salary.id, token);

    const result = await confirmToken(tokenRepo, salaryRepo, token);

    assert(Result.isErr(result));
});

Deno.test("it should return ok otherwise", async () => {
    const salaryRepo = InMemorySalaryRepository.new();
    salaryRepo.insert(salary);
    const tokenRepo = InMemoryTokenRepository.new();
    tokenRepo.insert(salary.id, token);

    const result = await confirmToken(tokenRepo, salaryRepo, token);

    assert(Result.isErr(result));
});
