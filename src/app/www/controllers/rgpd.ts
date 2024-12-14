import { Context } from "hono";
import { Env } from "@utils";
import { Rgpd } from "../pages/mod.ts";

export function get(c: Context) {
    return c.html(Rgpd({
        email: Env.get("RGPD_EMAIL"),
        daysBeforeDeletion: Env.get("RGPD_DAYS_BEFORE_DELETION")
    }));
}