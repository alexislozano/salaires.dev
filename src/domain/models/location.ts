import { Result } from "@utils";
import { Direction } from "@domain";

export type Location = {
    _type: "location";
    raw: string;
};

export type LocationError = "empty";

export const Location = {
    compare(a: Location, b: Location, direction: Direction): number {
        switch (direction) {
            case "asc": return a.raw.localeCompare(b.raw);
            case "desc": return b.raw.localeCompare(a.raw);
        }
    },
    generate(): Location {
        return {
            _type: "location",
            raw: "Paris"
        };
    },
    toString(location: Location): string {
        return location.raw;
    },
    tryFromString(location: string): Result<Location, LocationError> {
        const raw = location.trim();
        if (raw.length === 0) { return Result.err("empty"); }
        return Result.ok({ _type: "location", raw });
    }
};
