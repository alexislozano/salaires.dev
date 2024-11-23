import { Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { LocationRepository } from "./mod.ts";
import { Location, LocationError } from "@domain";
import { z } from "zod";

export class SupabaseLocationRepository implements LocationRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseLocationRepository(repo);
    }

    async fetchAll(): Promise<Result<Location[], string>> {
        const response = await this.repo.get("locations?select=*&order=location");
        if (! response.ok) { return Result.err("could not send request"); }
        
        const supabaseLocations = z
            .array(supabaseLocationSchema)
            .safeParse(await response.json());
        if (! supabaseLocations.success) { return Result.err("could not parse json"); }

        const locations: Location[] = [];
        for (const supabaseLocation of supabaseLocations.data) {
            const location = SupabaseLocation.tryToLocation(supabaseLocation);
            if (Result.isErr(location)) { return Result.err("could not convert to domain"); }
            locations.push(Result.unwrap(location));
        }

        return Result.ok(locations);
    }
}

const supabaseLocationSchema = z.object({
    location: z.string()
});
type SupabaseLocation = z.infer<typeof supabaseLocationSchema>;

const SupabaseLocation = {
    tryToLocation(location: SupabaseLocation): Result<Location, LocationError> {
        return Location.tryFromString(location.location);
    }
}