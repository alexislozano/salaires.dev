module Design.Utils exposing (..)

import Element
import Element.Font as Font
import Element.Input as Input
import I18n


label : Bool -> String -> Input.Label msg
label required text =
    Input.labelAbove [] <|
        Element.row []
            [ Element.text text
            , if required then
                Element.none

              else
                Element.el
                    [ Font.color (Element.rgb255 150 150 150)
                    ]
                <|
                    Element.text (" - " ++ I18n.translate I18n.French I18n.Optional)
            ]
