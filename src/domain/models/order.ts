import { UnreachableCaseError } from "@utils";

export type Order<K> = {
    key: K;
    direction: Direction;
};

export type Direction = "asc" | "desc";

export const Direction = {
    default(): Direction {
        return "desc";
    },
    fromString(direction: string): Direction {
        switch (direction.trim()) {
            case "asc": return "asc";
            case "desc": return "desc";
            default: return Direction.default();
        }
    },
    reverse(direction: Direction): Direction {
        switch (direction) {
            case "asc": return "desc";
            case "desc": return "asc";
            default: throw new UnreachableCaseError(direction);
        }
    }
};