import { Context } from "hono";
import { z } from "zod";
import { confirmToken, fetchSalaries, Token } from "@domain";
import { AdminNotifier, SalaryRepository, TokenRepository } from "@infra";
import { Home, TextOnly } from "../pages/mod.ts";
import { buildOrder, parseQuery } from "./utils/mod.ts"
import { Maybe, Result } from "@utils";
import { I18n } from "../i18n.ts";

const querySchema = z.object({
    key: z.string().array().optional(),
    direction: z.string().array().optional(),
    token: z.string().array().optional(),
});

export async function get(
    c: Context,
    salaryRepo: SalaryRepository,
    tokenRepo: TokenRepository,
    adminNotifier: AdminNotifier,
) {
    const query = parseQuery(c, querySchema);
    const order = buildOrder(query);

    const salariesResult = await fetchSalaries(order, salaryRepo);
    if (Result.isErr(salariesResult)) { return c.html(TextOnly({
        text: I18n.translate("salaries_fetching_error")
    })); } 

    return c.html(Home({
        salaries: Result.unwrap(salariesResult),
        order,
        notification: await buildNotification(
            query,
            salaryRepo,
            tokenRepo,
            adminNotifier
        )
    }));
}

async function buildNotification(
    query: z.infer<typeof querySchema>,
    salaryRepo: SalaryRepository,
    tokenRepo: TokenRepository,
    adminNotifier: AdminNotifier,
): Promise<Maybe<string>> {
    if (! query.token || query.token.length === 0) { return Maybe.none(); }
    
    const token = Token.tryFromString(query.token[0]);
    if (Result.isErr(token)) { return Maybe.some(I18n.translate("token_confirmation_error")); }

    return Result.match(await confirmToken(tokenRepo, salaryRepo, adminNotifier, Result.unwrap(token)), {
        onOk: () => Maybe.some(I18n.translate("token_confirmation_success")),
        onErr: () => Maybe.some(I18n.translate("token_confirmation_error")),
    });
}