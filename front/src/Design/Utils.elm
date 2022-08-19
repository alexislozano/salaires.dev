module Design.Utils exposing (..)

import Css
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import I18n


label : Bool -> String -> Html msg
label required text =
    Html.span [] <|
        Html.text text
            :: (if required then
                    []

                else
                    [ Html.span [ Attributes.css [ Css.color Palette.grey ] ]
                        [ Html.text (" - " ++ I18n.translate I18n.French I18n.Optional) ]
                    ]
               )
