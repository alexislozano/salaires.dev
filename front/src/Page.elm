module Page exposing (..)

import Css
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Notification
import Pages.Index as Index
import Pages.Insert as Insert
import Pages.Login as Login
import Pages.Maintenance as Maintenance
import Pages.NoInsert as NoInsert
import Pages.NotFound as NotFound
import Route
import Url exposing (Url)
import Utils


type Model
    = IndexModel Index.Model
    | InsertModel Insert.Model
    | NoInsertModel NoInsert.Model
    | LoginModel Login.Model
    | NotFoundModel NotFound.Model
    | MaintenanceModel Maintenance.Model


type Msg
    = IndexMsg Index.Msg
    | InsertMsg Insert.Msg
    | NoInsertMsg NoInsert.Msg
    | LoginMsg Login.Msg
    | NotFoundMsg NotFound.Msg
    | MaintenanceMsg Maintenance.Msg


init : Flags -> Url -> ( Model, Cmd Msg )
init flags url =
    if flags.maintenance then
        Maintenance.init flags
            |> Utils.map MaintenanceModel MaintenanceMsg

    else
        case Route.parse url of
            Route.Index ->
                Index.init flags
                    |> Utils.map IndexModel IndexMsg

            Route.Insert ->
                if flags.noInsert then
                    NoInsert.init flags
                        |> Utils.map NoInsertModel NoInsertMsg

                else
                    Insert.init flags
                        |> Utils.map InsertModel InsertMsg

            Route.Login ->
                if flags.noInsert then
                    NoInsert.init flags
                        |> Utils.map NoInsertModel NoInsertMsg

                else
                    Login.init flags
                        |> Utils.map LoginModel LoginMsg

            Route.NotFound ->
                NotFound.init flags
                    |> Utils.map NotFoundModel NotFoundMsg


update : Flags -> Msg -> Model -> ( Model, Cmd Msg )
update flags msg model =
    case ( msg, model ) of
        ( IndexMsg subMsg, IndexModel subModel ) ->
            Index.update subMsg subModel
                |> Utils.map IndexModel IndexMsg

        ( InsertMsg subMsg, InsertModel subModel ) ->
            Insert.update flags subMsg subModel
                |> Utils.map InsertModel InsertMsg

        ( NoInsertMsg subMsg, NoInsertModel subModel ) ->
            NoInsert.update subMsg subModel
                |> Utils.map NoInsertModel NoInsertMsg

        ( LoginMsg subMsg, LoginModel subModel ) ->
            Login.update flags subMsg subModel
                |> Utils.map LoginModel LoginMsg

        ( NotFoundMsg subMsg, NotFoundModel subModel ) ->
            NotFound.update subMsg subModel
                |> Utils.map NotFoundModel NotFoundMsg

        ( MaintenanceMsg subMsg, MaintenanceModel subModel ) ->
            Maintenance.update subMsg subModel
                |> Utils.map MaintenanceModel MaintenanceMsg

        _ ->
            ( model, Cmd.none )


extractNotification : Msg -> Maybe Notification.Msg
extractNotification msg =
    case msg of
        LoginMsg subMsg ->
            Login.extractNotification subMsg

        InsertMsg subMsg ->
            Insert.extractNotification subMsg

        _ ->
            Nothing


view : Model -> Maybe Notification.Msg -> Html Msg
view model mNotification =
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
                        Insert.view subModel
                            |> List.map (Html.map InsertMsg)

                    NoInsertModel subModel ->
                        NoInsert.view subModel
                            |> List.map (Html.map NoInsertMsg)

                    LoginModel subModel ->
                        Login.view subModel
                            |> List.map (Html.map LoginMsg)

                    NotFoundModel subModel ->
                        NotFound.view subModel
                            |> List.map (Html.map NotFoundMsg)

                    MaintenanceModel subModel ->
                        Maintenance.view subModel
                            |> List.map (Html.map MaintenanceMsg)
               )
        )
