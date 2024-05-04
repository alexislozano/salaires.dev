import { Env, Result } from "@utils";
import { TitleRepository } from "./mod.ts";
import { Title, TitleError } from "@domain";
import { z } from "zod";

export class SupabaseTitleRepository implements TitleRepository {
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
        return new SupabaseTitleRepository(
            Env.get("SUPABASE_URL"),
            Env.get("SUPABASE_KEY")
        );
    }

    async fetchAll(): Promise<Result<Title[], string>> {
        const response = await fetch(`${this.url}titles?select=*&order=title`, {
            method: "GET",
            headers: this.headers()
        });
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