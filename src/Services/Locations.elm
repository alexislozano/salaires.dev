module Services.Locations exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode exposing (Decoder, andThen, list, succeed)
import Json.Decode.Pipeline exposing (required)
import Models.Location as Location exposing (Location)
import Services.Supabase as Supabase


getAll : Flags -> (Result Http.Error (List Location) -> msg) -> Cmd msg
getAll flags msg =
    Http.request
        { method = "GET"
        , headers = Supabase.headers flags
        , url = Supabase.getAllLocationsUrl flags
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg decoder
        }


type alias Response =
    { location : Location }


toLocationDecoder : Response -> Decoder Location
toLocationDecoder { location } =
    succeed location


decoder : Decoder (List Location)
decoder =
    succeed Response
        |> required "location" Location.decoder
        |> andThen toLocationDecoder
        |> list
