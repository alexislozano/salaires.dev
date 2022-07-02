module Design.Button exposing (..)

import Element exposing (Element)
import Element.Border as Border
import Element.Input as Input


view :
    { disabled : Bool
    , label : String
    , onClick : msg
    }
    -> Element msg
view { disabled, label, onClick } =
    Input.button
        [ Border.width 1
        , Element.padding 12
        , Element.width Element.fill
        ]
        { onPress =
            if disabled then
                Nothing

            else
                Just onClick
        , label = Element.el [ Element.centerX ] <| Element.text label
        }
