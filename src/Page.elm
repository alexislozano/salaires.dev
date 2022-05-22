module Page exposing (..)

import Element exposing (Element)
import Flags exposing (Flags)
import Pages.Index as Index
import Pages.NotFound as NotFound
import Route
import Url exposing (Url)
import Utils


type Model
    = IndexModel Index.Model
    | NotFoundModel NotFound.Model


type Msg
    = IndexMsg Index.Msg
    | NotFoundMsg NotFound.Msg


init : Flags -> Url -> ( Model, Cmd Msg )
init flags url =
    case Route.parse url of
        Route.Index ->
            Index.init flags
                |> Utils.map IndexModel IndexMsg

        Route.NotFound ->
            NotFound.init flags
                |> Utils.map NotFoundModel NotFoundMsg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        ( IndexMsg subMsg, IndexModel subModel ) ->
            Index.update subMsg subModel
                |> Utils.map IndexModel IndexMsg

        ( NotFoundMsg subMsg, NotFoundModel subModel ) ->
            NotFound.update subMsg subModel
                |> Utils.map NotFoundModel NotFoundMsg

        _ ->
            ( model, Cmd.none )


view : Model -> Element msg
view model =
    case model of
        IndexModel subModel ->
            Index.view subModel

        NotFoundModel subModel ->
            NotFound.view subModel
