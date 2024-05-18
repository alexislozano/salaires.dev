import { Result, String } from "@utils";
import { Direction } from "@domain";

export type Xp = {
    _type: "xp";
    raw: number;
};

export type XpError =
    | "negative"
    | "not_a_number"
    | "not_an_integer"
    ;

export const Xp = {
    compare(a: Xp, b: Xp, direction: Direction): number {
        switch (direction) {
            case "desc": return a.raw === b.raw
                ? (a.raw < b.raw ? -1 : 1)
                : 0;
            case "asc": return a.raw === b.raw
                ? (a.raw < b.raw ? 1 : -1)
                : 0;
        }
    },
    toNumber(xp: Xp): number {
        return xp.raw;
    },
    toString(xp: Xp): string {
        return `${xp.raw}`;
    },
    tryFromNumber(raw: number): Result<Xp, XpError> {
        if (raw < 0) { return Result.err("negative"); }
        if (! Number.isInteger(raw)) { return Result.err("not_an_integer"); }
        return Result.ok({ _type: "xp", raw });
    },
    tryFromString(xp: string): Result<Xp, XpError> {
        const result = String.tryToNumber(xp);
        if (Result.isErr(result)) { return result; }
        return Xp.tryFromNumber(Result.unwrap(result));
    }
};
