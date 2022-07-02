module Models.Stock exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Stock
    = Stock Int


tryFromInt : Int -> Result String Stock
tryFromInt stock =
    if stock >= 0 then
        Ok (Stock stock)

    else
        Err (I18n.translate I18n.French I18n.ShouldBePositive)


tryFromString : String -> Result String Stock
tryFromString stock =
    String.toInt stock
        |> Result.fromMaybe (I18n.translate I18n.French I18n.ShouldBeANumber)
        |> Result.andThen tryFromInt


decoder : Decoder Stock
decoder =
    Decode.int
        |> Decode.map tryFromInt
        |> Decode.andThen Utils.resultDecoder


encode : Stock -> Value
encode (Stock stock) =
    Encode.int stock


toInt : Stock -> Int
toInt (Stock stock) =
    stock


toString : Stock -> String
toString stock =
    toInt stock |> Utils.toK


compare : Stock -> Stock -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
