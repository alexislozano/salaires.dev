import { InMemoryRepository } from "../utils/mod.ts";
import { SalaryRepository } from "./mod.ts";
import { Id, Key, Order, Salary } from "@domain";
import { Result } from "@utils";

export class InMemorySalaryRepository implements SalaryRepository {
    private repo: InMemoryRepository<Salary>;

    private constructor(salaries: Salary[], error: boolean) {
        this.repo = new InMemoryRepository(salaries, error);
    }

    static new() {
        return new InMemorySalaryRepository([], false);
    }

    static withError() {
        return new InMemorySalaryRepository([], true);
    }

    confirm(id: Id): Promise<Result<void, string>> {
        return this.repo.patch({
            filter: s => s.id.raw === id.raw,
            patch: Salary.confirm
        });
    }

    async fetchAll(order: Order<Key>): Promise<Result<Salary[], string>> {
        const fetchResult = await this.repo.fetchAll();

        return Result.map(
            fetchResult,
            salaries => salaries.toSorted((a, b) => Salary.compare(a, b, order))
        )
    }

    insert(salary: Salary): Promise<Result<void, string>> {
        return this.repo.insert(salary);
    }
}
