module Design.Select exposing (..)

import Design.Utils as Utils
import Element exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Element.Events exposing (onFocus, onLoseFocus)
import Element.Font as Font
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
        { error : String
        , label : String
        , onChange : String -> msg
        , options : List String
        , placeholder : String
        , required : Bool
        , toMsg : Msg -> msg
        , value : String
        }
    -> Element msg
view model { error, label, onChange, options, placeholder, required, toMsg, value } =
    Element.column
        [ Element.width Element.fill
        , Element.spacing 8
        ]
        [ Input.text
            [ onFocus <| toMsg FocusText
            , onLoseFocus <| toMsg UnfocusText
            ]
            { label = Utils.label required label
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
        , Element.el
            [ Font.color (Element.rgb255 255 0 0) ]
            (Element.text error)
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
