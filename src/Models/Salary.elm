module Models.Salary exposing (..)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Models.Company as Company exposing (Company)
import Models.CompanyXp as CompanyXp exposing (CompanyXp)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Date as Date exposing (Date)
import Models.Level as Level exposing (Level)
import Models.Location as Location exposing (Location)
import Models.SalaryId as SalaryId exposing (SalaryId)
import Models.Stock as Stock exposing (Stock)
import Models.TotalXp as TotalXp exposing (TotalXp)


type alias Salary =
    { -- required
      id : SalaryId
    , company : Company
    , location : Location
    , compensation : Compensation
    , date : Date

    -- optional
    , stock : Maybe Stock
    , level : Maybe Level
    , companyXp : Maybe CompanyXp
    , totalXp : Maybe TotalXp
    }


decode : Decode.Decoder Salary
decode =
    Decode.succeed Salary
        |> required "id" SalaryId.decode
        |> required "company" Company.decode
        |> required "location" Location.decode
        |> required "compensation" Compensation.decode
        |> required "date" Date.decode
        |> required "stock" (Decode.maybe Stock.decode)
        |> required "level" (Decode.maybe Level.decode)
        |> required "company_xp" (Decode.maybe CompanyXp.decode)
        |> required "total_xp" (Decode.maybe TotalXp.decode)
