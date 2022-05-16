module Models.Level exposing (..)

import Json.Decode as Decode


type Level
    = Level String


decode : Decode.Decoder Level
decode =
    Decode.map Level Decode.string
