module Models.Compensation exposing (..)

import Json.Decode as Decode


type Compensation
    = Compensation Int


decode : Decode.Decoder Compensation
decode =
    Decode.map Compensation Decode.int
