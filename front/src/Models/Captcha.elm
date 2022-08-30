module Models.Captcha exposing (..)

import Json.Encode as Encode exposing (Value)


type Captcha
    = Captcha String


fromString : String -> Captcha
fromString captcha =
    Captcha captcha


encode : Captcha -> Value
encode (Captcha captcha) =
    Encode.string captcha
