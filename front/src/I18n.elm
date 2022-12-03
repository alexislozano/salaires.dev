module I18n exposing (..)


type Lang
    = French


type Key
    = Company
    | Location
    | Compensation
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
    | SalaryInserted
    | SalaryInstertingError
    | LoadingData
    | InYears
    | InEuros
    | CompensationHelp
    | TitlePlaceholder
    | TheSiteIsInMaintenance
    | EmailShouldContainAnAt
    | Email
    | EmailShouldBePro
    | TokenConfirmationSuccess
    | TokenConfirmationError
    | EmailExplanation


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

                SalaryInserted ->
                    "Le salaire a été ajouté, un email de confirmation a été envoyé"

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

                EmailShouldContainAnAt ->
                    "Une adresse email doit comporter un @"

                Email ->
                    "Email"

                EmailShouldBePro ->
                    "L'adresse email doit être professionnelle"

                TokenConfirmationSuccess ->
                    "Le salaire a bien été confirmé, il sera publié prochainement"

                TokenConfirmationError ->
                    "Le salaire n'a pas pu être confirmé"

                EmailExplanation ->
                    "L'adresse email et l'entreprise renseignées doivent correspondre. Sans cela, le salaire ne sera pas publié."
