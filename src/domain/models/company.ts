import { Result } from "@utils";

export type Company = {
    _type: "company";
    raw: string;
};

export const Company = {
    toString(company: Company): string {
        return company.raw;
    },
    tryFromString(raw: string): Result<Company, "empty"> {
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "company", raw });
    }
};
