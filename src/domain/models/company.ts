import { Direction } from "@domain";
import { Result } from "@utils";

export type Company = {
    _type: "company";
    raw: string;
};

export type CompanyError = "empty";

export const Company = {
    compare(a: Company, b: Company, direction: Direction): number {
        switch (direction) {
            case "asc": return a.raw.localeCompare(b.raw);
            case "desc": return b.raw.localeCompare(a.raw);
        }
    },
    generate(): Company {
        return {
            _type: "company",
            raw: "Google"
        };
    },
    toString(company: Company): string {
        return company.raw;
    },
    tryFromString(company: string): Result<Company, CompanyError> {
        const raw = company.trim();
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "company", raw });
    }
};
