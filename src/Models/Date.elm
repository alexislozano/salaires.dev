module Models.Date exposing (..)

import Iso8601
import Json.Decode as Decode
import Time exposing (Month(..), Posix, posixToMillis, utc)


type Date
    = Date Posix


decode : Decode.Decoder Date
decode =
    Decode.map Date Iso8601.decoder


toString : Date -> String
toString date =
    case date of
        Date d ->
            String.fromInt (Time.toYear utc d)
                ++ "-"
                ++ padWithZero (String.fromInt (monthToInt (Time.toMonth utc d)))
                ++ "-"
                ++ padWithZero (String.fromInt (Time.toDay utc d))


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


compare : Date -> Date -> Order
compare (Date a) (Date b) =
    Basics.compare (posixToMillis a) (posixToMillis b)
