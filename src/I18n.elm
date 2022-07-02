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
