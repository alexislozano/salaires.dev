module Models.Challenge exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Utils


type Challenge
    = Challenge String


tryFromString : String -> Result String Challenge
tryFromString challenge =
    if String.isEmpty challenge then
        Err (I18n.translate I18n.French I18n.ShouldNotBeEmpty)

    else
        Ok (Challenge challenge)


toString : Challenge -> String
toString (Challenge challenge) =
    challenge


decoder : Decoder Challenge
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder
