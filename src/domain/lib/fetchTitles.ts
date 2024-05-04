import { Result } from "@utils";
import { TitleRepository } from "@infra";
import { Title } from "../models/mod.ts";

export async function fetchTitles(titleRepository: TitleRepository): Promise<Result<Title[], string>> {
    return Result.match(await titleRepository.fetchAll(), {
        onOk: (titles) => Result.ok(titles),
        onErr: (err) => Result.err(err)
    });
}