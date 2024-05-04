import { Context } from "hono";
import * as lib from "@domain";
import { LocationRepository } from "@infra";
import { Result } from "@utils";

export async function fetchLocations(
    c: Context,
    locationRepo: LocationRepository
) {
    return Result.match(await lib.fetchLocations(locationRepo), {
        onOk: (locations) => c.json(locations.map(location => ({
            location: lib.Location.toString(location)
        }))),
        onErr: () => c.json([], 500)
    });
}
