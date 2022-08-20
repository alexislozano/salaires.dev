module Design.Form exposing (..)

import Css
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes


view : { title : String } -> List (Html msg) -> Html msg
view { title } elements =
    Html.form
        [ Attributes.css
            [ Css.maxWidth (Css.px 500)
            , Css.property "height" "fit-content"
            , Css.displayFlex
            , Css.flexDirection Css.column
            , Css.property "gap" "16px"
            , Css.padding (Css.px 32)
            , Css.margin Css.auto
            ]
        ]
        (Html.h2
            [ Attributes.css
                [ Css.fontSize (Css.px 32)
                , Css.padding2 (Css.px 16) (Css.px 32)
                , Css.margin Css.zero
                , Css.fontWeight Css.normal
                , Css.textAlign Css.center
                ]
            ]
            [ Html.text title ]
            :: elements
        )
