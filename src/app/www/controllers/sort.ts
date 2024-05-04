import { Context } from "hono";
import { z } from "zod";
import { fetchSalaries } from "@domain";
import { SalaryRepository } from "@infra";
import { Maybe, Result } from "@utils";
import { SalaryTable } from "../fragments/mod.ts";
import { buildOrder, parseQuery } from "./utils/mod.ts";
import { Notification } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

const querySchema = z.object({
    key: z.string().array().optional(),
    direction: z.string().array().optional(),
});

export async function get(
    c: Context,
    salaryRepo: SalaryRepository
) {
    const query = parseQuery(c, querySchema);
    const order = buildOrder(query);

    const salariesResult = await fetchSalaries(order, salaryRepo);
    if (Result.isErr(salariesResult)) { return c.html(Notification({
        notification: Maybe.some(I18n.translate("sort_error"))
    })); } 

    return c.html(SalaryTable({
        salaries: Result.unwrap(salariesResult),
        order
    }));
}