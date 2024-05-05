import { SalaryRepository } from "./mod.ts";
import { Id, Key, Order, Salary } from "@domain";
import { Result } from "@utils";

export class InMemorySalaryRepository implements SalaryRepository {
    private salaries: Salary[];
    private error: boolean;

    private constructor(salaries: Salary[], error: boolean) {
        this.salaries = salaries;
        this.error = error;
    }

    static new() {
        return new InMemorySalaryRepository([], false);
    }

    static withError() {
        return new InMemorySalaryRepository([], true);
    }

    confirm(id: Id): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        const index = this.salaries
            .findIndex(s => s.id.raw === id.raw);

        if (index == -1) {
            return Promise.resolve(Result.err("salary not found"));
        }

        this.salaries[index] = Salary.confirm(this.salaries[index]);

        return Promise.resolve(Result.ok(undefined));
    }

    fetchAll(order: Order<Key>): Promise<Result<Salary[], string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        const salaries = this.salaries.toSorted((a, b) => Salary.compare(a, b, order));

        return Promise.resolve(Result.ok(salaries));
    }

    insert(salary: Salary): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        this.salaries.push(salary);

        return Promise.resolve(Result.ok(undefined));
    }
}
