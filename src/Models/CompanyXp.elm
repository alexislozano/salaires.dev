module Models.CompanyXp exposing (..)

import Json.Decode as Decode


type CompanyXp
    = CompanyXp Int


decode : Decode.Decoder CompanyXp
decode =
    Decode.map CompanyXp Decode.int


toInt : CompanyXp -> Int
toInt (CompanyXp companyXp) =
    companyXp


toString : CompanyXp -> String
toString companyXp =
    toInt companyXp |> String.fromInt


compare : CompanyXp -> CompanyXp -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
