module Models.SalaryId exposing (..)

import Json.Decode as Decode


type SalaryId
    = SalaryId String


decode : Decode.Decoder SalaryId
decode =
    Decode.map SalaryId Decode.string
