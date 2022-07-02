module Design.Select exposing (..)

import Element exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Element.Events exposing (onFocus, onLoseFocus)
import Element.Input as Input


type alias Model =
    { textFocused : Bool
    , radioFocused : Bool
    }


init : Model
init =
    { textFocused = False
    , radioFocused = False
    }


opened : Model -> Bool
opened { textFocused, radioFocused } =
    textFocused || radioFocused


type Msg
    = FocusText
    | UnfocusText
    | FocusRadio
    | UnfocusRadio


update : Msg -> Model -> Model
update msg model =
    case msg of
        FocusText ->
            { model | textFocused = True }

        UnfocusText ->
            { model | textFocused = False }

        FocusRadio ->
            { model | radioFocused = True }

        UnfocusRadio ->
            { model | radioFocused = False }


view :
    Model
    ->
        { label : String
        , onChange : String -> msg
        , options : List String
        , placeholder : String
        , toMsg : Msg -> msg
        , value : String
        }
    -> Element msg
view model { label, onChange, options, placeholder, toMsg, value } =
    Element.column
        [ Element.width Element.fill
        , Element.spacing 16
        ]
        [ Input.text
            [ onFocus <| toMsg FocusText
            , onLoseFocus <| toMsg UnfocusText
            ]
            { label = Input.labelAbove [] <| Element.text label
            , onChange = onChange
            , placeholder = Just <| Input.placeholder [] (Element.text placeholder)
            , text = value
            }
        , if List.isEmpty options || not (opened model) then
            Element.none

          else
            Input.radio
                [ Border.width 1
                , Element.width Element.fill
                , onFocus <| toMsg FocusRadio
                , onLoseFocus <| toMsg UnfocusRadio
                , Element.height <| Element.px 200
                , Element.scrollbarY
                ]
                { onChange = onChange
                , options = options |> List.map (\option -> Input.optionWith option (optionView option))
                , selected = Just value
                , label = Input.labelHidden label
                }
        ]


optionView : String -> Input.OptionState -> Element msg
optionView option state =
    Element.el
        [ Element.padding 8
        , Element.width Element.fill
        , Background.color <|
            case state of
                Input.Idle ->
                    Element.rgb255 255 255 255

                _ ->
                    Element.rgb255 200 200 200
        ]
        (Element.text option)
