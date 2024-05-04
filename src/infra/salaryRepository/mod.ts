import { Result } from "@utils";
import { Id, Key, Order, Salary } from "@domain";

export type SalaryRepository = {
    confirm: (id: Id) => Promise<Result<void, string>>;
    fetchAll: (order: Order<Key>) => Promise<Result<Salary[], string>>;
    insert: (salary: Salary) => Promise<Result<void, string>>;
}

export { InMemorySalaryRepository } from "./inMemory.ts";
export { SupabaseSalaryRepository } from "./supabase.ts";