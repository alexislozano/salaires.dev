import { Result } from "@utils";

export type Location = {
    _type: "location";
    raw: string;
};

export const Location = {
    toString(location: Location): string {
        return location.raw;
    },
    tryFromString(raw: string): Result<Location, "empty"> {
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "location", raw });
    }
};
