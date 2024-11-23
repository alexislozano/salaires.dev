import { InMemoryRepository } from "../utils/mod.ts";
import { LocationRepository } from "./mod.ts";
import { Location } from "@domain";
import { Result } from "@utils";

export class InMemoryLocationRepository implements LocationRepository {
    private repo: InMemoryRepository<Location>;

    private constructor(locations: Location[], error: boolean) {
        this.repo = new InMemoryRepository(locations, error);
    }

    static new() {
        return new InMemoryLocationRepository([], false);
    }

    static withError() {
        return new InMemoryLocationRepository([], true);
    }

    insert(location: Location) {
        this.repo.insert(location);
    }

    fetchAll(): Promise<Result<Location[], string>> {
        return this.repo.fetchAll();
    }
}
