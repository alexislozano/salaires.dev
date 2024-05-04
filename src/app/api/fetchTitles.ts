import { Context } from "hono";
import * as lib from "@domain";
import { TitleRepository } from "@infra";
import { Result } from "@utils";

export async function fetchTitles(
    c: Context,
    titleRepo: TitleRepository
) {
    return Result.match(await lib.fetchTitles(titleRepo), {
        onOk: (titles) => c.json(titles.map(title => ({
            title: lib.Title.toString(title)
        }))),
        onErr: () => c.json([], 500)
    });
}
