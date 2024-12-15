import { Maybe, UnreachableCaseError } from "@utils";
import { Company } from "./company.ts";
import { Compensation } from "./compensation.ts";
import { SalaryDate } from "./date.ts";
import { Email } from "./email.ts";
import { Id } from "./id.ts";
import { Level } from "./level.ts";
import { Location } from "./location.ts";
import { Remote } from "./remote.ts";
import { Title } from "./title.ts";
import { Xp } from "./xp.ts";
import { Order } from "@domain";

type SalaryBase = {
    id: Id;
    company: Company;
    title: Maybe<Title>;
    location: Location;
    compensation: Compensation;
    date: SalaryDate;
    level: Maybe<Level>;
    companyXp: Maybe<Xp>;
    totalXp: Maybe<Xp>;
    remote: Maybe<Remote>;
};

export type WaitingSalary = SalaryBase & {
    email: Email;
    status: "waiting";
};

export type ConfirmedSalary = SalaryBase & {
    email: Email;
    status: "confirmed";
};

export type PublishedSalary = SalaryBase & {
    status: "published";
}

export type Salary = WaitingSalary | ConfirmedSalary | PublishedSalary;

export const Salary = {
    compare(a: Salary, b: Salary, order: Order<Key>): number {
        switch (order.key) {
            case "company": return Company.compare(a.company, b.company, order.direction);
            case "title": return Maybe.compare(a.title, b.title, order.direction, Title.compare);
            case "location": return Location.compare(a.location, b.location, order.direction);
            case "compensation": return Compensation.compare(a.compensation, b.compensation, order.direction);
            case "date": return SalaryDate.compare(a.date, b.date, order.direction);
            case "level": return Maybe.compare(a.level, b.level, order.direction, Level.compare);
            case "companyXp": return Maybe.compare(a.companyXp, b.companyXp, order.direction, Xp.compare);
            case "totalXp": return Maybe.compare(a.totalXp, b.totalXp, order.direction, Xp.compare);
            case "remote": return Maybe.compare(a.remote, b.remote, order.direction, Remote.compare);
            default: throw new UnreachableCaseError(order.key);
        };
    },
    confirm(salary: WaitingSalary): ConfirmedSalary {
        return { ...salary, status: "confirmed" };
    },
    generate(overrides?: Partial<WaitingSalary>): WaitingSalary {
        return {
            id: Id.generate(),
            email: Email.generate(),
            company: Company.generate(),
            title: Maybe.none(),
            location: Location.generate(),
            compensation: Compensation.generate(),
            date: SalaryDate.generate(),
            level: Maybe.none(),
            companyXp: Maybe.none(),
            totalXp: Maybe.none(),
            remote: Maybe.none(),
            status: "waiting",
            ...overrides
        };
    },
    isConfirmed(salary: Salary): salary is ConfirmedSalary {
        return salary.status === "confirmed"
    },
    isPublished(salary: Salary): salary is PublishedSalary {
        return salary.status === "published"
    },
    isWaiting(salary: Salary): salary is WaitingSalary {
        return salary.status === "waiting"
    },
    publish(salary: ConfirmedSalary): PublishedSalary {
        return { ...salary, status: "published" };
    },
};

export type Key = keyof Omit<Salary, "id" | "email" | "status">;

export const Key = {
    default(): Key {
        return "date";
    },
    fromString(key: string): Key {
        switch (key.trim()) {
            case "company": return "company";
            case "title": return "title";
            case "location": return "location";
            case "compensation": return "compensation";
            case "date": return "date";
            case "level": return "level";
            case "companyXp": return "companyXp";
            case "totalXp": return "totalXp";
            case "remote": return "remote";
            default: return Key.default();
        }
    }
}