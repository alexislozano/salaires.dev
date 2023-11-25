import { Title } from "@domain";
import { Result } from "@utils";

export type TitleRepository = {
    fetchAll: () => Result<Title[], string>;
};
