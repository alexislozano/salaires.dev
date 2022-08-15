module Services.Companies exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode exposing (Decoder, andThen, list, succeed)
import Json.Decode.Pipeline exposing (required)
import Models.Company as Company exposing (Company)


getAll : Flags -> (Result Http.Error (List Company) -> msg) -> Cmd msg
getAll flags msg =
    Http.request
        { method = "GET"
        , headers = []
        , url = flags.apiUrl ++ "/companies"
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg decoder
        }


type alias Response =
    { company : Company }


toCompanyDecoder : Response -> Decoder Company
toCompanyDecoder { company } =
    succeed company


decoder : Decoder (List Company)
decoder =
    succeed Response
        |> required "company" Company.decoder
        |> andThen toCompanyDecoder
        |> list
