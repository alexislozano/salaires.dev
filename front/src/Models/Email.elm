module Models.Email exposing (..)

import I18n
import Json.Encode as Encode exposing (Value)


type Email
    = Email String


tryFromString : String -> Result String Email
tryFromString email =
    if String.contains "@" email then
        let
            isValid =
                0
                    == (forbiddenDomains
                            |> List.filter (\domain -> String.contains domain email)
                            |> List.length
                       )
        in
        if isValid then
            Ok (Email email)

        else
            Err (I18n.translate I18n.French I18n.EmailShouldBePro)

    else
        Err (I18n.translate I18n.French I18n.EmailShouldContainAnAt)


forbiddenDomains : List String
forbiddenDomains =
    [ "gmail"
    , "yahoo"
    , "hotmail"
    , "aol"
    , "wanadoo"
    , "msn"
    , "live"
    , "free"
    , "outlook"
    , "laposte"
    , "protonmail"
    , "yopmail"
    , "minutemail"
    ]


encode : Email -> Value
encode (Email token) =
    Encode.string token
