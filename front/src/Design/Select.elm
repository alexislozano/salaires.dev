module Design.Select exposing (..)

import Css
import Design.Palette as Palette
import Design.Utils as Utils
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Html.Styled.Events as Events


view :
    { error : Maybe String
    , id : String
    , label : String
    , onChange : String -> msg
    , options : List String
    , placeholder : String
    , required : Bool
    , value : String
    }
    -> Html msg
view { error, id, label, onChange, options, placeholder, required, value } =
    Html.label
        [ Attributes.css
            [ Css.displayFlex
            , Css.flexDirection Css.column
            , Css.property "gap" "4px"
            , Css.width (Css.pct 100)
            ]
        ]
        [ Utils.label required label Nothing
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
            , Attributes.list id
            , Attributes.value value
            , Attributes.placeholder placeholder
            , Events.onInput onChange
            ]
            []
        , Html.datalist [ Attributes.id id ]
            (options
                |> List.map (\option -> Html.option [ Attributes.value option ] [])
            )
        , Html.span [ Attributes.css [ Css.color Palette.red ] ]
            [ Html.text (error |> Maybe.withDefault " ") ]
        ]
