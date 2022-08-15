module Design.Link exposing (..)

import Design.Palette as Palette
import Element exposing (Attribute, Element)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font


view :
    List (Attribute msg)
    ->
        { label : String
        , url : String
        }
    -> Element msg
view attributes { label, url } =
    Element.link
        (List.concat
            [ [ Border.width 2
              , Border.rounded 4
              , Element.paddingXY 16 12
              , Font.bold
              , Background.color Palette.yellow
              ]
            , attributes
            ]
        )
        { label =
            Element.el [ Element.centerX ] <|
                Element.text label
        , url = url
        }
