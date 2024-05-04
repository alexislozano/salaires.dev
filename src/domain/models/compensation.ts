import { Result } from "@utils";
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
            case "desc": return a.raw === b.raw
                ? (a.raw < b.raw ? -1 : 1)
                : 0;
            case "asc": return a.raw === b.raw
            ? (a.raw < b.raw ? 1 : -1)
            : 0;
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
        return `${compensation.raw / 1000}K`;
    },
    tryFromNumber(raw: number): Result<Compensation, CompensationError> {
        if (raw < 0) { return Result.err("negative"); }
        if (! Number.isInteger(raw)) { return Result.err("not_an_integer"); }
        return Result.ok({ _type: "compensation", raw });
    },
    tryFromString(compensation: string): Result<Compensation, CompensationError> {
        const raw = compensation.trim();
        const number = Number(raw);
        if (Number.isNaN(number)) { return Result.err("not_a_number"); }
        return Compensation.tryFromNumber(number);
    }
};
