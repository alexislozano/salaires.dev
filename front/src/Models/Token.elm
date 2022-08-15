module Models.Token exposing (..)

import I18n
import Json.Encode as Encode exposing (Value)


type Token
    = Token String


tryFromString : String -> Result String Token
tryFromString token =
    if String.length token /= 6 then
        Err (I18n.translate I18n.French I18n.ShouldBeSixCharactersLong)

    else
        Ok (Token token)


encode : Token -> Value
encode (Token token) =
    Encode.string token
