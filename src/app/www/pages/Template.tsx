import { html } from "hono/html";
import { PropsWithChildren } from "hono/jsx";
import { Env, Maybe } from "@utils";
import { BLACK, PEACH, SAND } from "../components/palette.ts";
import { Notification } from "../components/mod.ts";
import { ButtonLink } from "../components/mod.ts";
import { I18n } from "../i18n.ts";

type Props = PropsWithChildren<{
    notification: Maybe<string>;
}>;

export function Template(props: Props) {
    return (
        <>
            {html`<!DOCTYPE html>`}
            <html>
                <Head />
                <Body notification={props.notification}>
                    { props.children }
                </Body>  
            </html>
        </>
    );
}

function Head() {
    const appUrl = Env.get("APP_URL");
    return (
        <head>
            <meta charSet="utf-8" />
            <meta httpEquiv="X-UA-Compatible" content="IE-edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>salaires.dev</title>
            <meta name="description" content="Partagez et comparez votre salaire anonymement" />
            <link rel="canonical" href={appUrl} />
            { /* twitter meta */ }
            <meta property="twitter:card" content="summary" />
            <meta property="twitter:title" content="salaires.dev" />
            <meta property="twitter:description" content="Partagez et comparez votre salaire anonymement" />
            <meta property="twitter:image" content={`${appUrl}/assets/hero.png`} />
            <meta property="twitter:image-alt" content="Capture d'écran de salaires.dev" />
            { /* facebook / linkedin meta */ }
            <meta property="og:type" content="website" />
            <meta property="og:title" content="salaires.dev" />
            <meta property="og:description" content="Partagez et comparez votre salaire anonymement" />
            <meta property="og:image" content={`${appUrl}/assets/hero.png`} />
            <meta property="og:image:secure_url" content={`${appUrl}/assets/hero.png`} />
            <meta property="og:image:alt" content="Capture d'écran de salaires.dev" />
            <meta property="og:url" content={appUrl} />
        </head>
    )
}

function Body(props: Props) {
    return (
        <body style={{
            backgroundColor: SAND,
            fontFamily: "Open Sans, Helvetica, Verdana, sans-serif",
            fontSize: "14px",
            margin: 0,
            height: "100vh",
            display: "flex",
            flexDirection: "column"
        }}>
            <Header />
            <main style={{
                overflow: "auto",
                flexGrow: 1
            }}>
                <Notification notification={props.notification} />
                { props.children }
            </main>
            <script src="https://unpkg.com/htmx.org@1.9.11"></script>
            <script src="https://hcaptcha.com/1/api.js" async defer></script>
        </body>
    )
}

function Header() {
    return (
        <nav style={{
            backgroundColor: PEACH,
            borderBottom: `2px solid ${BLACK}`,
            display: "flex",
            gap: "8px",
            justifyContent: "space-between",
            flexWrap: "wrap",
            padding: "8px"
        }}>
            <div style={{
                display: "flex",
                gap: "8px"
            }}>
                <ButtonLink label="salaires.dev" url="/" />
                <ButtonLink label="RGPD" url="/rgpd" />
                <ButtonLink label="API" url="/api/salaries" />
                <ButtonLink label="Github" url="https://github.com/alexislozano/salaires.dev" />
            </div>
            <ButtonLink label={I18n.translate("i_add_my_salary")} url="/insert" />
        </nav>
    )
}