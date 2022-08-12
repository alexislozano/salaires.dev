module Models.Date exposing (..)

import Iso8601
import Json.Decode as Decode exposing (Decoder)
import Time exposing (Month(..), Posix, posixToMillis, utc)


type Date
    = Date Posix


new : Posix -> Date
new date =
    Date date


decoder : Decoder Date
decoder =
    Iso8601.decoder
        |> Decode.map new


toString : Date -> String
toString (Date date) =
    String.fromInt (Time.toYear utc date)
        ++ "-"
        ++ padWithZero (String.fromInt (monthToInt (Time.toMonth utc date)))
        ++ "-"
        ++ padWithZero (String.fromInt (Time.toDay utc date))


monthToInt : Month -> Int
monthToInt month =
    case month of
        Jan ->
            1

        Feb ->
            2

        Mar ->
            3

        Apr ->
            4

        May ->
            5

        Jun ->
            6

        Jul ->
            7

        Aug ->
            8

        Sep ->
            9

        Oct ->
            10

        Nov ->
            11

        Dec ->
            12


padWithZero : String -> String
padWithZero s =
    if String.length s == 1 then
        "0" ++ s

    else
        s


toInt : Date -> Int
toInt (Date date) =
    posixToMillis date


compare : Date -> Date -> Order
compare a b =
    Basics.compare (toInt a) (toInt b)
