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

    get(url: string): Promise<Response> {
        return fetch(`${this.baseUrl}${url}`, {
            method: "GET",
            headers: this.headers()
        });
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
