import { Context } from "hono";
import { Notification } from "../components/mod.ts";
import { Maybe } from "@utils";

export function del(c: Context) {
    return c.html(Notification({
        notification: Maybe.none()
    }));
}