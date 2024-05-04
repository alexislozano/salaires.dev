import { LocationRepository } from "./mod.ts";
import { Location } from "@domain";
import { Result } from "@utils";

export class InMemoryLocationRepository implements LocationRepository {
    private locations: Location[];
    private error: boolean;

    private constructor(locations: Location[], error: boolean) {
        this.locations = locations;
        this.error = error;
    }

    static new() {
        return new InMemoryLocationRepository([], false);
    }

    static withError() {
        return new InMemoryLocationRepository([], true);
    }

    insert(location: Location) {
        this.locations.push(location);
    }

    fetchAll(): Promise<Result<Location[], string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        return Promise.resolve(Result.ok(this.locations));
    }
}
