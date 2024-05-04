import { Result } from "@utils";
import { LocationRepository } from "@infra";
import { Location } from "../models/mod.ts";

export async function fetchLocations(locationRepository: LocationRepository): Promise<Result<Location[], string>> {
    return Result.match(await locationRepository.fetchAll(), {
        onOk: (locations) => Result.ok(locations),
        onErr: (err) => Result.err(err)
    });
}