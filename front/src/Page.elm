module Page exposing (..)

import Design.Link as Link
import Design.Palette as Palette
import Element exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Flags exposing (Flags)
import I18n
import Pages.Index as Index
import Pages.Insert as Insert
import Pages.NoInsert as NoInsert
import Pages.NotFound as NotFound
import Route
import Url exposing (Url)
import Utils


type Model
    = IndexModel Index.Model
    | InsertModel Insert.Model
    | NoInsertModel NoInsert.Model
    | NotFoundModel NotFound.Model


type Msg
    = IndexMsg Index.Msg
    | InsertMsg Insert.Msg
    | NoInsertMsg NoInsert.Msg
    | NotFoundMsg NotFound.Msg


init : Flags -> Url -> ( Model, Cmd Msg )
init flags url =
    case Route.parse url of
        Route.Index ->
            Index.init flags
                |> Utils.map IndexModel IndexMsg

        Route.Insert ->
            NoInsert.init flags
                |> Utils.map NoInsertModel NoInsertMsg

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

        ( NotFoundMsg subMsg, NotFoundModel subModel ) ->
            NotFound.update subMsg subModel
                |> Utils.map NotFoundModel NotFoundMsg

        _ ->
            ( model, Cmd.none )


view : Model -> Element Msg
view model =
    Element.column
        [ Element.width Element.fill
        , Element.height Element.fill
        ]
        [ header
        , Element.el
            [ Element.width Element.fill
            , Element.height Element.fill
            , Background.color Palette.sand
            , Element.scrollbarY
            ]
          <|
            case model of
                IndexModel subModel ->
                    Index.view subModel
                        |> Element.map IndexMsg

                InsertModel subModel ->
                    Insert.view subModel
                        |> Element.map InsertMsg

                NoInsertModel subModel ->
                    NoInsert.view subModel
                        |> Element.map NoInsertMsg

                NotFoundModel subModel ->
                    NotFound.view subModel
                        |> Element.map NotFoundMsg
        ]


header : Element msg
header =
    Element.row
        [ Element.height <| Element.px 64
        , Element.paddingXY 8 0
        , Element.width Element.fill
        , Border.widthEach { top = 0, right = 0, bottom = 2, left = 0 }
        , Element.spaceEvenly
        , Background.color Palette.peach
        ]
        [ Link.view
            { label = "salaires.dev"
            , url = Route.toString Route.Index
            }
        , Link.view
            { label = I18n.translate I18n.French I18n.IAddMySalary
            , url = Route.toString Route.Insert
            }
        ]
