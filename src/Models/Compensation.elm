module Models.Compensation exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Compensation
    = Compensation Int


tryFromInt : Int -> Result String Compensation
tryFromInt compensation =
    if compensation >= 0 then
        Ok (Compensation compensation)

    else
        Err (I18n.translate I18n.French I18n.ShouldBePositive)


tryFromString : String -> Result String Compensation
tryFromString compensation =
    String.toInt compensation
        |> Result.fromMaybe (I18n.translate I18n.French I18n.ShouldBeANumber)
        |> Result.andThen tryFromInt


decoder : Decoder Compensation
decoder =
    Decode.int
        |> Decode.map tryFromInt
        |> Decode.andThen Utils.resultDecoder


encode : Compensation -> Value
encode (Compensation compensation) =
    Encode.int compensation


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
