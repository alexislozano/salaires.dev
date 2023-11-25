import { Result } from "@utils";

const ALL_STATUSES = ["waiting", "confirmed", "published"] as const;

export type Status = typeof ALL_STATUSES[number];

export const Status = {
    all(): Readonly<Status[]> {
        return ALL_STATUSES;
    },
    toString(status: Status): string {
        switch (status) {
            case "waiting": return "WAITING";
            case "confirmed": return "CONFIRMED";
            case "published": return "PUBLISHED";
        }
    },
    tryFromString(raw: string): Result<Status, "not_a_status"> {
        switch (raw) {
            case "WAITING": return Result.ok("waiting");
            case "CONFIRMED": return Result.ok("confirmed");
            case "PUBLISHED": return Result.ok("published");
            default: return Result.err("not_a_status");
        }
    }
};
