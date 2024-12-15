import { Context, Hono } from "hono";
import { serveStatic } from "hono/deno";
import { AdminNotifier, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository, TokenRepository, CaptchaService, TokenSender } from "@infra";
import { Env } from "@utils";
import { home, insert, maintenance, noInsert, notFound, notification, rgpd, sort, validate } from "./www/mod.ts";
import * as api from "./api/mod.ts";

export function serve(
    salaryRepo: SalaryRepository,
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository,
    captchaService: CaptchaService,
    tokenRepo: TokenRepository,
    tokenSender: TokenSender,
    adminNotifier: AdminNotifier
) {
    const app = new Hono();

    if (Env.get("MAINTENANCE") === "true") {
        app.all("*", (c: Context) => maintenance.get(c));
    } else {
        app.get("/", (c: Context) => home.get(c, salaryRepo, tokenRepo, adminNotifier));
        app.get("/sort", (c: Context) => sort.get(c, salaryRepo));
        app.get("/rgpd", (c: Context) => rgpd.get(c));
        app.delete("/notification", (c: Context) => notification.del(c));
        app.get("/api/salaries", (c: Context) => api.fetchSalaries(c, salaryRepo));
        app.get('/assets/hero.png', serveStatic({ path: "./assets/hero.png" }));

        if (Env.get("NO_INSERT") === "true") {
            app.get("/insert", (c: Context) => noInsert.get(c));
        } else {
            app.get("/insert", (c: Context) => insert.get(c, companyRepo, locationRepo, titleRepo));
            app.post("/insert", (c: Context) => insert.post(c, companyRepo, locationRepo, titleRepo, salaryRepo, tokenRepo, captchaService, tokenSender));
            app.post("/validate", (c: Context) => validate.post(c, companyRepo, locationRepo, titleRepo));    
        }

        app.all("*", (c: Context) => notFound.get(c));
    }
    
    Deno.serve({ port: Number(Env.get("PORT")) }, app.fetch);
}
