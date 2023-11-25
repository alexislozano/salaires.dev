import { Result } from "@utils";

const ALL_LEVELS = ["junior", "mid", "senior"] as const;

export type Level = typeof ALL_LEVELS[number];

export const Level = {
    all(): Readonly<Level[]> {
        return ALL_LEVELS;
    },
    toString(level: Level): string {
        switch (level) {
            case "junior": return "Junior";
            case "mid": return "Mid";
            case "senior": return "Senior";
        }
    },
    tryFromString(raw: string): Result<Level, "not_a_level"> {
        switch (raw) {
            case "Junior": return Result.ok("junior");
            case "Mid": return Result.ok("mid");
            case "Senior": return Result.ok("senior");
            default: return Result.err("not_a_level");
        }
    }
};
