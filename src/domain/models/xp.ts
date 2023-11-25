import { Result } from "@utils";

export type Xp = {
    _type: "xp";
    raw: number;
};

export const Xp = {
    toNumber(xp: Xp): number {
        return xp.raw;
    },
    tryFromNumber(raw: number): Result<Xp, "negative" | "not_an_integer"> {
        if (raw < 0) { return Result.err("negative"); }
        if (! Number.isInteger(raw)) { return Result.err("not_an_integer"); }
        return Result.ok({ _type: "xp", raw });
    },
    tryFromString(raw: string): Result<Xp, "negative" | "not_a_number" | "not_an_integer"> {
        const number = Number(raw);
        if (Number.isNaN(number)) { return Result.err("not_a_number"); }
        return Xp.tryFromNumber(number);
    }
};
