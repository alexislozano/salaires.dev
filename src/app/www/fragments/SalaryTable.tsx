import { Company, Compensation, Direction, Key, Level, Location, Order, PublishedSalary, Remote, RemoteBase, RemoteLocation, SalaryDate, Title, Xp } from "@domain";
import { Maybe } from "@utils";
import { UnreachableCaseError } from "@utils";
import { Column, Table } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = {
    salaries: PublishedSalary[];
    order: Order<Key>;
};

export function SalaryTable(props: Props) {
    const columns = [
        buildColumn("company", props.order),
        buildColumn("title", props.order),
        buildColumn("location", props.order),
        buildColumn("compensation", props.order),
        buildColumn("companyXp", props.order),
        buildColumn("totalXp", props.order),
        buildColumn("level", props.order),
        buildColumn("remote", props.order),
        buildColumn("date", props.order),
    ];
    const extractableSalaries = props.salaries.map(salary => ({
        ...salary,
        extract: (key: Key) => extract(salary, key)
    }));

    return <Table
        items={extractableSalaries}
        columns={columns}
        order={props.order}
    />
}

function buildColumn(key: Key, order: Order<Key>): Column<Key> {
    const { label, subLabel } = buildLabels(key);
    const pushedUrl = `?key=${key}${key === order.key ? `&direction=${Direction.reverse(order.direction)}` : ""}`;
    const sortUrl = `/sort${pushedUrl}`;
    return { key, label, subLabel, sortUrl, pushedUrl };
}

function buildLabels(key: Key): { label: string, subLabel: string } {
    switch (key) {
        case "company": return { label: I18n.translate("company"), subLabel: "" };
        case "title": return { label: I18n.translate("title"), subLabel: "" };
        case "location": return { label: I18n.translate("location"), subLabel: "" };
        case "compensation": return { label: I18n.translate("compensation"), subLabel: I18n.translate("compensation_help") };
        case "date": return { label: I18n.translate("date"), subLabel: "" };
        case "level": return { label: I18n.translate("level"), subLabel: "" };
        case "companyXp": return { label: I18n.translate("company_xp"), subLabel: I18n.translate("in_years") };
        case "totalXp": return { label: I18n.translate("total_xp"), subLabel: I18n.translate("in_years") };
        case "remote": return { label: I18n.translate("remote_terms"), subLabel: "" };
        default: throw new UnreachableCaseError(key);
    }
}

function extract(salary: PublishedSalary, key: Key): string {
    switch (key) {
        case "company": return Company.toString(salary.company);
        case "title": return Maybe.match(salary.title, {
            onSome: (title) => Title.toString(title),
            onNone: () => ""
        });
        case "location": return Location.toString(salary.location);
        case "compensation": return Compensation.toString(salary.compensation);
        case "date": return SalaryDate.toString(salary.date, "yyyy-MM-dd");
        case "level": return Maybe.match(salary.level, {
            onSome: (level) => extractLevel(level),
            onNone: () => ""
        });
        case "companyXp": return Maybe.match(salary.companyXp, {
            onSome: (companyXp) => Xp.toString(companyXp),
            onNone: () => ""
        });
        case "totalXp": return Maybe.match(salary.totalXp, {
            onSome: (totalXp) => Xp.toString(totalXp),
            onNone: () => ""
        });
        case "remote": return Maybe.match(salary.remote, {
            onSome: (remote) => extractRemote(remote),
            onNone: () => ""
        });
        default: throw new UnreachableCaseError(key);
    };
}

function extractLevel(level: Level): string {
    switch (level) {
        case "junior": return I18n.translate("junior");
        case "mid": return I18n.translate("mid");
        case "senior": return I18n.translate("senior");
        default: throw new UnreachableCaseError(level);
    }
}

function extractRemote(remote: Remote): string {
    const variant = remote.variant;
    switch (variant) {
        case "none": return I18n.translate("no_remote");
        case "full": return I18n.translate("full_remote");
        case "partial": return `${ remote.dayCount } ${ I18n.translate("days_per") } ${ extractBase(remote.base) } ${ I18n.translate("in") } ${ extractLocation(remote.location) }`
        default: throw new UnreachableCaseError(variant);
    }

    function extractBase(base: RemoteBase): string {
        switch (base) {
            case "week": return I18n.translate("week");
            case "month": return I18n.translate("month");
            default: throw new UnreachableCaseError(base);
        }
    }

    function extractLocation(location: RemoteLocation): string {
        switch (location) {
            case "office": return I18n.translate("office");
            case "remote": return I18n.translate("remote");
            default: throw new UnreachableCaseError(location);
        }
    }
}