import * as app from "@app";
import * as cron from "@cron";
import { EmailAdminNotifier, EmailSender, SupabaseCompanyRepository, SupabaseLocationRepository, SupabaseRepository, SupabaseSalaryRepository, SupabaseTitleRepository, SupabaseTokenRepository, HCaptchaService, EmailTokenSender, EmailRgpdNotifier } from "@infra";
import { Env } from "@utils";

Env.init();

const emailSender = EmailSender.new();
const supabaseRepo = SupabaseRepository.new();

const salaryRepo = SupabaseSalaryRepository.new(supabaseRepo);
const companyRepo = SupabaseCompanyRepository.new(supabaseRepo);
const locationRepo = SupabaseLocationRepository.new(supabaseRepo);
const titleRepo = SupabaseTitleRepository.new(supabaseRepo)
const captchaService = HCaptchaService.new();
const tokenRepo = SupabaseTokenRepository.new(supabaseRepo);
const tokenSender = EmailTokenSender.new(emailSender);
const adminNotifier = EmailAdminNotifier.new(emailSender);
const rgpdNotifier = EmailRgpdNotifier.new(emailSender);

app.serve(
    salaryRepo,
    companyRepo,
    locationRepo,
    titleRepo,
    captchaService,
    tokenRepo,
    tokenSender,
    adminNotifier
);

Deno.cron("Send RGPD email", { hour: { every: 24 } }, async () => {
    await cron.sendRgpdEmail(salaryRepo, rgpdNotifier);
});