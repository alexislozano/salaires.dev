module Pages.Insert exposing (..)

import Element exposing (Element)
import Flags exposing (Flags)


type alias Model =
    ()


type Msg
    = NoOp


init : Flags -> ( Model, Cmd Msg )
init _ =
    ( (), Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        _ ->
            ( model, Cmd.none )


view : Model -> Element msg
view _ =
    Element.el
        [ Element.centerX
        , Element.centerY
        ]
    <|
        Element.text "Insert"
