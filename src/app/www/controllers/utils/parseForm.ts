import { Context } from "hono";
import { z } from "zod";
import { parse } from "./parse.ts";

export async function parseForm<S extends z.ZodType>(c: Context, schema: S): Promise<z.infer<S>> {
    return parse(await c.req.parseBody(), schema);
}
