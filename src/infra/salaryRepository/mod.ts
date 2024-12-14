import { Result } from "@utils";
import { Id, Key, Order, PublishedSalary, WaitingSalary } from "@domain";

export type SalaryRepository = {
    confirm: (id: Id) => Promise<Result<void, string>>;
    fetchAll: (order: Order<Key>) => Promise<Result<PublishedSalary[], string>>;
    insert: (salary: WaitingSalary) => Promise<Result<void, string>>;
}

export { InMemorySalaryRepository } from "./inMemory.ts";
export { SupabaseSalaryRepository } from "./supabase.ts";