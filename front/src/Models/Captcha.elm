module Models.Captcha exposing (..)

import I18n
import Json.Encode as Encode exposing (Value)


type Captcha
    = Captcha String


tryFromString : String -> Result String Captcha
tryFromString captcha =
    if String.isEmpty captcha then
        Err (I18n.translate I18n.French I18n.ShouldNotBeEmpty)

    else
        Ok (Captcha captcha)


encode : Captcha -> Value
encode (Captcha captcha) =
    Encode.string captcha
