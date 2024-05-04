import { Key, Order, Salary } from "@domain";
import { Maybe } from "@utils";
import { Template } from "./Template.tsx";
import { SalaryTable } from "../fragments/mod.ts";

type Props = {
    salaries: Salary[],
    order: Order<Key>,
    notification: Maybe<string>
};

export function Home(props: Props) {
    return (
        <Template notification={props.notification}>
            <SalaryTable
                salaries={props.salaries}
                order={props.order}
            />
        </Template>
    )
}