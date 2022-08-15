module Services.Tokens exposing (..)

import Flags exposing (Flags)
import Http
import Json.Encode exposing (Value, object)
import Models.Email as Email exposing (Email)


post : Flags -> (Result Http.Error () -> msg) -> Body -> Cmd msg
post flags msg body =
    Http.request
        { method = "POST"
        , headers = []
        , url = flags.apiUrl ++ "/tokens"
        , body = Http.jsonBody <| encode body
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectWhatever msg
        }


type alias Body =
    { email : Email }


encode : Body -> Value
encode body =
    object
        [ ( "email", Email.encode body.email )
        ]
