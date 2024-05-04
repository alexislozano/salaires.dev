import { Title } from "@domain";
import { Result } from "@utils";

export type TitleRepository = {
    fetchAll: () => Promise<Result<Title[], string>>;
};

export { InMemoryTitleRepository } from "./inMemory.ts";
export { SupabaseTitleRepository } from "./supabase.ts";