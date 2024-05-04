import { Direction, Key, Order } from "@domain";

export function buildOrder({ key, direction }: {
    key?: string[];
    direction?: string[];
}): Order<Key> {
    return {
        key: (key && key.length !== 0)
            ? Key.fromString(key[0])
            : Key.default(),
        direction: (direction && direction.length !== 0)
            ? Direction.fromString(direction[0])
            : Direction.default()
    };
}