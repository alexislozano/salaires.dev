import { AdminNotifier, SalaryRepository, TokenRepository } from "@infra";
import { Result } from "@utils";
import { Token } from "../models/mod.ts";

export async function confirmToken(
    tokenRepository: TokenRepository,
    salaryRepository: SalaryRepository,
    adminNotifier: AdminNotifier,
    token: Token,
): Promise<Result<void, string>> {
    const salaryIdResult = await tokenRepository.delete(token);
    if (Result.isErr(salaryIdResult)) { return salaryIdResult; }

    const confirmationResult = await salaryRepository.confirm(Result.unwrap(salaryIdResult));
    if (Result.isErr(confirmationResult)) { return confirmationResult; }

    return Result.match(await adminNotifier.notify(), {
        onOk: () => Result.ok(undefined),
        onErr: () => Result.ok(undefined),
    });
}
