module Services.Tokens exposing (..)

import Flags exposing (Flags)
import Http
import Json.Encode as Encode exposing (Value, object)


post : Flags -> (Result Http.Error () -> msg) -> String -> Cmd msg
post flags msg token =
    Http.request
        { method = "POST"
        , headers = []
        , url = flags.apiUrl ++ "/tokens"
        , body = Http.jsonBody <| encode token
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectWhatever msg
        }


encode : String -> Value
encode token =
    object
        [ ( "token", Encode.string token ) ]
