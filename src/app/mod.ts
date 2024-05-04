import { Context, Hono } from "hono";
import { serveStatic } from "hono/middleware";
import { CompanyRepository, LocationRepository, SalaryRepository, TitleRepository, TokenRepository, CaptchaService, TokenSender } from "@infra";
import { Env } from "@utils";
import * as home from "./www/controllers/home.ts";
import * as sort from "./www/controllers/sort.ts";
import * as notification from "./www/controllers/notification.ts";
import * as notFound from "./www/controllers/notFound.ts";
import * as insert from "./www/controllers/insert.ts";
import * as validate from "./www/controllers/validate.ts";
import * as maintenance from "./www/controllers/maintenance.ts";
import * as noInsert from "./www/controllers/noInsert.ts";
import * as api from "./api/mod.ts";

export function serve(
    salaryRepo: SalaryRepository,
    companyRepo: CompanyRepository,
    locationRepo: LocationRepository,
    titleRepo: TitleRepository,
    captchaService: CaptchaService,
    tokenRepo: TokenRepository,
    tokenSender: TokenSender
) {
    const app = new Hono();

    if (Env.get("MAINTENANCE") === "true") {
        app.all("*", (c: Context) => maintenance.get(c));
    } else {
        app.get("/", (c: Context) => home.get(c, salaryRepo, tokenRepo));
        app.get("/sort", (c: Context) => sort.get(c, salaryRepo));
        app.delete("/notification", (c: Context) => notification.del(c));
        app.get("/api/salaries", (c: Context) => api.fetchSalaries(c, salaryRepo));
        app.get("/api/companies", (c: Context) => api.fetchCompanies(c, companyRepo));
        app.get("/api/locations", (c: Context) => api.fetchLocations(c, locationRepo));
        app.get("/api/titles", (c: Context) => api.fetchTitles(c, titleRepo));
        app.use('/assets/hero.png', serveStatic({ path: "./assets/hero.png" }))

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
