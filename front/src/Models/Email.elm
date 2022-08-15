module Models.Email exposing (..)

import I18n
import Json.Encode as Encode exposing (Value)


type Email
    = Email String


tryFromString : String -> Result String Email
tryFromString email =
    if String.contains "@" email then
        Ok (Email email)

    else
        Err (I18n.translate I18n.French I18n.ShouldContainAnAt)


encode : Email -> Value
encode (Email token) =
    Encode.string token
