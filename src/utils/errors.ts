export class UnreachableCaseError extends Error {
    constructor(e: never) {
        super(e);
    }
}