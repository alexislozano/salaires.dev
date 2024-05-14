import * as datetime from "datetime";
import { Result } from "@utils";
import { Direction } from "@domain";

export type SalaryDate = {
    _type: "date"
    raw: Date;
}

export const SalaryDate = {
    compare(a: SalaryDate, b: SalaryDate, direction: Direction): number {
        switch (direction) {
            case "desc": return a.raw.getTime() === b.raw.getTime()
                ? (a.raw.getTime() < b.raw.getTime() ? -1 : 1)
                : 0;
            case "asc": return a.raw.getTime() === b.raw.getTime()
                ? (a.raw.getTime() < b.raw.getTime() ? 1 : -1)
                : 0;
        }
    },
    fromDate(raw: Date): SalaryDate {
        return { _type: "date", raw };
    },
    generate(): SalaryDate {
        return {
            _type: "date",
            raw: new Date(2022, 0, 1)
        }
    },
    tryFromString(date: string): Result<SalaryDate, void> {
        const raw = new Date(date);
        if (raw.toString() === "Invalid Date") { return Result.err(undefined); } 
        return Result.ok(SalaryDate.fromDate(raw));
    },
    toDate(date: SalaryDate): Date {
        return date.raw;
    },
    toString(date: SalaryDate, format: string): string {
        return datetime.format(date.raw, format);
    }
}
