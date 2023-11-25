import { Location } from "@domain";
import { Result } from "@utils";

export type LocationRepository = {
    fetchAll: () => Result<Location[], string>;
};
