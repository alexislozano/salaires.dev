module Models.Location exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Utils


type Location
    = Location String


tryNew : String -> Result String Location
tryNew location =
    if String.length location > 0 then
        Ok (Location location)

    else
        Err "La localisation ne peut pas être vide"


decoder : Decoder Location
decoder =
    Decode.string
        |> Decode.map tryNew
        |> Decode.andThen Utils.resultDecoder


toString : Location -> String
toString (Location location) =
    location


compare : Location -> Location -> Order
compare a b =
    Basics.compare (toString a) (toString b)
