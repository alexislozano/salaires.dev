import { SalaryRepository } from "./mod.ts";
import {
    Company,
    Compensation,
    Email,
    Id,
    Key,
    Level,
    Location,
    Order,
    PublishedSalary,
    Remote,
    Salary,
    SalaryDate,
    Status,
    Title,
    Xp,
    WaitingSalary
} from "@domain";
import { Maybe, Result } from "@utils";
import { SupabaseRepository } from "../utils/mod.ts";
import { z } from "zod";

const SERVICE = "SupabaseSalaryRepository";

export class SupabaseSalaryRepository implements SalaryRepository {
    private repo: SupabaseRepository;

    private constructor(repo: SupabaseRepository) {
        this.repo = repo;
    }

    static new(repo: SupabaseRepository) {
        return new SupabaseSalaryRepository(repo);
    }

    confirm(id: Id): Promise<Result<void, string>> {
        return this.repo.patch({
            url: `salaries?id=eq.${Id.toString(id)}`,
            body: SupabaseStatus.fromStatus("confirmed"),
            service: SERVICE
        });
    }

    countExpiredSalaries(expirationDate: Date): Promise<Result<number, string>> {
        const url = `salaries?select=id&status=in.(${Status.toString("waiting")},${Status.toString("confirmed")})&date=lt.${expirationDate.toISOString()}`;

        return this.repo.count({
            url,
            service: SERVICE
        });
    }

    async fetchAll(order: Order<Key>): Promise<Result<PublishedSalary[], string>> {
        const supabaseOrder = SupabaseOrder.fromOrder(order);
        const url = `salaries?select=*&status=eq.${Status.toString("published")}&order=${supabaseOrder.key}.${supabaseOrder.direction}`;

        const fetchResult = await this.repo.fetchAll({
            url,
            schema: supabaseSalarySchema,
            convert: SupabaseSalary.tryToSalary,
            service: SERVICE
        });

        return Result.map(
            fetchResult,
            salaries => salaries.toSorted((a, b) => Salary.compare(a, b, order))
        );
    }

    insert(salary: WaitingSalary): Promise<Result<void, string>> {
        return this.repo.insert({
            url: "salaries",
            body: SupabaseSalary.fromSalary(salary),
            service: SERVICE
        });
    }
}

const supabaseSalarySchema = z.object({
    id: z.string(),
    email: z.string().nullable(),
    company: z.string(),
    title: z.string().nullable(),
    location: z.string(),
    compensation: z.number(),
    date: z.string(),
    level: z.string().nullable(),
    company_xp: z.number().nullable(),
    total_xp: z.number().nullable(),
    remote_variant: z.string().nullable(),
    remote_day_count: z.number().nullable(),
    remote_base: z.string().nullable(),
    remote_location: z.string().nullable(),
    status: z.string(),
});
type SupabaseSalary = z.infer<typeof supabaseSalarySchema>;

const SupabaseSalary = {
    fromSalary(salary: WaitingSalary): SupabaseSalary {
        return {
            id: Id.toString(salary.id),
            email: Email.toString(salary.email),
            company: Company.toString(salary.company),
            title: Maybe.toNullable(Maybe.map(salary.title, Title.toString)),
            location: Location.toString(salary.location),
            compensation: Compensation.toNumber(salary.compensation),
            date: SalaryDate.toString(salary.date, "yyyy-MM-ddTHH:mm:ss"),
            level: Maybe.toNullable(Maybe.map(salary.level, Level.toString)),
            company_xp: Maybe.toNullable(Maybe.map(salary.companyXp, Xp.toNumber)),
            total_xp: Maybe.toNullable(Maybe.map(salary.totalXp, Xp.toNumber)),
            status: Status.toString(salary.status),
            ...buildRemoteFields(salary.remote)
        };

        function buildRemoteFields(maybeRemote: Maybe<Remote>): {
            remote_variant: string | null,
            remote_day_count: number | null,
            remote_base: string | null,
            remote_location: string | null
        } {
            if (Maybe.isNone(maybeRemote)) { return {
                remote_variant: null,
                remote_day_count: null,
                remote_base: null,
                remote_location: null
            }; }
            const rawRemote = Remote.toRaw(Maybe.unwrap(maybeRemote));
            return {
                remote_variant: rawRemote.variant,
                remote_day_count: Maybe.toNullable(rawRemote.dayCount),
                remote_base: Maybe.toNullable(rawRemote.base),
                remote_location: Maybe.toNullable(rawRemote.location),
            };
        }
    },
    tryToSalary(salary: SupabaseSalary): Result<PublishedSalary, void> {
        if (salary.email) { return Result.err(undefined); }
        
        const status: Result<"published", string> = Result.bind(
            Status.tryFromString(salary.status),
            s => s === "published" ? Result.ok("published") : Result.err("not_published")
        );
        if (Result.isErr(status)) { return Result.err(undefined); }

        const id = Id.tryFromString(salary.id);
        if (Result.isErr(id)) { return Result.err(undefined); }

        const company = Company.tryFromString(salary.company);
        if (Result.isErr(company)) { return Result.err(undefined); }

        const title = tryFromNullable(salary.title, Title.tryFromString);
        if (Result.isErr(title)) { return Result.err(undefined); }

        const location = Location.tryFromString(salary.location);
        if (Result.isErr(location)) { return Result.err(undefined); }

        const compensation = Compensation.tryFromNumber(salary.compensation);
        if (Result.isErr(compensation)) { return Result.err(undefined); }

        const date = SalaryDate.tryFromString(salary.date);
        if (Result.isErr(date)) { return Result.err(undefined); }

        const level = tryFromNullable(salary.level, Level.tryFromString);
        if (Result.isErr(level)) { return Result.err(undefined); }

        const companyXp = tryFromNullable(salary.company_xp, Xp.tryFromNumber);
        if (Result.isErr(companyXp)) { return Result.err(undefined); }

        const totalXp = tryFromNullable(salary.total_xp, Xp.tryFromNumber);
        if (Result.isErr(totalXp)) { return Result.err(undefined); }

        const remoteVariant = Maybe.fromNullable(salary.remote_variant);
        const remoteDayCount = Maybe.fromNullable(salary.remote_day_count);
        const remoteBase = Maybe.fromNullable(salary.remote_base);
        const remoteLocation = Maybe.fromNullable(salary.remote_location);
        const remote: Result<Maybe<Remote>, unknown> = Maybe.match(remoteVariant, {
            onSome: (variant) => Result.map(Remote.tryFromRaw({
                variant,
                dayCount: remoteDayCount,
                base: remoteBase,
                location: remoteLocation
            }), Maybe.some),
            onNone: () => Result.ok(Maybe.none())
        });
        if (Result.isErr(remote)) { return Result.err(undefined); }

        return Result.ok({
            _type: "salary",
            id: Result.unwrap(id),
            company: Result.unwrap(company),
            title: Result.unwrap(title),
            location: Result.unwrap(location),
            compensation: Result.unwrap(compensation),
            level: Result.unwrap(level),
            companyXp: Result.unwrap(companyXp),
            totalXp: Result.unwrap(totalXp),
            remote: Result.unwrap(remote),
            status: Result.unwrap(status),
            date: Result.unwrap(date),
        });

        function tryFromNullable<T, U, E>(
            field: T | null,
            f: (_: T) => Result<U, E>
        ): Result<Maybe<U>, E> {
            return Maybe.match(Maybe.fromNullable(field), {
                onSome: (field) => Result.map(f(field), Maybe.some),
                onNone: () => Result.ok(Maybe.none())
            });
        }
    }
};

type SupabaseStatus = {
    status: string;
};

const SupabaseStatus = {
    fromStatus(status: Status): SupabaseStatus {
        return { status: Status.toString(status) };
    }
};

type SupabaseOrder = {
    key: keyof SupabaseSalary;
    direction: "asc" | "desc";
}

const SupabaseOrder = {
    fromOrder(order: Order<Key>): SupabaseOrder {
        return {
            key: fromKey(order.key),
            direction: order.direction,
        };

        function fromKey(key: Key): keyof SupabaseSalary {
            switch (key) {
                case "company": return "company";
                case "title": return "title";
                case "location": return "location";
                case "compensation": return "compensation";
                case "date": return "date";
                case "level": return "level";
                case "companyXp": return "company_xp";
                case "totalXp": return "total_xp";
                case "remote": return "remote_variant";
            }
        }
    }
}