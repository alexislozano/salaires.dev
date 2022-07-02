module Models.Level exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Level
    = Level String


tryFromString : String -> Result String Level
tryFromString level =
    if String.length level > 0 then
        Ok (Level level)

    else
        Err (I18n.translate I18n.French I18n.ShouldNotBeEmpty)


decoder : Decoder Level
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder


encode : Level -> Value
encode (Level level) =
    Encode.string level


toString : Level -> String
toString (Level level) =
    level


compare : Level -> Level -> Order
compare a b =
    Basics.compare (toString a) (toString b)
