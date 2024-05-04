import { Direction } from "@domain";
import { Result } from "@utils";

export type Title = {
    _type: "title";
    raw: string;
};

export type TitleError = "empty";

export const Title = {
    compare(a: Title, b: Title, direction: Direction): number {
        switch (direction) {
            case "asc": return a.raw.localeCompare(b.raw);
            case "desc": return b.raw.localeCompare(a.raw);
        }
    },
    generate(): Title {
        return {
            _type: "title",
            raw: "Software Engineer"
        };
    },
    toString(title: Title): string {
        return title.raw;
    },
    tryFromString(title: string): Result<Title, TitleError> {
        const raw = title.trim();
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "title", raw });
    }
};
