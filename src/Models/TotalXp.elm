module Models.TotalXp exposing (..)

import Json.Decode as Decode


type TotalXp
    = TotalXp Int


decode : Decode.Decoder TotalXp
decode =
    Decode.map TotalXp Decode.int
