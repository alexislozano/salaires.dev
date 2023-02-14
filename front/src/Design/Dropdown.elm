module Design.Dropdown exposing (..)

import Css
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
    , options : List { key : String, label : String }
    , required : Bool
    , value : String
    }
    -> Html msg
view { error, label, sublabel, onChange, options, required, value } =
    Html.label
        [ Attributes.css
            [ Css.displayFlex
            , Css.flexDirection Css.column
            , Css.property "gap" "4px"
            , Css.width (Css.pct 100)
            ]
        ]
        [ Utils.label required label sublabel
        , Html.select
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
                , Css.backgroundColor Palette.white
                ]
            , Events.onInput onChange
            ]
            (options
                |> List.map
                    (\option ->
                        Html.option
                            [ Attributes.value option.key
                            , Attributes.selected <| option.key == value
                            ]
                            [ Html.text option.label ]
                    )
            )
        , Html.span [ Attributes.css [ Css.color Palette.red ] ]
            [ Html.text (error |> Maybe.withDefault " ") ]
        ]
