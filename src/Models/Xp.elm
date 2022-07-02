module Models.Xp exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Utils


type Xp
    = Xp Int


tryFromInt : Int -> Result String Xp
tryFromInt xp =
    if xp >= 0 then
        Ok (Xp xp)

    else
        Err (I18n.translate I18n.French I18n.ShouldBePositive)


tryFromString : String -> Result String Xp
tryFromString xp =
    String.toInt xp
        |> Result.fromMaybe (I18n.translate I18n.French I18n.ShouldBeANumber)
        |> Result.andThen tryFromInt


decoder : Decoder Xp
decoder =
    Decode.int
        |> Decode.map tryFromInt
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
