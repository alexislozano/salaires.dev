module Models.Company exposing (..)

import Json.Decode as Decode


type Company
    = Company String


decode : Decode.Decoder Company
decode =
    Decode.map Company Decode.string


toString : Company -> String
toString (Company company) =
    company


compare : Company -> Company -> Order
compare a b =
    Basics.compare (toString a) (toString b)
