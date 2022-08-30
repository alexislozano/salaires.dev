module Page exposing (..)

import Css
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Notification
import Pages.Index as Index
import Pages.Insert as Insert
import Pages.Maintenance as Maintenance
import Pages.NoInsert as NoInsert
import Pages.NotFound as NotFound
import Route
import Url exposing (Url)
import Utils


type Model
    = IndexModel Index.Model
    | InsertModel Insert.Model
    | NoInsertModel
    | NotFoundModel
    | MaintenanceModel


type Msg
    = IndexMsg Index.Msg
    | InsertMsg Insert.Msg


init : Flags -> Url -> ( Model, Cmd Msg )
init flags url =
    if flags.maintenance then
        ( MaintenanceModel, Cmd.none )

    else
        case Route.parse url of
            Route.Index ->
                Index.init flags
                    |> Utils.map IndexModel IndexMsg

            Route.Insert ->
                if flags.noInsert then
                    ( NoInsertModel, Cmd.none )

                else
                    Insert.init flags
                        |> Utils.map InsertModel InsertMsg

            Route.NotFound ->
                ( NotFoundModel, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    case model of
        InsertModel _ ->
            Insert.subscriptions |> Sub.map InsertMsg

        _ ->
            Sub.none


update : Flags -> Msg -> Model -> ( Model, Cmd Msg )
update flags msg model =
    case ( msg, model ) of
        ( IndexMsg subMsg, IndexModel subModel ) ->
            Index.update subMsg subModel
                |> Utils.map IndexModel IndexMsg

        ( InsertMsg subMsg, InsertModel subModel ) ->
            Insert.update flags subMsg subModel
                |> Utils.map InsertModel InsertMsg

        _ ->
            ( model, Cmd.none )


extractNotification : Msg -> Maybe Notification.Msg
extractNotification msg =
    case msg of
        InsertMsg subMsg ->
            Insert.extractNotification subMsg

        _ ->
            Nothing


view : Flags -> Model -> Maybe Notification.Msg -> Html Msg
view flags model mNotification =
    Html.main_
        [ Attributes.css
            [ Css.overflow Css.auto
            , Css.flexGrow (Css.num 1)
            ]
        ]
        ((case mNotification of
            Just notification ->
                Notification.view notification

            Nothing ->
                Html.text ""
         )
            :: (case model of
                    IndexModel subModel ->
                        Index.view subModel
                            |> List.map (Html.map IndexMsg)

                    InsertModel subModel ->
                        Insert.view flags subModel
                            |> List.map (Html.map InsertMsg)

                    NoInsertModel ->
                        NoInsert.view

                    NotFoundModel ->
                        NotFound.view

                    MaintenanceModel ->
                        Maintenance.view
               )
        )
