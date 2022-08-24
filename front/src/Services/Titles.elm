module Services.Titles exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode exposing (Decoder, andThen, list, succeed)
import Json.Decode.Pipeline exposing (required)
import Models.Title as Title exposing (Title)


getAll : Flags -> (Result Http.Error (List Title) -> msg) -> Cmd msg
getAll flags msg =
    Http.request
        { method = "GET"
        , headers = []
        , url = flags.apiUrl ++ "/titles"
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg decoder
        }


type alias Response =
    { title : Title }


toTitleDecoder : Response -> Decoder Title
toTitleDecoder { title } =
    succeed title


decoder : Decoder (List Title)
decoder =
    succeed Response
        |> required "title" Title.decoder
        |> andThen toTitleDecoder
        |> list
