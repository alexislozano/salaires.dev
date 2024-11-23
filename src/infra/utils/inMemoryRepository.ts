import { Result } from "@utils";

export class InMemoryRepository<Entity> {
    private entities: Entity[];
    private error: boolean;

    constructor(entities: Entity[], error: boolean) {
        this.entities = entities;
        this.error = error;
    }

    insert(entity: Entity) {
        this.entities.push(entity);
    }

    fetchAll(): Promise<Result<Entity[], string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        return Promise.resolve(Result.ok(this.entities));
    }
}