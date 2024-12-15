import { RgpdNotifier, SalaryRepository } from "@infra";
import { Env, Result } from "@utils";

export async function sendRgpdEmail(
    salaryRepo: SalaryRepository,
    rgpdNotifier: RgpdNotifier,
    today = new Date(),
    daysBeforeDeletion = Number(Env.get("RGPD_DAYS_BEFORE_DELETION"))
): Promise<Result<string | void, string>> {
    const expirationDate = new Date(today.getTime() - (daysBeforeDeletion * 24 * 60 * 60 * 1000));

    const countResult = await salaryRepo.countExpiredSalaries(expirationDate);
    if (Result.isErr(countResult)) { return countResult; }

    const count = Result.unwrap(countResult);
    if (count === 0) { return Result.ok("no_expired_emails"); }

    return rgpdNotifier.notify();
}