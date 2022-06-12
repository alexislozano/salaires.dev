module Models.Company exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Utils


type Company
    = Company String


tryNew : String -> Result String Company
tryNew company =
    if String.length company > 0 then
        Ok (Company company)

    else
        Err "L'entreprise ne peut pas être vide"


decoder : Decoder Company
decoder =
    Decode.string
        |> Decode.map tryNew
        |> Decode.andThen Utils.resultDecoder


toString : Company -> String
toString (Company company) =
    company


compare : Company -> Company -> Order
compare a b =
    Basics.compare (toString a) (toString b)
