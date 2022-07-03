module Design.Link exposing (..)

import Design.Palette as Palette
import Element exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font


view :
    { label : String
    , url : String
    }
    -> Element msg
view { label, url } =
    Element.link
        [ Border.width 2
        , Border.rounded 4
        , Element.paddingXY 16 12
        , Font.bold
        , Background.color Palette.yellow
        ]
        { label = Element.text label
        , url = url
        }
