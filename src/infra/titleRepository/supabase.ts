import { Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { TitleRepository } from "./mod.ts";
import { Title, TitleError } from "@domain";
import { z } from "zod";

const SERVICE = "SupabaseTitleRepository";

export class SupabaseTitleRepository implements TitleRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseTitleRepository(repo);
    }

    fetchAll(): Promise<Result<Title[], string>> {
        return this.repo.fetchAll({
            url: "titles?select=*&order=title",
            schema: supabaseTitleSchema,
            convert: SupabaseTitle.tryToTitle,
            service: SERVICE
        });
    }
}

const supabaseTitleSchema = z.object({
    title: z.string()
});
type SupabaseTitle = z.infer<typeof supabaseTitleSchema>;

const SupabaseTitle = {
    tryToTitle(title: SupabaseTitle): Result<Title, TitleError> {
        return Title.tryFromString(title.title);
    }
}