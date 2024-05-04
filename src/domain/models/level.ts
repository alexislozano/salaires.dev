import { Result, UnreachableCaseError } from "@utils";
import { Direction } from "@domain";

const ALL_LEVELS = ["junior", "mid", "senior"] as const;

export type Level = typeof ALL_LEVELS[number];

export type LevelError = "not_a_level";

export const Level = {
    all(): Readonly<Level[]> {
        return ALL_LEVELS;
    },
    compare(a: Level, b: Level, direction: Direction): number {
        switch (direction) {
            case "asc": return a.localeCompare(b);
            case "desc": return b.localeCompare(a);
        }
    },
    toString(level: Level): string {
        switch (level) {
            case "junior": return "Junior";
            case "mid": return "Mid";
            case "senior": return "Senior";
            default: throw new UnreachableCaseError(level);
        }
    },
    tryFromString(raw: string): Result<Level, LevelError> {
        switch (raw.trim()) {
            case "Junior": return Result.ok("junior");
            case "Mid": return Result.ok("mid");
            case "Senior": return Result.ok("senior");
            default: return Result.err("not_a_level");
        }
    }
};
