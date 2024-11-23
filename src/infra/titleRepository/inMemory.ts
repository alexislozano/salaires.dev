import { InMemoryRepository } from "../utils/mod.ts";
import { TitleRepository } from "./mod.ts";
import { Title } from "@domain";
import { Result } from "@utils";

export class InMemoryTitleRepository implements TitleRepository {
    private repo: InMemoryRepository<Title>;

    private constructor(titles: Title[], error: boolean) {
        this.repo = new InMemoryRepository(titles, error);
    }

    static new() {
        return new InMemoryTitleRepository([], false);
    }

    static withError() {
        return new InMemoryTitleRepository([], true);
    }

    insert(title: Title) {
        this.repo.insert(title);
    }

    fetchAll(): Promise<Result<Title[], string>> {
        return this.repo.fetchAll();
    }
}
