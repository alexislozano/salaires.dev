module Models.LocationName exposing (..)

import Json.Decode as Decode


type LocationName
    = LocationName String


decode : Decode.Decoder LocationName
decode =
    Decode.map LocationName Decode.string
