import { Env, Result } from "@utils";
import { LocationRepository } from "./mod.ts";
import { Location, LocationError } from "@domain";
import { z } from "zod";

export class SupabaseLocationRepository implements LocationRepository {
    private url: string;
    private key: string;

    private constructor(url: string, key: string) {
        this.url = url;
        this.key = key;
    }

    private headers(): Record<string, string> {
        return {
            apiKey: this.key,
            Authorization: `Bearer ${this.key}`,
            "Content-Type": "application/json",
        };
    }

    static new() {
        return new SupabaseLocationRepository(
            Env.get("SUPABASE_URL"),
            Env.get("SUPABASE_KEY")
        );
    }

    async fetchAll(): Promise<Result<Location[], string>> {
        const response = await fetch(`${this.url}locations?select=*&order=location`, {
            method: "GET",
            headers: this.headers()
        });
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