module Utils exposing (..)

import Json.Decode as Decode exposing (Decoder)


map :
    (model1 -> model2)
    -> (msg1 -> msg2)
    -> ( model1, Cmd msg1 )
    -> ( model2, Cmd msg2 )
map toModel toCmd ( model, cmd ) =
    ( toModel model, Cmd.map toCmd cmd )


type HttpData a
    = Loading
    | Success a
    | Failure


toK : Int -> String
toK money =
    money
        |> (\n -> n // 1000)
        |> String.fromInt
        |> (\n -> n ++ "K")


compareMaybe : (a -> a -> Order) -> Maybe a -> Maybe a -> Order
compareMaybe compare maybeA maybeB =
    case ( maybeA, maybeB ) of
        ( Just a, Just b ) ->
            compare a b

        ( Just _, Nothing ) ->
            GT

        ( Nothing, Just _ ) ->
            LT

        ( Nothing, Nothing ) ->
            EQ


resultDecoder : Result String a -> Decoder a
resultDecoder result =
    case result of
        Ok r ->
            Decode.succeed r

        Err e ->
            Decode.fail e
