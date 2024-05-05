import { Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { TitleRepository } from "./mod.ts";
import { Title, TitleError } from "@domain";
import { z } from "zod";

export class SupabaseTitleRepository implements TitleRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseTitleRepository(repo);
    }

    async fetchAll(): Promise<Result<Title[], string>> {
        const response = await this.repo.fetch("titles?select=*&order=title");
        if (! response.ok) { return Result.err("could not send request"); }
        
        const supabaseTitles = z
            .array(supabaseTitleSchema)
            .safeParse(await response.json());
        if (! supabaseTitles.success) { return Result.err("could not parse json"); }

        const titles: Title[] = [];
        for (const supabaseTitle of supabaseTitles.data) {
            const title = SupabaseTitle.tryToTitle(supabaseTitle);
            if (Result.isErr(title)) { return Result.err("could not convert to domain"); }
            titles.push(Result.unwrap(title));
        }

        return Result.ok(titles);
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