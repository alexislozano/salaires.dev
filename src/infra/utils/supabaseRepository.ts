import { z } from "zod";
import { Env } from "@utils";
import { Result } from "@utils";

export class SupabaseRepository {
    private baseUrl: string;
    private key: string;

    private constructor() {
        this.baseUrl = Env.get("SUPABASE_URL");
        this.key = Env.get("SUPABASE_KEY");
    }

    private headers(): Record<string, string> {
        return {
            apiKey: this.key,
            Authorization: `Bearer ${this.key}`,
            "Content-Type": "application/json",
        };
    }

    static new() {
        return new SupabaseRepository();
    }

    delete(url: string): Promise<Response> {
        return fetch(`${this.baseUrl}${url}`, {
            method: "DELETE",
            headers: this.headers()
        });
    }

    private async fetch<Entity, Schema extends z.ZodTypeAny>({ url, schema, service }: {
        url: string;
        schema: Schema;
        service: string;
    }): Promise<Result<Schema["_output"][], string>> {
        const response = await fetch(`${this.baseUrl}${url}`, {
            method: "GET",
            headers: this.headers()
        });
        if (! response.ok) { return Result.err(`${service}: could not send request`); }
        
        const supabaseEntities = z
            .array(schema)
            .safeParse(await response.json());
        if (! supabaseEntities.success) { return Result.err(`${service}: could not parse json`); }

        return Result.ok(supabaseEntities.data);
    }

    async fetchAll<Entity, EntityError, Schema extends z.ZodTypeAny>({ url, schema, convert, service }: {
        url: string;
        schema: Schema;
        convert: (_: z.infer<Schema>) => Result<Entity, EntityError>;
        service: string;
    }): Promise<Result<Entity[], string>> {
        const supabaseEntities = await this.fetch({ url, schema, service });
        if (Result.isErr(supabaseEntities)) { return supabaseEntities; }

        const entities: Entity[] = [];
        for (const supabaseEntity of Result.unwrap(supabaseEntities)) {
            const entity = convert(supabaseEntity);
            if (Result.isErr(entity)) { return Result.err(`${service}: could not convert to domain`); }
            entities.push(Result.unwrap(entity));
        }

        return Result.ok(entities);
    }

    async fetchOne<Entity, EntityError, Schema extends z.ZodTypeAny>({ url, schema, convert, service }: {
        url: string;
        schema: Schema;
        convert: (_: z.infer<Schema>) => Result<Entity, EntityError>;
        service: string;
    }): Promise<Result<Entity, string>> {
        const supabaseEntities = await this.fetch({ url, schema, service });
        if (Result.isErr(supabaseEntities)) { return supabaseEntities; }

        const supabaseEntity = Result.unwrap(supabaseEntities).pop();
        if (! supabaseEntity) { return Result.err(`${service}: could not extract first element`); }

        const entity = convert(supabaseEntity);
        if (Result.isErr(entity)) { return Result.err(`${service}: could not convert to domain`); }
            
        return entity;
    }

    patch(url: string, body: unknown): Promise<Response> {
        return fetch(`${this.baseUrl}${url}`, {
            method: "PATCH",
            headers: this.headers(),
            body: JSON.stringify(body)
        });
    }

    async insert({ url, body, service }: {
        url: string;
        body: unknown;
        service: unknown;
    }): Promise<Result<void, string>> {
        const response = await fetch(`${this.baseUrl}${url}`, {
            method: "POST",
            headers: this.headers(),
            body: JSON.stringify(body)
        });
        if (! response.ok) { return Result.err(`${service}: could not send request`); }
        return Result.ok(undefined);
    }
}
