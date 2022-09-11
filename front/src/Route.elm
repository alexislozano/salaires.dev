module Route exposing (..)

import Url exposing (Url)
import Url.Parser exposing ((<?>))
import Url.Parser.Query


type Route
    = Index (Maybe String)
    | Insert
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
    Url.Parser.oneOf
        [ Url.Parser.map Index <| Url.Parser.top <?> Url.Parser.Query.string "token"
        , Url.Parser.map Insert <| Url.Parser.s "insert"
        ]


toString : Route -> String
toString route =
    case route of
        Index (Just token) ->
            "/?token=" ++ token

        Index Nothing ->
            "/"

        Insert ->
            "/insert"

        NotFound ->
            "/notfound"
