module Models.Location exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Location
    = Location String


tryFromString : String -> Result String Location
tryFromString location =
    if String.isEmpty location then
        Err (I18n.translate I18n.French I18n.ShouldNotBeEmpty)

    else
        Ok (Location location)


decoder : Decoder Location
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder


encode : Location -> Value
encode (Location location) =
    Encode.string location


toString : Location -> String
toString (Location location) =
    location


compare : Location -> Location -> Order
compare a b =
    Basics.compare (toString a) (toString b)
