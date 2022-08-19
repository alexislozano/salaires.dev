module Pages.NoInsert exposing (..)

import Css
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
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


view : Model -> Html msg
view _ =
    Html.main_
        [ Attributes.css
            [ Css.fontWeight Css.bold
            , Css.displayFlex
            , Css.justifyContent Css.center
            , Css.alignItems Css.center
            , Css.flexGrow (Css.num 1)
            ]
        ]
        [ Html.text (I18n.translate I18n.French I18n.InsertIsDownForNow) ]
