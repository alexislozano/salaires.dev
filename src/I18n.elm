module I18n exposing (..)


type Lang
    = French


type Key
    = Company
    | Location
    | Compensation
    | Stock
    | Level
    | CompanyXp
    | TotalXp
    | Date
    | IAddMySalary
    | ShouldNotBeEmpty
    | ShouldBeANumber
    | ShouldBePositive
    | Optional
    | Send
    | Sending
    | Error
    | Sent
    | ThisPageDoesNotExist
    | InsertIsDownForNow


translate : Lang -> Key -> String
translate lang key =
    case lang of
        French ->
            case key of
                Company ->
                    "Entreprise"

                Location ->
                    "Localisation"

                Compensation ->
                    "Compensation"

                Stock ->
                    "Stock"

                Level ->
                    "Niveau"

                CompanyXp ->
                    "Expérience entreprise"

                TotalXp ->
                    "Expérience totale"

                Date ->
                    "Date d'ajout"

                IAddMySalary ->
                    "J'ajoute mon salaire"

                ShouldNotBeEmpty ->
                    "Ce champ ne peut pas être vide"

                ShouldBeANumber ->
                    "Cette valeur doit être un nombre"

                ShouldBePositive ->
                    "Ce nombre doit être positif"

                Optional ->
                    "Optionel"

                Send ->
                    "Envoyer"

                Sending ->
                    "En cours d'envoi..."

                Sent ->
                    "Envoyé"

                Error ->
                    "Erreur"

                ThisPageDoesNotExist ->
                    "Cette page n'existe pas"

                InsertIsDownForNow ->
                    "Il n'est plus possible d'ajouter des salaires pour le moment."
