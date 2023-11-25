import { Result } from "@utils";
import { LocationRepository } from "@infra";
import { Location } from "../models/index.ts";

export function fetchLocations(locationRepository: LocationRepository): Result<Location[], string> {
    return Result.match(locationRepository.fetchAll(), {
        onOk: (locations) => Result.ok(locations),
        onErr: (err) => Result.err(err)
    });
}