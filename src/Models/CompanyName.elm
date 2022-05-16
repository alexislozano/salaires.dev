module Models.CompanyName exposing (..)

import Json.Decode as Decode


type CompanyName
    = CompanyName String


decode : Decode.Decoder CompanyName
decode =
    Decode.map CompanyName Decode.string
