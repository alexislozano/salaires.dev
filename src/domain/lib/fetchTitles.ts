import { Result } from "@utils";
import { TitleRepository } from "@infra";
import { Title } from "../models/index.ts";

export function fetchTitles(titleRepository: TitleRepository): Result<Title[], string> {
    return Result.match(titleRepository.fetchAll(), {
        onOk: (titles) => Result.ok(titles),
        onErr: (err) => Result.err(err)
    });
}