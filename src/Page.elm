module Page exposing (..)

import Element exposing (Element)
import Element.Border as Border
import Flags exposing (Flags)
import I18n
import Pages.Index as Index
import Pages.Insert as Insert
import Pages.NotFound as NotFound
import Route
import Url exposing (Url)
import Utils


type Model
    = IndexModel Index.Model
    | InsertModel Insert.Model
    | NotFoundModel NotFound.Model


type Msg
    = IndexMsg Index.Msg
    | InsertMsg Insert.Msg
    | NotFoundMsg NotFound.Msg


init : Flags -> Url -> ( Model, Cmd Msg )
init flags url =
    case Route.parse url of
        Route.Index ->
            Index.init flags
                |> Utils.map IndexModel IndexMsg

        Route.Insert ->
            Insert.init flags
                |> Utils.map InsertModel InsertMsg

        Route.NotFound ->
            NotFound.init flags
                |> Utils.map NotFoundModel NotFoundMsg


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        ( IndexMsg subMsg, IndexModel subModel ) ->
            Index.update subMsg subModel
                |> Utils.map IndexModel IndexMsg

        ( InsertMsg subMsg, InsertModel subModel ) ->
            Insert.update subMsg subModel
                |> Utils.map InsertModel InsertMsg

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
            ]
          <|
            case model of
                IndexModel subModel ->
                    Index.view subModel
                        |> Element.map IndexMsg

                InsertModel subModel ->
                    Insert.view subModel
                        |> Element.map InsertMsg

                NotFoundModel subModel ->
                    NotFound.view subModel
                        |> Element.map NotFoundMsg
        ]


header : Element msg
header =
    Element.row
        [ Element.height <| Element.px 64
        , Element.width Element.fill
        , Border.widthEach { top = 0, right = 0, bottom = 1, left = 0 }
        ]
        [ Element.link
            [ Element.paddingEach { top = 0, right = 16, bottom = 0, left = 16 }
            ]
            { label = Element.text "salaires.dev"
            , url = Route.toString Route.Index
            }
        , Element.link
            [ Element.alignRight
            , Element.paddingEach { top = 0, right = 16, bottom = 0, left = 16 }
            ]
            { label = Element.text (I18n.translate I18n.French I18n.IAddMySalary)
            , url = Route.toString Route.Insert
            }
        ]
