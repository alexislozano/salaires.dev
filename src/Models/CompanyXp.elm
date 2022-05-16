module Models.CompanyXp exposing (..)

import Json.Decode as Decode


type CompanyXp
    = CompanyXp Int


decode : Decode.Decoder CompanyXp
decode =
    Decode.map CompanyXp Decode.int
