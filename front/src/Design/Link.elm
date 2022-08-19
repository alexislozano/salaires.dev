module Design.Link exposing (..)

import Css exposing (Style)
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes


view :
    List Style
    ->
        { label : String
        , url : String
        }
    -> Html msg
view styles { label, url } =
    Html.a
        [ Attributes.href url
        , Attributes.css <|
            List.concat
                [ [ Css.border2 (Css.px 2) Css.solid
                  , Css.borderRadius (Css.px 4)
                  , Css.fontWeight Css.bold
                  , Css.backgroundColor Palette.yellow
                  , Css.padding2 (Css.px 12) (Css.px 16)
                  , Css.textAlign Css.center
                  , Css.color Palette.black
                  , Css.textDecoration Css.none
                  , Css.lineHeight (Css.num 1)
                  , Css.boxSizing Css.borderBox
                  , Css.whiteSpace Css.noWrap
                  ]
                , styles
                ]
        ]
        [ Html.text label ]
