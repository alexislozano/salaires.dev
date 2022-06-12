module Models.Level exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Utils


type Level
    = Level String


tryNew : String -> Result String Level
tryNew level =
    if String.length level > 0 then
        Ok (Level level)

    else
        Err "Le niveau ne peut pas être vide"


decoder : Decoder Level
decoder =
    Decode.string
        |> Decode.map tryNew
        |> Decode.andThen Utils.resultDecoder


toString : Level -> String
toString (Level level) =
    level


compare : Level -> Level -> Order
compare a b =
    Basics.compare (toString a) (toString b)
