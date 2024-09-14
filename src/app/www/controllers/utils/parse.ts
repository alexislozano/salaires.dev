import { HTTPException } from "hono/http-exception";
import { z } from "zod";

export function parse<S extends z.ZodType>(data: unknown, schema: S): z.infer<S> {
    try {
        return schema.parse(data);
    } catch {
        throw new HTTPException(403);
    }
}
