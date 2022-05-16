module Models.Stock exposing (..)

import Json.Decode as Decode


type Stock
    = Stock Int


decode : Decode.Decoder Stock
decode =
    Decode.map Stock Decode.int
