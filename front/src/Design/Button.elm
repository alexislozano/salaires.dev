module Design.Button exposing (..)

import Css
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Html.Styled.Events as Events


view :
    { disabled : Bool
    , label : String
    , onClick : msg
    }
    -> Html msg
view { disabled, label, onClick } =
    Html.button
        [ Attributes.css
            [ Css.border2 (Css.px 2) Css.solid
            , Css.borderRadius (Css.px 4)
            , Css.padding (Css.px 12)
            , Css.width (Css.pct 100)
            , Css.fontWeight Css.bold
            , Css.fontFamily Css.inherit
            , Css.fontSize Css.inherit
            , Css.color <|
                if disabled then
                    Palette.grey

                else
                    Palette.black
            , Css.borderColor <|
                if disabled then
                    Palette.grey

                else
                    Palette.black
            , Css.backgroundColor <|
                if disabled then
                    Palette.white

                else
                    Palette.yellow
            ]
        , Events.onClick onClick
        , Attributes.disabled disabled
        , Attributes.type_ "button"
        ]
        [ Html.text label ]
