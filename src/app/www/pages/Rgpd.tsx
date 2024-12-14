import { Maybe } from "../../../utils/maybe.ts";
import { Template } from "./Template.tsx";

type Props = {
    email: string;
    daysBeforeDeletion: string;
};

export function Rgpd(props: Props) {
    return (
        <Template notification={Maybe.none()}>
            <div style={{
                maxWidth: "500px",
                padding: "32px",
                margin: "auto"
            }}>
                <h2 style={{
                    fontSize: "32px",
                    fontWeight: "normal"
                }}>
                    Gestion des données personnelles collectées par salaires.dev
                </h2>
                <h3 style={{
                    fontSize: "24px",
                    fontWeight: "normal"
                }}>
                    Typologie des données
                </h3>
                <p>Adresse email.</p>
                <h3 style={{
                    fontSize: "24px",
                    fontWeight: "normal"
                }}>
                    Politique de partage des données
                </h3>
                <p>Aucune donnée personnelle n'a été partagée ou n'est partagée à des tiers.</p>
                <h3 style={{
                    fontSize: "24px",
                    fontWeight: "normal"
                }}>
                    Cycle de vie des données
                </h3>
                <h4 style={{
                    fontSize: "18px",
                    fontWeight: "normal"
                }}>
                    Collection
                </h4>
                <p>Les données sont collectées lors de la soumission du formulaire d'ajout d'un salaire.</p>
                <h4 style={{
                    fontSize: "18px",
                    fontWeight: "normal"
                }}>
                    Suppression
                </h4>
                <p>Les données sont supprimées lors de la publication du salaire ou au bout de { props.daysBeforeDeletion } jours après collection si non confirmation par la personne ayant soumis le formulaire.</p>
                <h3 style={{
                    fontSize: "24px",
                    fontWeight: "normal"
                }}>
                    Demande de suppression des données
                </h3>
                <p>Si vous voulez faire supprimer vos données personnelles sans attendre la publication ou les { props.daysBeforeDeletion } jours avant suppression automatique, envoyez un email à <a href={`mailto:${props.email}?subject=Demande de suppression de données`}>{ props.email }</a>.</p>
            </div>
        </Template>
    )
}