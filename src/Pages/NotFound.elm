module Pages.NotFound exposing (..)

import Element exposing (Element)


view : Element msg
view =
    Element.el
        [ Element.centerX
        , Element.centerY
        ]
    <|
        Element.text "Not found"
