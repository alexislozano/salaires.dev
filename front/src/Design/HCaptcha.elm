module Design.HCaptcha exposing (..)

import Css
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes


view : { key : String } -> Html msg
view { key } =
    Html.div
        [ Attributes.id "h-captcha"
        , Attributes.attribute "data-sitekey" key
        , Attributes.css
            [ Css.displayFlex
            , Css.justifyContent Css.center
            ]
        ]
        []
