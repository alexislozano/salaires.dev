module Models.LocationName exposing (..)

import Json.Decode as Decode


type LocationName
    = LocationName String


decode : Decode.Decoder LocationName
decode =
    Decode.map LocationName Decode.string


toString : LocationName -> String
toString locationName =
    case locationName of
        LocationName n ->
            n


compare : LocationName -> LocationName -> Order
compare a b =
    Basics.compare (toString a) (toString b)
