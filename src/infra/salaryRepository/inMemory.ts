import { InMemoryRepository } from "../utils/mod.ts";
import { SalaryRepository } from "./mod.ts";
import { ConfirmedSalary, Id, Key, Order, PublishedSalary, Salary, SalaryDate, WaitingSalary } from "@domain";
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
            filter,
            patch: Salary.confirm
        });

        function filter(s: Salary): s is WaitingSalary {
            return Salary.isWaiting(s) && s.id.raw === id.raw;
        }
    }

    async countExpiredSalaries(expirationDate: Date): Promise<Result<number, string>> {
        const fetchResult = await this.repo.fetchAll();

        return Result.map(
            fetchResult,
            salaries => salaries
                .filter(s => ! Salary.isPublished(s))
                .filter(s =>  SalaryDate.toDate(s.date) < expirationDate)
                .length
        );
    }

    async fetchAll(order: Order<Key>): Promise<Result<PublishedSalary[], string>> {
        const fetchResult = await this.repo.fetchAll();

        return Result.map(
            fetchResult,
            salaries => salaries
                .filter(Salary.isPublished)
                .toSorted((a, b) => Salary.compare(a, b, order)
            )
        )
    }

    insert(salary: WaitingSalary): Promise<Result<void, string>> {
        return this.repo.insert(salary);
    }

    publish(id: Id): Promise<Result<void, string>> {
        return this.repo.patch({
            filter,
            patch: Salary.publish
        });

        function filter(s: Salary): s is ConfirmedSalary {
            return Salary.isConfirmed(s) && s.id.raw === id.raw;
        }
    }
}
