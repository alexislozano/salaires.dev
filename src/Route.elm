module Route exposing (..)

import Url exposing (Url)
import Url.Parser


type Route
    = Index
    | NotFound


parse : Url -> Route
parse url =
    case Url.Parser.parse parser url of
        Just route ->
            route

        Nothing ->
            NotFound


parser : Url.Parser.Parser (Route -> a) a
parser =
    Url.Parser.oneOf [ Url.Parser.map Index <| Url.Parser.top ]
