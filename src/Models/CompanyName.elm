module Models.CompanyName exposing (..)

import Json.Decode as Decode


type CompanyName
    = CompanyName String


decode : Decode.Decoder CompanyName
decode =
    Decode.map CompanyName Decode.string


toString : CompanyName -> String
toString companyName =
    case companyName of
        CompanyName n ->
            n


compare : CompanyName -> CompanyName -> Order
compare a b =
    Basics.compare (toString a) (toString b)
