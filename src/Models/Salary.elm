module Models.Salary exposing (..)

import Models.Company exposing (Company)
import Models.Compensation exposing (Compensation)
import Models.Date exposing (Date)
import Models.Level exposing (Level)
import Models.Location exposing (Location)
import Models.Stock exposing (Stock)
import Models.Xp as Xp exposing (Xp)


type Salary
    = Salary Fields


type alias Fields =
    { -- required
      company : Company
    , location : Location
    , compensation : Compensation
    , date : Date

    -- optional
    , stock : Maybe Stock
    , level : Maybe Level
    , companyXp : Maybe Xp
    , totalXp : Maybe Xp
    }


toFields : Salary -> Fields
toFields (Salary fields) =
    fields


tryNew : Fields -> Result String Salary
tryNew fields =
    case ( fields.companyXp, fields.totalXp ) of
        ( Just cXp, Just tXp ) ->
            if Xp.toInt cXp > Xp.toInt tXp then
                Err "L'expérience entreprise ne peut pas être plus grande que l'expérience totale"

            else
                Ok (Salary fields)

        _ ->
            Ok (Salary fields)
