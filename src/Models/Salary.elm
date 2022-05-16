module Models.Salary exposing (..)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Models.CompanyName as CompanyName exposing (CompanyName)
import Models.CompanyXp as CompanyXp exposing (CompanyXp)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Level as Level exposing (Level)
import Models.LocationName as LocationName exposing (LocationName)
import Models.SalaryId as SalaryId exposing (SalaryId)
import Models.Stock as Stock exposing (Stock)
import Models.TotalXp as TotalXp exposing (TotalXp)


type alias Salary =
    { -- required
      id : SalaryId
    , companyName : CompanyName
    , locationName : LocationName
    , compensation : Compensation
    , date : String

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
        |> required "companyName" CompanyName.decode
        |> required "locationName" LocationName.decode
        |> required "compensation" Compensation.decode
        |> required "date" Decode.string
        |> required "stock" (Decode.maybe Stock.decode)
        |> required "level" (Decode.maybe Level.decode)
        |> required "companyXp" (Decode.maybe CompanyXp.decode)
        |> required "totalXp" (Decode.maybe TotalXp.decode)
