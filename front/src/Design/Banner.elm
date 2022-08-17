module Design.Banner exposing (..)

import Design.Palette as Palette
import Element exposing (Attribute, Element)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font


view :
    List (Attribute msg)
    -> { text : String }
    -> Element msg
view attributes { text } =
    Element.el
        attributes
    <|
        Element.el
            [ Element.width Element.fill
            , Element.padding 12
            , Border.width 1
            , Border.rounded 4
            , Border.color Palette.darkBlue
            , Background.color Palette.lightBlue
            , Font.color Palette.darkBlue
            ]
        <|
            Element.paragraph
                []
                [ Element.text text ]
