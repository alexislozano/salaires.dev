import * as app from "@app";
import { SupabaseCompanyRepository, SupabaseLocationRepository, SupabaseRepository, SupabaseSalaryRepository, SupabaseTitleRepository, SupabaseTokenRepository, HCaptchaService, EmailTokenSender } from "@infra";
import { Env } from "@utils";

Env.init();

const supabaseRepo = SupabaseRepository.new();

const salaryRepo = SupabaseSalaryRepository.new(supabaseRepo);
const companyRepo = SupabaseCompanyRepository.new(supabaseRepo);
const locationRepo = SupabaseLocationRepository.new(supabaseRepo);
const titleRepo = SupabaseTitleRepository.new(supabaseRepo)
const captchaService = HCaptchaService.new();
const tokenRepo = SupabaseTokenRepository.new(supabaseRepo);
const tokenSender = EmailTokenSender.new();

app.serve(
    salaryRepo,
    companyRepo,
    locationRepo,
    titleRepo,
    captchaService,
    tokenRepo,
    tokenSender
);
