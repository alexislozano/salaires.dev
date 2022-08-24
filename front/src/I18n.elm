module I18n exposing (..)


type Lang
    = French


type Key
    = Company
    | Location
    | Compensation
    | Stock
    | Level
    | Title
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
    | ThisPageDoesNotExist
    | InsertIsDownForNow
    | Token
    | ShouldBeSixCharactersLong
    | ShouldContainAnAt
    | Email
    | GetAToken
    | IGotAToken
    | LoginBanner
    | EmailSent
    | EmailSendingError
    | SalaryInserted
    | SalaryInstertingError
    | LoadingData
    | InYears
    | InEuros
    | CompensationHelp
    | TitlePlaceholder
    | TheSiteIsInMaintenance
    | Captcha


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
                    "Rémunération brute"

                Stock ->
                    "Stock"

                Level ->
                    "Niveau"

                Title ->
                    "Intitulé du poste"

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

                ThisPageDoesNotExist ->
                    "Cette page n'existe pas"

                InsertIsDownForNow ->
                    "Il n'est plus possible d'ajouter des salaires pour le moment."

                Token ->
                    "Code de confirmation"

                ShouldBeSixCharactersLong ->
                    "Cette valeur doit comporter six caractères"

                ShouldContainAnAt ->
                    "Doit comporter un @"

                Email ->
                    "Email"

                GetAToken ->
                    "Obtenir un code de confirmation"

                IGotAToken ->
                    "J'ai reçu un code de confirmation"

                LoginBanner ->
                    "Entrez une adresse email et recevez un code de confirmation que vous pourrez utiliser dans le formulaire d'ajout de salaire. L'adresse entrée ne sert qu'à recevoir le code, elle n'est pas enregistrée."

                EmailSent ->
                    "Un email a éte envoyé"

                EmailSendingError ->
                    "L'email n'a pas pu être envoyé"

                SalaryInserted ->
                    "Le salaire a été ajouté"

                SalaryInstertingError ->
                    "Le salaire n'a pas pu être ajouté"

                LoadingData ->
                    "Chargement des données..."

                InYears ->
                    "en années"

                InEuros ->
                    "en €"

                CompensationHelp ->
                    "fixe + variable en € / an"

                TheSiteIsInMaintenance ->
                    "Le site est en maintenance"

                TitlePlaceholder ->
                    "Dev fullstack"

                Captcha ->
                    "Captcha"
