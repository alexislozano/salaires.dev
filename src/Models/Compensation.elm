module Models.Compensation exposing (..)

import Json.Decode as Decode
import Utils


type Compensation
    = Compensation Int


decode : Decode.Decoder Compensation
decode =
    Decode.map Compensation Decode.int


toInt : Compensation -> Int
toInt (Compensation compensation) =
    compensation


toString : Compensation -> String
toString compensation =
    compensation
        |> toInt
        |> Utils.toK


compare : Compensation -> Compensation -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
