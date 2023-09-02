module Models.CompanyType exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Json.Encode as Encode exposing (Value)
import Utils


type CompanyType
    = GrandGroupe
    | PME
    | ESN


all : List CompanyType
all =
    [ GrandGroupe, PME, ESN ]


toString : CompanyType -> String
toString companyType =
    case companyType of
        GrandGroupe ->
            "Grand groupe"

        PME ->
            "PME"

        ESN ->
            "ESN"


tryFromString : String -> Result String CompanyType
tryFromString companyType =
    case companyType of
        "Grand groupe" ->
            Ok GrandGroupe

        "PME" ->
            Ok PME

        "ESN" ->
            Ok ESN

        _ ->
            Err "Ce type d'entreprise n'existe pas"


decoder : Decoder CompanyType
decoder =
    Decode.string
        |> Decode.map tryFromString
        |> Decode.andThen Utils.resultDecoder


encode : CompanyType -> Value
encode companyType =
    Encode.string (toString companyType)


compare : CompanyType -> CompanyType -> Order
compare a b =
    Basics.compare (toString a) (toString b)
