module Design.Input exposing (..)

import Css
import Css.Global as Global
import Design.Palette as Palette
import Design.Utils as Utils
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Html.Styled.Events as Events


view :
    { error : Maybe String
    , label : String
    , sublabel : Maybe String
    , onChange : String -> msg
    , placeholder : String
    , required : Bool
    , value : String
    }
    -> Html msg
view { error, label, sublabel, onChange, placeholder, required, value } =
    Html.label
        [ Attributes.css
            [ Css.displayFlex
            , Css.flexDirection Css.column
            , Css.property "gap" "4px"
            , Css.width (Css.pct 100)
            ]
        ]
        [ Global.global
            [ Global.selector
                "::placeholder"
                [ Css.color Palette.grey
                , Css.opacity (Css.num 1)
                ]
            ]
        , Utils.label required label sublabel
        , Html.input
            [ Attributes.css
                [ Css.border3 (Css.px 2) Css.solid <|
                    case error of
                        Just _ ->
                            Palette.red

                        Nothing ->
                            Palette.black
                , Css.borderRadius (Css.px 4)
                , Css.fontFamily Css.inherit
                , Css.fontSize Css.inherit
                , Css.padding (Css.px 12)
                , Css.fontWeight Css.bold
                ]
            , Attributes.value value
            , Attributes.placeholder placeholder
            , Events.onInput onChange
            ]
            []
        , Html.span [ Attributes.css [ Css.color Palette.red ] ]
            [ Html.text (error |> Maybe.withDefault " ") ]
        ]
