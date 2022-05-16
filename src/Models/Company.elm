module Models.Company exposing (..)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Models.CompanyName as CompanyName exposing (CompanyName)


type alias Company =
    { name : CompanyName }


decode : Decode.Decoder Company
decode =
    Decode.succeed Company
        |> required "name" CompanyName.decode
