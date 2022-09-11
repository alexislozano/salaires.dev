module Design.Banner exposing (..)

import Css
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes


view :
    { text : String }
    -> Html msg
view { text } =
    Html.p
        [ Attributes.css
            [ Css.padding (Css.px 12)
            , Css.margin Css.zero
            , Css.border3 (Css.px 1) Css.solid Palette.darkBlue
            , Css.borderRadius (Css.px 4)
            , Css.backgroundColor Palette.lightBlue
            , Css.color Palette.darkBlue
            ]
        ]
        [ Html.text text ]
