module Design.Form exposing (..)

import Element exposing (Element)
import Element.Font as Font


view : { title : String } -> List (Element msg) -> Element msg
view { title } elements =
    Element.column
        [ Element.paddingXY 0 32
        , Element.centerX
        , Element.spacing 16
        , Element.width <| Element.maximum 500 Element.fill
        ]
        ((Element.el
            [ Font.size 32
            , Element.centerX
            , Element.paddingXY 0 32
            ]
          <|
            Element.text title
         )
            :: elements
        )
