import { SalaryRepository, TokenRepository } from "@infra";
import { Result } from "@utils";
import { Token } from "../models/mod.ts";

export async function confirmToken(
    tokenRepository: TokenRepository,
    salaryRepository: SalaryRepository,
    token: Token,
): Promise<Result<void, string>> {
    const salaryIdResult = await tokenRepository.delete(token);
    if (Result.isErr(salaryIdResult)) { return salaryIdResult; }

    return salaryRepository.confirm(Result.unwrap(salaryIdResult));
}
