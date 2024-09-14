import { assert, assertEquals } from "assert";
import { InMemoryTitleRepository } from "@infra";
import { fetchTitles } from "./fetchTitles.ts";
import { Result } from "@utils";
import { Title } from "@domain";

Deno.test("it should return an error when an error occurs", async () => {
    const titleRepo = InMemoryTitleRepository.withError();

    const result = await fetchTitles(titleRepo);

    assert(Result.isErr(result));
});

Deno.test("it should return all titles otherwise", async () => {
    const title = Title.generate();
    const titleRepo = InMemoryTitleRepository.new();
    titleRepo.insert(title);

    const result = await fetchTitles(titleRepo);

    assert(Result.isOk(result));
    assertEquals(Result.unwrap(result), [title]);
});
