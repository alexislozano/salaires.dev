module Models.Stock exposing (..)

import Json.Decode as Decode
import Utils


type Stock
    = Stock Int


decode : Decode.Decoder Stock
decode =
    Decode.map Stock Decode.int


toInt : Stock -> Int
toInt stock =
    case stock of
        Stock s ->
            s


toString : Stock -> String
toString stock =
    toInt stock
        |> Utils.toK


compare : Stock -> Stock -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
