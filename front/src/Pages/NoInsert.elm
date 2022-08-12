module Pages.NoInsert exposing (..)

import Element exposing (Element)
import Element.Font as Font
import Flags exposing (Flags)
import I18n


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
        , Font.bold
        ]
    <|
        Element.text (I18n.translate I18n.French I18n.InsertIsDownForNow)
