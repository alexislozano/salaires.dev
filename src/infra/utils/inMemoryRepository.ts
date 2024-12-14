import { Result } from "@utils";

export class InMemoryRepository<Entity> {
    private entities: Entity[];
    private error: boolean;

    constructor(entities: Entity[], error: boolean) {
        this.entities = entities;
        this.error = error;
    }

    insert(entity: Entity): Promise<Result<void, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        this.entities.push(entity);

        return Promise.resolve(Result.ok(undefined));
    }

    fetchAll(): Promise<Result<Entity[], string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        return Promise.resolve(Result.ok(this.entities));
    }

    patch<T extends Entity>({ filter, patch }: {
        filter: (t: Entity) => t is T;
        patch: (_: T) => Entity;
    }): Promise<Result<undefined, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        const index = this.entities.findIndex(filter);

        if (index == -1) {
            return Promise.resolve(Result.err("salary not found"));
        }

        this.entities[index] = patch(this.entities[index] as T);

        return Promise.resolve(Result.ok(undefined));
    }

    delete({ filter }: {
        filter: (_: Entity) => boolean
    }): Promise<Result<Entity, string>> {
        if (this.error) {
            return Promise.resolve(Result.err("error flag is on"));
        }

        const index = this.entities.findIndex(filter);

        if (index == -1) {
            return Promise.resolve(Result.err("salary not found"));
        }

        const [entity] = this.entities.splice(index, 1);

        return Promise.resolve(Result.ok(entity))
    }
}