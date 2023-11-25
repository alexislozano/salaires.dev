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
    }
}
