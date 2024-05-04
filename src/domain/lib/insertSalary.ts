import {
    CaptchaService, 
    SalaryRepository, 
    TokenRepository,
    TokenSender
} from "@infra";
import { Result } from "@utils";
import { Captcha, Salary, Token } from "../models/mod.ts";

export async function insertSalary(
    captchaService: CaptchaService,
    salaryRepository: SalaryRepository,
    tokenRepository: TokenRepository,
    tokenSender: TokenSender,
    captcha: Captcha,
    salary: Salary,
): Promise<Result<void, string>> {
    const validationResult = await captchaService.validate(captcha);
    if (Result.isErr(validationResult)) { return validationResult; }

    const insertionResult = await salaryRepository.insert(salary);
    if (Result.isErr(insertionResult)) { return insertionResult; }

    const token = Token.generate();

    const sendingResult = await tokenSender.send(token, salary.email);
    if (Result.isErr(sendingResult)) { return sendingResult; }

    return tokenRepository.insert(salary.id, token);
}
