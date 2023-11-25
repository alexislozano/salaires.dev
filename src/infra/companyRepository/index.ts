import { Result } from "@utils";
import { Company } from "@domain";

export type CompanyRepository = {
    fetchAll: () => Result<Company[], string>;
};
