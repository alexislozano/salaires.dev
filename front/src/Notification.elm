module Notification exposing (..)

import Css
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import I18n
import Process
import Task


type Msg
    = EmailSent
    | EmailSendingError
    | SalaryInserted
    | SalaryInsertingError


send : (Msg -> msg) -> Msg -> Cmd msg
send toMsg msg =
    Task.perform toMsg (Task.succeed msg)


hide : msg -> Cmd msg
hide msg =
    Process.sleep 2000
        |> Task.perform (always msg)


view : Msg -> Html msg
view msg =
    Html.aside
        [ Attributes.css
            [ Css.position Css.absolute
            , Css.zIndex (Css.int 1)
            , Css.width (Css.pct 100)
            , Css.backgroundColor Palette.lightBlue
            , Css.borderBottom3 (Css.px 2) Css.solid Palette.darkBlue
            , Css.color Palette.darkBlue
            , Css.padding (Css.px 8)
            , Css.boxSizing Css.borderBox
            , Css.width (Css.pct 100)
            , Css.fontWeight Css.bold
            , Css.textAlign Css.center
            ]
        ]
        [ Html.text <|
            case msg of
                EmailSent ->
                    I18n.translate I18n.French I18n.EmailSent

                EmailSendingError ->
                    I18n.translate I18n.French I18n.EmailSendingError

                SalaryInserted ->
                    I18n.translate I18n.French I18n.SalaryInserted

                SalaryInsertingError ->
                    I18n.translate I18n.French I18n.SalaryInstertingError
        ]
