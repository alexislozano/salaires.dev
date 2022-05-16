module Models.Location exposing (..)

import Json.Decode as Decode
import Json.Decode.Pipeline exposing (required)
import Models.LocationName as LocationName exposing (LocationName)


type alias Location =
    { name : LocationName }


decode : Decode.Decoder Location
decode =
    Decode.succeed Location
        |> required "name" LocationName.decode
