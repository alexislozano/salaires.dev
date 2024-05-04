import { TitleRepository } from "./mod.ts";
import { Title } from "@domain";
import { Result } from "@utils";

export class InMemoryTitleRepository implements TitleRepository {
    private titles: Title[];
    private error: boolean;

    private constructor(titles: Title[], error: boolean) {
        this.titles = titles;
        this.error = error;
    }

    static new() {
        return new InMemoryTitleRepository([], false);
    }

    static withError() {
        return new InMemoryTitleRepository([], true);
    }

    insert(title: Title) {
        this.titles.push(title);
    }

    fetchAll(): Promise<Result<Title[], string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        return Promise.resolve(Result.ok(this.titles));
    }
}
