import { SmtpClient } from "smtp";
import { Email } from "@domain";
import { Env, Result } from "@utils";

export class EmailSender {
    private host: string;
    private email: Email;
    private password: string;

    private constructor() {
        const email = Email.tryFromString(Env.get("SMTP_EMAIL"), { admin: true });
        if (Result.isErr(email)) { throw new Error("SMTP_EMAIL should be an email address"); }

        this.host = Env.get("SMTP_HOST");
        this.email = Result.unwrap(email);
        this.password = Env.get("SMTP_PASSWORD");
    }

    static new() {
        return new EmailSender();
    }

    async send({ to, subject, body }: {
        to: Email;
        subject: string;
        body: string;
    }): Promise<Result<void, string>> {
        const client = new SmtpClient();

        try {
            await client.connectTLS({
                hostname: this.host,
                port: 465,
                username: Email.toString(this.email),
                password: this.password
            });
        } catch {
            return Result.err("could not connect via smtp");
        }

        try {
            await client.send({
                from: Email.toString(this.email),
                to: Email.toString(to),
                subject,
                content: body,
                html: body
            });
            return Result.ok(undefined);
        } catch (e: unknown) {
            console.log(e);
            return Result.err("could not send email");
        } finally {
            await client.close();
        }
    }
}
