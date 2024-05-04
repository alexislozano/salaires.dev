import { SmtpClient } from "smtp";
import { Email, Token } from "@domain";
import { TokenSender } from "./mod.ts";
import { Env, Result } from "@utils";

export class EmailTokenSender implements TokenSender {
    private host: string;
    private email: Email;
    private password: string;
    private appUrl: string;

    private constructor(host: string, email: Email, password: string, appUrl: string) {
        this.host = host;
        this.email = email;
        this.password = password;
        this.appUrl = appUrl;
    }

    static new() {
        const email = Email.tryFromString(Env.get("SMTP_EMAIL"));
        if (Result.isErr(email)) { throw new Error("SMTP_EMAIL should be an email address"); }

        return new EmailTokenSender(
            Env.get("SMTP_HOST"),
            Result.unwrap(email),
            Env.get("SMTP_PASSWORD"),
            Env.get("APP_URL")
        );
    }

    async send(token: Token, email: Email): Promise<Result<void, string>> {
        const client = new SmtpClient();
        
        try {
            await client.connectTLS({
                hostname: this.host,
                port: 465,
                username: Email.toString(this.email),
                password: this.password
            });
        } catch (error) {
            console.log(error);
            return Result.err("could not connect via smtp");
        }

        const content = `Confirmez votre salaire en cliquant sur ce lien:<br>${this.appUrl}/?token=${Token.toString(token)}`;

        const message = {
            from: Email.toString(this.email),
            to: Email.toString(email),
            subject: "Veuillez confirmer votre salaire",
            content,
            html: content
        };

        try {
            await client.send(message);
            await client.close();
            return Result.ok(undefined);
        } catch (error) {
            console.log(error);
            return Result.err("could not send email");
        }
    }
}