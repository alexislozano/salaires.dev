module Models.TotalXp exposing (..)

import Json.Decode as Decode


type TotalXp
    = TotalXp Int


decode : Decode.Decoder TotalXp
decode =
    Decode.map TotalXp Decode.int


toInt : TotalXp -> Int
toInt totalXp =
    case totalXp of
        TotalXp xp ->
            xp


toString : TotalXp -> String
toString totalXp =
    toInt totalXp |> String.fromInt


compare : TotalXp -> TotalXp -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
