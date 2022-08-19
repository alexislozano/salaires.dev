module Design.Banner exposing (..)

import Css exposing (Style)
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes


view :
    List Style
    -> { text : String }
    -> Html msg
view styles { text } =
    Html.p
        [ Attributes.css <|
            List.concat
                [ [ Css.padding (Css.px 12)
                  , Css.margin Css.zero
                  , Css.border3 (Css.px 1) Css.solid Palette.darkBlue
                  , Css.borderRadius (Css.px 4)
                  , Css.backgroundColor Palette.lightBlue
                  , Css.color Palette.darkBlue
                  ]
                , styles
                ]
        ]
        [ Html.text text ]
