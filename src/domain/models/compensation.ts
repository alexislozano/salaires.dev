import { Result } from "@utils";

export type Compensation = {
    _type: "compensation";
    raw: number;
};

export const Compensation = {
    toNumber(compensation: Compensation): number {
        return compensation.raw;
    },
    tryFromNumber(raw: number): Result<Compensation, "negative" | "not_an_integer"> {
        if (raw < 0) { return Result.err("negative"); }
        if (! Number.isInteger(raw)) { return Result.err("not_an_integer"); }
        return Result.ok({ _type: "compensation", raw });
    },
    tryFromString(raw: string): Result<Compensation, "negative" | "not_a_number" | "not_an_integer"> {
        const number = Number(raw);
        if (Number.isNaN(number)) { return Result.err("not_a_number"); }
        return Compensation.tryFromNumber(number);
    }
};
