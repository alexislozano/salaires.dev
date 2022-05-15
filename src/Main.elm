module Main exposing (..)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation exposing (Key)
import Flags exposing (Flags)
import Url exposing (Url)


main : Program Flags Model Msg
main =
    Browser.application
        { init = init
        , onUrlChange = onUrlChange
        , onUrlRequest = onUrlRequest
        , subscriptions = subscriptions
        , update = update
        , view = view
        }


type alias Model =
    ()


type Msg
    = NoOp


init : Flags -> Url -> Key -> ( Model, Cmd Msg )
init _ _ _ =
    ( (), Cmd.none )


onUrlChange : Url -> Msg
onUrlChange _ =
    NoOp


onUrlRequest : UrlRequest -> Msg
onUrlRequest _ =
    NoOp


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none


update : Msg -> Model -> ( Model, Cmd Msg )
update _ model =
    ( model, Cmd.none )


view : Model -> Document Msg
view _ =
    { title = "salaires.dev"
    , body = []
    }
