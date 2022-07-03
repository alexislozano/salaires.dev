module Design.Input exposing (..)

import Design.Palette as Palette
import Design.Utils as Utils
import Element exposing (Element)
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input


view :
    { error : Maybe String
    , label : String
    , onChange : String -> msg
    , placeholder : String
    , required : Bool
    , value : String
    }
    -> Element msg
view { error, label, onChange, placeholder, required, value } =
    Element.column
        [ Element.width Element.fill
        , Element.spacing 8
        , Font.bold
        ]
        [ Input.text
            [ Border.width 2
            , Border.color <|
                case error of
                    Just _ ->
                        Palette.red

                    Nothing ->
                        Palette.black
            , Border.rounded 4
            ]
            { label = Utils.label required label
            , onChange = onChange
            , placeholder =
                placeholder
                    |> Element.text
                    |> Input.placeholder [ Font.color Palette.grey ]
                    |> Just
            , text = value
            }
        , Element.el
            [ Font.color Palette.red ]
            (Element.text (error |> Maybe.withDefault " "))
        ]
