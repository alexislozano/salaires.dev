module Models.Salary exposing (..)

import Models.Company exposing (Company)
import Models.Compensation exposing (Compensation)
import Models.Date exposing (Date)
import Models.Level exposing (Level)
import Models.Location exposing (Location)
import Models.Stock exposing (Stock)
import Models.Xp exposing (Xp)


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
    Ok (Salary fields)
