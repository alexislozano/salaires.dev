import { Result } from "./result.ts";

export const String = {
    tryToNumber(s: string): Result<number, "not_a_number"> {
        const trimmed = s.trim();
        if (trimmed.length === 0) { return Result.err("not_a_number"); }
        const parsed = Number(trimmed);
        if (Number.isNaN(parsed)) { return Result.err("not_a_number"); }
        return Result.ok(parsed); 
    }
}