module Design.Select exposing (..)

import Design.Palette as Palette
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
        { error : Maybe String
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
        , Font.bold
        ]
        [ Input.text
            [ onFocus <| toMsg FocusText
            , onLoseFocus <| toMsg UnfocusText
            , Border.width 2
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
        , if List.isEmpty options || not (opened model) then
            Element.none

          else
            Input.radio
                [ Border.width 2
                , Element.width Element.fill
                , onFocus <| toMsg FocusRadio
                , onLoseFocus <| toMsg UnfocusRadio
                , Element.height <| Element.px 200
                , Element.scrollbarY
                , Border.rounded 4
                ]
                { onChange = onChange
                , options = options |> List.map (\option -> Input.optionWith option (optionView option))
                , selected = Just value
                , label = Input.labelHidden label
                }
        , Element.el
            [ Font.color Palette.red ]
            (Element.text (error |> Maybe.withDefault " "))
        ]


optionView : String -> Input.OptionState -> Element msg
optionView option state =
    Element.el
        [ Element.padding 8
        , Element.width Element.fill
        , Background.color <|
            case state of
                Input.Idle ->
                    Palette.white

                _ ->
                    Palette.yellow
        ]
        (Element.text option)
