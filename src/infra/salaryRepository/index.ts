import { Result } from "@utils";
import { Id, Salary } from "@domain";

export type SalaryRepository = {
    confirm: (id: Id) => Result<void, string>;
    fetchAll: () => Result<Salary[], string>;
    insert: (salary: Salary) => Result<void, string>;
}