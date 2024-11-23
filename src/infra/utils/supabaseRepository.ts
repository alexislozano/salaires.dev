import { Env } from "@utils";

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

    post(url: string, body: unknown): Promise<Response> {
        return fetch(`${this.baseUrl}${url}`, {
            method: "POST",
            headers: this.headers(),
            body: JSON.stringify(body)
        });
    }
}
