import { Context } from "hono";
import { z } from "zod";
import { parse } from "./parse.ts";

export function parseQuery<S extends z.ZodType>(c: Context, schema: S): z.infer<S> {
    return parse(c.req.queries(), schema);
}
