module Models.Location exposing (..)

import Json.Decode as Decode


type Location
    = Location String


decode : Decode.Decoder Location
decode =
    Decode.map Location Decode.string


toString : Location -> String
toString (Location location) =
    location


compare : Location -> Location -> Order
compare a b =
    Basics.compare (toString a) (toString b)
