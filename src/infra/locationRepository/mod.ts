import { Location } from "@domain";
import { Result } from "@utils";

export type LocationRepository = {
    fetchAll: () => Promise<Result<Location[], string>>;
};

export { InMemoryLocationRepository } from "./inMemory.ts";
export { SupabaseLocationRepository } from "./supabase.ts";