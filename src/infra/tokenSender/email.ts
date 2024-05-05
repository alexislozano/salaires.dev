import { Email, Token } from "@domain";
import { TokenSender } from "./mod.ts";
import { Env, Result } from "@utils";
import { EmailSender } from "../utils/mod.ts";

export class EmailTokenSender implements TokenSender {
    private sender: EmailSender;
    private appUrl: string;

    private constructor(sender: EmailSender) {
        this.sender = sender;
        this.appUrl = Env.get("APP_URL");
    }

    static new(sender: EmailSender) {
        return new EmailTokenSender(sender);
    }

    send(token: Token, to: Email): Promise<Result<void, string>> {
        return this.sender.send({
            to,
            subject: "Veuillez confirmer votre salaire",
            body: `Confirmez votre salaire en cliquant sur ce lien:<br>${this.appUrl}/?token=${Token.toString(token)}`
        });
    }
}