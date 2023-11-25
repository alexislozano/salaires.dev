import { SalaryRepository, TokenRepository } from "@infra";
import { Result } from "@utils";
import { Token } from "../models/index.ts";

export function confirmToken(
    tokenRepository: TokenRepository,
    salaryRepository: SalaryRepository,
    token: Token,
): Result<void, string> {
    const salaryIdResult = tokenRepository.delete(token);
    if (Result.isErr(salaryIdResult)) { return salaryIdResult; }

    return salaryRepository.confirm(Result.unwrap(salaryIdResult));
}
