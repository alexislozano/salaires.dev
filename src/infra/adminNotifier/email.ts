import { Email } from "@domain";
import { AdminNotifier } from "./mod.ts";
import { Env, Result } from "@utils";
import { EmailSender } from "../utils/mod.ts";

export class EmailAdminNotifier implements AdminNotifier {
    private sender: EmailSender;
    private adminEmail: Email;

    private constructor(sender: EmailSender) {
        const adminEmail = Email.tryFromString(Env.get("ADMIN_EMAIL"), { admin: true });
        if (Result.isErr(adminEmail)) { throw new Error("ADMIN_EMAIL should be an email address"); }

        this.sender = sender;
        this.adminEmail = Result.unwrap(adminEmail);
    }

    static new(sender: EmailSender) {
        return new EmailAdminNotifier(sender);
    }

    notify(): Promise<Result<void, string>> {
        return this.sender.send({
            to: this.adminEmail,
            subject: "Notification de salaires.dev",
            body: "Un salaire vient d'être confirmé"
        });
    }
}