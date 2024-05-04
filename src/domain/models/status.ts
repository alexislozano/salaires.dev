import { Result, UnreachableCaseError } from "@utils";

const ALL_STATUSES = ["waiting", "confirmed", "published"] as const;

export type Status = typeof ALL_STATUSES[number];

export const Status = {
    all(): Readonly<Status[]> {
        return ALL_STATUSES;
    },
    generate(): Status {
        return "waiting";
    },
    toString(status: Status): string {
        switch (status) {
            case "waiting": return "WAITING";
            case "confirmed": return "CONFIRMED";
            case "published": return "PUBLISHED";
            default: throw new UnreachableCaseError(status);
        }
    },
    tryFromString(raw: string): Result<Status, "not_a_status"> {
        switch (raw.trim()) {
            case "WAITING": return Result.ok("waiting");
            case "CONFIRMED": return Result.ok("confirmed");
            case "PUBLISHED": return Result.ok("published");
            default: return Result.err("not_a_status");
        }
    }
};
