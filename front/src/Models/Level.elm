module Models.Level exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Level
    = Junior
    | Mid
    | Senior


all : List Level
all =
    [ Junior, Mid, Senior ]


tryFromString : String -> Result String Level
tryFromString level =
    case level of
        "Junior" ->
            Ok Junior

        "Mid" ->
            Ok Mid

        "Senior" ->
            Ok Senior

        _ ->
            Err (I18n.translate I18n.French I18n.LevelIsNotInTheProvidedChoices)


decoder : Decoder Level
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder


encode : Level -> Value
encode level =
    Encode.string (toString level)


toString : Level -> String
toString level =
    case level of
        Junior ->
            "Junior"

        Mid ->
            "Mid"

        Senior ->
            "Senior"


toWording : Level -> String
toWording level =
    case level of
        Junior ->
            I18n.translate I18n.French I18n.Junior

        Mid ->
            I18n.translate I18n.French I18n.Mid

        Senior ->
            I18n.translate I18n.French I18n.Senior


compare : Level -> Level -> Order
compare a b =
    Basics.compare (toString a) (toString b)
