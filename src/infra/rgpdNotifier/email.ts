import { Email } from "@domain";
import { RgpdNotifier } from "./mod.ts";
import { Env, Result } from "@utils";
import { EmailSender } from "../utils/mod.ts";

export class EmailRgpdNotifier implements RgpdNotifier {
    private sender: EmailSender;
    private rgpdEmail: Email;

    private constructor(sender: EmailSender) {
        const rgpdEmail = Email.tryFromString(Env.get("RGPD_EMAIL"), { admin: true });
        if (Result.isErr(rgpdEmail)) { throw new Error("RGPD_EMAIL should be an email address"); }

        this.sender = sender;
        this.rgpdEmail = Result.unwrap(rgpdEmail);
    }

    static new(sender: EmailSender) {
        return new EmailRgpdNotifier(sender);
    }

    notify(): Promise<Result<void, string>> {
        return this.sender.send({
            to: this.rgpdEmail,
            subject: "Notification de salaires.dev",
            body: "Un salaire doit être supprimé"
        });
    }
}