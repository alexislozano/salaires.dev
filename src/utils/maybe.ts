type Some<T> = { raw: T, _type: "some" };
type None = { _type: "none" };

export type Maybe<T> =
    | Some<T>
    | None;

export const Maybe = {
    some<T>(raw: T): Maybe<T> {
        return { raw, _type: "some" };
    },

    none<T>(): Maybe<T> {
        return { _type: "none" };
    },

    isSome<T>(maybe: Maybe<T>): maybe is Some<T> {
        return maybe._type === "some";
    },

    isNone<T>(maybe: Maybe<T>): maybe is None {
        return maybe._type === "none";
    },

    map<T, U>(maybe: Maybe<T>, f: (_: T) => U): Maybe<U> {
        switch (maybe._type) {
            case "some": return Maybe.some(f(maybe.raw));
            case "none": return maybe;
        }
    },

    bind<T, U>(maybe: Maybe<T>, f: (_: T) => Maybe<U>): Maybe<U> {
        switch (maybe._type) {
            case "some": return f(maybe.raw);
            case "none": return maybe;
        }
    },

    match<T, U>(
        maybe: Maybe<T>,
        { onSome, onNone }: { onSome: (_: T) => U , onNone: () => U }
    ): U {
        switch (maybe._type) {
            case "some": return onSome(maybe.raw);
            case "none": return onNone();
        }
    },

    unwrap<T>(some: Some<T>): T {
        return some.raw;
    },

    fromNullable<T>(nullable: T | null): Maybe<T> {
        if (nullable === null) { return Maybe.none(); }
        return Maybe.some(nullable);
    },

    toNullable<T>(maybe: Maybe<T>): T | null {
        switch (maybe._type) {
            case "some": return maybe.raw;
            case "none": return null;
        }
    },

    compare<T>(
        a: Maybe<T>,
        b: Maybe<T>,
        direction: "asc" | "desc",
        compare: (_a: T, _b: T, direction: "asc" | "desc") => number
    ): number {
        if (Maybe.isNone(a) && Maybe.isNone(b)) { return 0; }
        if (Maybe.isNone(a)) { return direction === "asc" ? -1 : 1; }
        if (Maybe.isNone(b)) { return direction === "asc" ? 1 : -1; }
        else { return compare(Maybe.unwrap(a), Maybe.unwrap(b), direction); }
    }
}
