import { Maybe } from "../../utils/maybe.ts";
import { Company } from "./company.ts";
import { Compensation } from "./compensation.ts";
import { Email } from "./email.ts";
import { Id } from "./id.ts";
import { Level } from "./level.ts";
import { Location } from "./location.ts";
import { Status } from "./status.ts";
import { Title } from "./title.ts";
import { Xp } from "./xp.ts";

export type Salary = {
    id: Id;
    email: Email;
    company: Company;
    title: Title;
    location: Location;
    compensation: Compensation;
    date: Date,
    level: Maybe<Level>;
    companyXp: Maybe<Xp>;
    totalXp: Maybe<Xp>;
    status: Status;
};

export type Key = keyof Omit<Salary, "id" | "email" | "status">;

export const Salary = {
    confirm(salary: Salary): Salary {
        return { ...salary, status: "confirmed" };
    }
};
