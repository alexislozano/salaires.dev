export class InMemoryRepository<T> {
    private entities: T[];
    private error: boolean;

    private constructor(entities: T[], error: boolean) {
        this.entities = entities;
        this.error = error;
    }
}