module Design.Input exposing (..)

import Design.Utils as Utils
import Element exposing (Element)
import Element.Font as Font
import Element.Input as Input
import Html.Attributes exposing (placeholder)


view :
    { error : String
    , label : String
    , onChange : String -> msg
    , placeholder : String
    , required : Bool
    , value : String
    }
    -> Element msg
view { error, label, onChange, placeholder, required, value } =
    Element.column
        [ Element.width Element.fill
        , Element.spacing 8
        ]
        [ Input.text
            []
            { label = Utils.label required label
            , onChange = onChange
            , placeholder = Just <| Input.placeholder [] <| Element.text placeholder
            , text = value
            }
        , Element.el
            [ Font.color (Element.rgb255 255 0 0) ]
            (Element.text error)
        ]
