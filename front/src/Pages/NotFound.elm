module Pages.NotFound exposing (..)

import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
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


view : Model -> List (Html msg)
view _ =
    [ Html.text (I18n.translate I18n.French I18n.ThisPageDoesNotExist) ]
