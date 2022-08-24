module Design.Challenge exposing (..)

import Css
import Css.Global as Global
import Design.Palette as Palette
import Design.Utils as Utils
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Html.Styled.Events as Events
import I18n
import Models.Challenge as Challenge exposing (Challenge)


view :
    { challenge : Challenge
    , error : Maybe String
    , onChange : String -> msg
    , required : Bool
    , value : String
    }
    -> Html msg
view { challenge, error, onChange, required, value } =
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
        , Utils.label required
            (I18n.translate I18n.French I18n.Captcha)
            Nothing
        , Html.div
            [ Attributes.css
                [ Css.width (Css.pct 100)
                , Css.padding (Css.px 8)
                , Css.backgroundColor Palette.white
                , Css.boxSizing Css.borderBox
                , Css.borderRadius (Css.px 4)
                , Css.displayFlex
                , Css.justifyContent Css.center
                ]
            ]
            [ Html.img
                [ Attributes.src <| "data:image/png;base64," ++ Challenge.toString challenge
                , Attributes.width 220
                , Attributes.height 120
                ]
                []
            ]
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
            , Attributes.placeholder "abc123"
            , Events.onInput onChange
            ]
            []
        , Html.span [ Attributes.css [ Css.color Palette.red ] ]
            [ Html.text (error |> Maybe.withDefault " ") ]
        ]
