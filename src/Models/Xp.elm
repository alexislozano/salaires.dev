module Models.Xp exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Utils


type Xp
    = Xp Int


tryNew : Int -> Result String Xp
tryNew xp =
    if xp >= 0 then
        Ok (Xp xp)

    else
        Err "L'expérience ne peut pas être négative"


decoder : Decoder Xp
decoder =
    Decode.int
        |> Decode.map tryNew
        |> Decode.andThen Utils.resultDecoder


toInt : Xp -> Int
toInt (Xp xp) =
    xp


toString : Xp -> String
toString xp =
    toInt xp |> String.fromInt


compare : Xp -> Xp -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
