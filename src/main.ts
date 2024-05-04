import * as app from "@app";
import { SupabaseCompanyRepository, SupabaseLocationRepository, SupabaseSalaryRepository, SupabaseTitleRepository, SupabaseTokenRepository, HCaptchaService, EmailTokenSender } from "@infra";
import { Env } from "@utils";

Env.init();

const salaryRepo = SupabaseSalaryRepository.new();
const companyRepo = SupabaseCompanyRepository.new();
const locationRepo = SupabaseLocationRepository.new();
const titleRepo = SupabaseTitleRepository.new()
const captchaService = HCaptchaService.new();
const tokenRepo = SupabaseTokenRepository.new();
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
