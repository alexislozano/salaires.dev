module Models.Company exposing (..)

import I18n
import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type Company
    = Company String


tryFromString : String -> Result String Company
tryFromString company =
    if String.isEmpty company then
        Err (I18n.translate I18n.French I18n.ShouldNotBeEmpty)

    else
        Ok (Company company)


decoder : Decoder Company
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder


encode : Company -> Value
encode (Company company) =
    Encode.string company


toString : Company -> String
toString (Company company) =
    company


compare : Company -> Company -> Order
compare a b =
    Basics.compare (toString a) (toString b)
