import * as uuid from "uuid";
import { Result } from "@utils";

export type Id = {
    _type: "id";
    raw: string;
};

export type IdError = "not_an_uuid";

export const Id = {
    generate(): Id {
        return { _type: "id", raw: crypto.randomUUID() };
    },
    toString(id: Id): string {
        return id.raw;
    },
    tryFromString(id: string): Result<Id, IdError> {
        const raw = id.trim();
        if (! uuid.validate(raw)) { return Result.err("not_an_uuid"); }
        return Result.ok({ _type: "id", raw });
    }
};
