module Models.Stock exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Utils


type Stock
    = Stock Int


tryNew : Int -> Result String Stock
tryNew stock =
    if stock >= 0 then
        Ok (Stock stock)

    else
        Err "Le stock ne peut pas être négatif"


decoder : Decoder Stock
decoder =
    Decode.int
        |> Decode.map tryNew
        |> Decode.andThen Utils.resultDecoder


toInt : Stock -> Int
toInt (Stock stock) =
    stock


toString : Stock -> String
toString stock =
    toInt stock |> Utils.toK


compare : Stock -> Stock -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
