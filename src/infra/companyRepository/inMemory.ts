import { CompanyRepository } from "./index.ts";
import { Company } from "@domain";
import { Result } from "@utils";

type State = {
    companies: Company[],
    error: boolean;
}

export const InMemoryCompanyRepository = {
    new(): CompanyRepository {
        return build({ companies: [], error: false });        
    },
    withError(): CompanyRepository {
        return build({ companies: [], error: true });
    }
};

function build(state: State): CompanyRepository {
    return {
        fetchAll: () => {
            if (state.error) {
                return Result.err("error flag is on");
            }

            return Result.ok(state.companies);
        }
    };   
}
