module Models.Compensation exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Utils


type Compensation
    = Compensation Int


tryNew : Int -> Result String Compensation
tryNew compensation =
    if compensation >= 0 then
        Ok (Compensation compensation)

    else
        Err "La compensation ne peut pas être négative"


decoder : Decoder Compensation
decoder =
    Decode.int
        |> Decode.map tryNew
        |> Decode.andThen Utils.resultDecoder


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
