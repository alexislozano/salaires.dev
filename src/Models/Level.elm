module Models.Level exposing (..)

import Json.Decode as Decode


type Level
    = Level String


decode : Decode.Decoder Level
decode =
    Decode.map Level Decode.string


toString : Level -> String
toString (Level level) =
    level


compare : Level -> Level -> Order
compare a b =
    Basics.compare (toString a) (toString b)
