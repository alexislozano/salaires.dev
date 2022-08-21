module Design.Utils exposing (..)

import Css
import Design.Palette as Palette
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import I18n


label : Bool -> String -> Maybe String -> Html msg
label required text mSub =
    Html.span [ Attributes.css [ Css.displayFlex, Css.justifyContent Css.spaceBetween, Css.alignItems Css.center ] ]
        (List.concat
            [ [ Html.span []
                    (List.concat
                        [ [ Html.span [ Attributes.css [ Css.fontWeight Css.bold ] ] [ Html.text text ] ]
                        , if required then
                            []

                          else
                            [ Html.span [ Attributes.css [ Css.color Palette.grey, Css.fontWeight Css.bold ] ]
                                [ Html.text (" - " ++ I18n.translate I18n.French I18n.Optional) ]
                            ]
                        ]
                    )
              ]
            , case mSub of
                Just sub ->
                    [ Html.span [ Attributes.css [ Css.fontSize (Css.px 12) ] ]
                        [ Html.text sub ]
                    ]

                Nothing ->
                    []
            ]
        )
