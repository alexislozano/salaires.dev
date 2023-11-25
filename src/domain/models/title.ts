import { Result } from "@utils";

export type Title = {
    _type: "title";
    raw: string;
};

export const Title = {
    toString(title: Title): string {
        return title.raw;
    },
    tryFromString(raw: string): Result<Title, "empty"> {
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "title", raw });
    }
};
