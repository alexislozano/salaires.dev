import { Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { LocationRepository } from "./mod.ts";
import { Location, LocationError } from "@domain";
import { z } from "zod";

const SERVICE = "SupabaseLocationRepository";

export class SupabaseLocationRepository implements LocationRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseLocationRepository(repo);
    }

    fetchAll(): Promise<Result<Location[], string>> {
        return this.repo.fetchAll({
            url: "locations?select=*&order=location",
            schema: supabaseLocationSchema,
            convert: SupabaseLocation.tryToLocation,
            service: SERVICE
        })
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