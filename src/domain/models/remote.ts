import { Compare, Maybe, Result, String, UnreachableCaseError } from "@utils";
import { Direction } from "@domain";

export type Remote =
    | { variant: "none" }
    | { variant: "full" }
    | {
        variant: "partial";
        dayCount: RemoteDayCount;
        base: RemoteBase;
        location: RemoteLocation;
    }
    ;

export type RemoteError =
    | "not_a_remote_variant"
    | "remote_day_count_is_negative"
    | "remote_day_count_is_not_a_number"
    | "remote_day_count_is_not_an_integer"
    | "remote_base_is_not_a_week_or_a_month"
    | "remote_day_count_should_be_between_1_and_7"
    | "remote_day_count_should_be_between_1_and_31"
    | "remote_location_is_not_remote_or_office"
    ;

export const Remote = {
    compare(a: Remote, b: Remote, direction: Direction): number {
        const aDays = convertToRemoteDaysPerMonth(a);
        const bDays = convertToRemoteDaysPerMonth(b);
        switch (direction) {
            case "desc": return Compare.desc(aDays, bDays);
            case "asc": return Compare.asc(aDays, bDays);
            default: throw new UnreachableCaseError(direction);
        }

        function convertToRemoteDaysPerMonth(remote: Remote): number {
            const variant = remote.variant;
            switch (variant) {
                case "none": return 0;
                case "full": return 31;
                case "partial": {
                    const { dayCount, base, location } = remote;
                    if (base === "month") {
                        if (location === "remote") { return dayCount; }
                        if (location === "office") { return 31 - dayCount; }
                        throw new UnreachableCaseError(location);
                    }
                    if (base === "week") {
                        if (location === "remote") { return dayCount * 4; }
                        if (location === "office") { return 31 - dayCount * 4; }
                        throw new UnreachableCaseError(location);
                    }
                    throw new UnreachableCaseError(base);
                }
            }
        }
    },
    toRaw(remote: Remote): { variant: string, dayCount: Maybe<number>, base: Maybe<string>, location: Maybe<string> } {
        return {
            variant: remote.variant,
            dayCount: remote.variant === "partial" ? Maybe.some(remote.dayCount) : Maybe.none(),
            base: remote.variant === "partial" ? Maybe.some(remote.base) : Maybe.none(),
            location: remote.variant === "partial" ? Maybe.some(remote.location) : Maybe.none(),
        };
    },
    tryFromRaw({ variant, dayCount: maybeDayCount, base: maybeBase, location: maybeLocation }: {
        variant: string;
        dayCount: Maybe<number>;
        base: Maybe<string>;
        location: Maybe<string>
    }): Result<Remote, RemoteError> {
        switch (variant) {
            case "none":
            case "full": return Result.ok({ variant });
            case "partial": {
                if (Maybe.isNone(maybeDayCount)) { return Result.err("not_a_remote_variant"); }
                if (Maybe.isNone(maybeBase)) { return Result.err("not_a_remote_variant"); }
                if (Maybe.isNone(maybeLocation)) { return Result.err("not_a_remote_variant"); }

                const parsedDayCount = RemoteDayCount.tryFromNumber(Maybe.unwrap(maybeDayCount));
                if (Result.isErr(parsedDayCount)) { return parsedDayCount; }

                const parsedBase = RemoteBase.tryFromString(Maybe.unwrap(maybeBase));
                if (Result.isErr(parsedBase)) { return parsedBase; }

                const parsedLocation = RemoteLocation.tryFromString(Maybe.unwrap(maybeLocation));
                if (Result.isErr(parsedLocation)) { return parsedLocation; }

                const dayCount = Result.unwrap(parsedDayCount);
                const base = Result.unwrap(parsedBase);
                const validatedDayCount = RemoteDayCount.validate(dayCount, base);
                if (Result.isErr(validatedDayCount)) { return validatedDayCount; }

                return Result.ok({
                    variant,
                    dayCount,
                    base,
                    location: Result.unwrap(parsedLocation)
                });
            }
            default: return Result.err("not_a_remote_variant");
        }
    },
    tryFromForm({ variant, dayCount: dayCountString, base: baseString, location: locationString }: {
        variant: string;
        dayCount: string;
        base: string;
        location: string;
    }): Result<Remote, RemoteError> {
        switch (variant) {
            case "none":
            case "full": return Result.ok({ variant });
            case "partial": {
                const parsedDayCount = RemoteDayCount.tryFromString(dayCountString);
                if (Result.isErr(parsedDayCount)) { return parsedDayCount; }

                const parsedBase = RemoteBase.tryFromString(baseString);
                if (Result.isErr(parsedBase)) { return parsedBase; }

                const parsedLocation = RemoteLocation.tryFromString(locationString);
                if (Result.isErr(parsedLocation)) { return parsedLocation; }

                const dayCount = Result.unwrap(parsedDayCount);
                const base = Result.unwrap(parsedBase);
                const validatedDayCount = RemoteDayCount.validate(dayCount, base);
                if (Result.isErr(validatedDayCount)) { return validatedDayCount; }

                return Result.ok({
                    variant,
                    dayCount,
                    base,
                    location: Result.unwrap(parsedLocation)
                });
            }
            default: return Result.err("not_a_remote_variant");
        }
    }
};

type RemoteDayCount = number;

const RemoteDayCount = {
    tryFromNumber(dayCountNumber: number): Result<RemoteDayCount, RemoteError> {
        if (dayCountNumber < 0) { return Result.err("remote_day_count_is_negative"); }
        if (! Number.isInteger(dayCountNumber)) { return Result.err("remote_day_count_is_not_an_integer"); }
        return Result.ok(dayCountNumber);
    },
    tryFromString(dayCountString: string): Result<RemoteDayCount, RemoteError> {
        const parsedDayCount = String.tryToNumber(dayCountString);
        if (Result.isErr(parsedDayCount)) { return Result.err("remote_day_count_is_not_a_number"); }
        return RemoteDayCount.tryFromNumber(Result.unwrap(parsedDayCount));
    },
    validate(dayCount: RemoteDayCount, base: "week" | "month"): Result<void, RemoteError> {
        switch (base) {
            case "week": return dayCount >= 1 && dayCount <= 7
                ? Result.ok(undefined)
                : Result.err("remote_day_count_should_be_between_1_and_7");
            case "month": return dayCount >= 1 && dayCount <= 31
                ? Result.ok(undefined)
                : Result.err("remote_day_count_should_be_between_1_and_31");
            default: throw new UnreachableCaseError(base);
        }
    }
};

export type RemoteBase = "week" | "month";

const RemoteBase = {
    tryFromString(baseString: string): Result<RemoteBase, RemoteError> {
        switch (baseString) {
            case "week":
            case "month": return Result.ok(baseString);
            default: return Result.err("remote_base_is_not_a_week_or_a_month");
        };
    }
};

export type RemoteLocation = "remote" | "office";

const RemoteLocation = {
    tryFromString(locationString: string): Result<RemoteLocation, RemoteError> {
        switch (locationString) {
            case "remote":
            case "office": return Result.ok(locationString);
            default: return Result.err("remote_location_is_not_remote_or_office");
        };
    }
};
