import { Context } from "hono";
import { TextOnly } from "../pages/mod.ts";
import { I18n } from "../i18n.ts";

export function get(c: Context) {
    return c.html(TextOnly({
        text: I18n.translate("the_site_is_in_maintenance")
    }));
}