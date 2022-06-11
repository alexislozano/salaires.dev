module Models.Location exposing (..)

import Json.Decode as Decode


type Location
    = Location String


decode : Decode.Decoder Location
decode =
    Decode.map Location Decode.string


toString : Location -> String
toString location =
    case location of
        Location l ->
            l


compare : Location -> Location -> Order
compare a b =
    Basics.compare (toString a) (toString b)
