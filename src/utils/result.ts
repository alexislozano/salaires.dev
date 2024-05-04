type Ok<T> = { raw: T, _type: "ok" };
type Err<E> = { raw: E, _type: "err" };

export type Result<T, E> =
    | Ok<T>
    | Err<E>;

export const Result = {
    ok<T, E>(raw: T): Result<T, E> {
        return { raw, _type: "ok" };
    },

    err<T, E>(raw: E): Result<T, E> {
        return { raw, _type: "err" };
    },

    isOk<T, E>(result: Result<T, E>): result is Ok<T> {
        return result._type === "ok";
    },

    isErr<T, E>(result: Result<T, E>): result is Err<E> {
        return result._type === "err";
    },

    map<T, U, E>(result: Result<T, E>, f: (_: T) => U): Result<U, E> {
        switch (result._type) {
            case "ok": return Result.ok(f(result.raw));
            case "err": return result;
        }
    },

    bind<T, U, E, F>(result: Result<T, E>, f: (_: T) => Result<U, F>): Result<U, E | F> {
        switch (result._type) {
            case "ok": return f(result.raw);
            case "err": return result;
        }
    },

    match<T, U, E>(
        result: Result<T, E>,
        { onOk, onErr }: { onOk: (_: T) => U, onErr: (_: E) => U }
    ): U {
        switch (result._type) {
            case "ok": return onOk(result.raw);
            case "err": return onErr(result.raw);
        }
    },

    unwrap<T>(ok: Ok<T>): T {
        return ok.raw;
    },

    unwrapErr<E>(err: Err<E>): E {
        return err.raw;
    }
}