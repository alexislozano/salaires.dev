import { Compare, Result, String } from "@utils";
import { Direction } from "@domain";

export type Compensation = {
    _type: "compensation";
    raw: number;
};

export type CompensationError =
    | "negative"
    | "not_a_number"
    | "not_an_integer"
    ;

export const Compensation = {
    compare(a: Compensation, b: Compensation, direction: Direction): number {
        switch (direction) {
            case "desc": return Compare.desc(a.raw, b.raw);
            case "asc": return Compare.asc(a.raw, b.raw);
        }
    },
    generate(): Compensation {
        return {
            _type: "compensation",
            raw: 50000
        };
    },
    toNumber(compensation: Compensation): number {
        return compensation.raw;
    },
    toString(compensation: Compensation): string {
        return `${Math.round(compensation.raw / 1000)}K`;
    },
    tryFromNumber(raw: number): Result<Compensation, CompensationError> {
        if (raw < 0) { return Result.err("negative"); }
        if (! Number.isInteger(raw)) { return Result.err("not_an_integer"); }
        return Result.ok({ _type: "compensation", raw });
    },
    tryFromString(compensation: string): Result<Compensation, CompensationError> {
        const result = String.tryToNumber(compensation);
        if (Result.isErr(result)) { return result; }
        return Compensation.tryFromNumber(Result.unwrap(result));
    }
};
