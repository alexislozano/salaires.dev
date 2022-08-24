module Services.Challenges exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode exposing (Decoder, andThen, succeed)
import Json.Decode.Pipeline exposing (required)
import Models.Challenge as Challenge exposing (Challenge)


compute : Flags -> (Result Http.Error Challenge -> msg) -> Cmd msg
compute flags msg =
    Http.request
        { method = "GET"
        , headers = []
        , url = flags.apiUrl ++ "/challenge"
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg decoder
        }


type alias Response =
    { challenge : Challenge }


toChallengeDecoder : Response -> Decoder Challenge
toChallengeDecoder { challenge } =
    succeed challenge


decoder : Decoder Challenge
decoder =
    succeed Response
        |> required "challenge" Challenge.decoder
        |> andThen toChallengeDecoder
