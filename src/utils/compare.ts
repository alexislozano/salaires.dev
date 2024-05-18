export const Compare = {
    asc(a: number, b: number): number {
        return a === b
            ? 0
            : (a > b ? 1 : -1);
    },
    desc(a: number, b: number): number {
        return a === b
            ? 0
            : (a < b ? 1 : -1);
    },
};
