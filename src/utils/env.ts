import * as dotenv from "dotenv";

const ENV_KEYS = [
    "MAINTENANCE",
    "NO_INSERT",
    "PORT",
    "APP_URL",
    "SUPABASE_URL",
    "SUPABASE_KEY",
    "HCAPTCHA_KEY",
    "HCAPTCHA_SECRET",
    "SMTP_HOST",
    "SMTP_EMAIL",
    "SMTP_PASSWORD",
    "ADMIN_EMAIL",
    "RGPD_EMAIL",
    "RGPD_DAYS_BEFORE_DELETION"
] as const;

type EnvKey = typeof ENV_KEYS[number];

export const Env = {
    get(key: EnvKey): string {
        const variable = Deno.env.get(key);
        if (variable) { return variable; }

        throw new Error(`${key} env var does not exist`);
    },
    init() {
        dotenv.loadSync({ export: true });

        ENV_KEYS.forEach(Env.get);
    }
};
