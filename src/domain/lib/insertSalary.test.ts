import { assert } from "assert";
import { InMemorySalaryRepository, InMemoryTokenRepository, StubCaptchaService, StubTokenSender } from "@infra";
import { insertSalary } from "./insertSalary.ts";
import { Result } from "@utils";
import { Captcha, Salary } from "@domain";

const captcha = Captcha.generate();
const salary = Salary.generate();

Deno.test("it should return an error when the captcha is invalid", async () => {
    const captchaService = StubCaptchaService.withError();
    const salaryRepo = InMemorySalaryRepository.new();
    const tokenRepo = InMemoryTokenRepository.new();
    const tokenSender = StubTokenSender.new();

    const result = await insertSalary(captchaService, salaryRepo, tokenRepo, tokenSender, captcha, salary);

    assert(Result.isErr(result));
});

Deno.test("it should return an error when the salary could not be inserted", async () => {
    const captchaService = StubCaptchaService.new();
    const salaryRepo = InMemorySalaryRepository.withError();
    const tokenRepo = InMemoryTokenRepository.new();
    const tokenSender = StubTokenSender.new();

    const result = await insertSalary(captchaService, salaryRepo, tokenRepo, tokenSender, captcha, salary);

    assert(Result.isErr(result));
});

Deno.test("it should return an error when the token could not be inserted", async () => {
    const captchaService = StubCaptchaService.new();
    const salaryRepo = InMemorySalaryRepository.new();
    const tokenRepo = InMemoryTokenRepository.withError();
    const tokenSender = StubTokenSender.new();

    const result = await insertSalary(captchaService, salaryRepo, tokenRepo, tokenSender, captcha, salary);

    assert(Result.isErr(result));
});

Deno.test("it should return an error when the token could not be sent", async () => {
    const captchaService = StubCaptchaService.new();
    const salaryRepo = InMemorySalaryRepository.new();
    const tokenRepo = InMemoryTokenRepository.new();
    const tokenSender = StubTokenSender.withError();

    const result = await insertSalary(captchaService, salaryRepo, tokenRepo, tokenSender, captcha, salary);

    assert(Result.isErr(result));
});

Deno.test("it should return ok otherwise", async () => {
    const captchaService = StubCaptchaService.new();
    const salaryRepo = InMemorySalaryRepository.new();
    const tokenRepo = InMemoryTokenRepository.new();
    const tokenSender = StubTokenSender.new();

    const result = await insertSalary(captchaService, salaryRepo, tokenRepo, tokenSender, captcha, salary);

    assert(Result.isOk(result));
});