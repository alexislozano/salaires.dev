module Models.Title exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Title
    = Title String


tryFromString : String -> Result String Title
tryFromString title =
    if String.length title > 0 then
        Ok (Title title)

    else
        Err (I18n.translate I18n.French I18n.ShouldNotBeEmpty)


decoder : Decoder Title
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder


encode : Title -> Value
encode (Title title) =
    Encode.string title


toString : Title -> String
toString (Title title) =
    title


compare : Title -> Title -> Order
compare a b =
    Basics.compare (toString a) (toString b)
