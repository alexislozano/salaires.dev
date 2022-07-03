module Design.Button exposing (..)

import Design.Palette as Palette
import Element exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input


view :
    { disabled : Bool
    , label : String
    , onClick : msg
    }
    -> Element msg
view { disabled, label, onClick } =
    Input.button
        [ Border.width 2
        , Border.rounded 4
        , Element.padding 12
        , Element.width Element.fill
        , Font.bold
        , Font.color <|
            if disabled then
                Palette.grey

            else
                Palette.black
        , Border.color <|
            if disabled then
                Palette.grey

            else
                Palette.black
        , Background.color <|
            if disabled then
                Palette.white

            else
                Palette.yellow
        ]
        { onPress =
            if disabled then
                Nothing

            else
                Just onClick
        , label = Element.el [ Element.centerX ] <| Element.text label
        }
