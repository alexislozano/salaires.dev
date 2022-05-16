module Main exposing (..)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation as Nav exposing (Key)
import Element
import Flags exposing (Flags)
import Pages.Index as Index
import Pages.NotFound as NotFound
import Route exposing (Route)
import Url exposing (Url)


main : Program Flags Model Msg
main =
    Browser.application
        { init = init
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        , subscriptions = subscriptions
        , update = update
        , view = view
        }


type alias Model =
    { key : Key
    , route : Route
    , page : Page
    , flags : Flags
    }


type Page
    = IndexPage Index.Model
    | NotFoundPage


type Msg
    = LinkClicked UrlRequest
    | UrlChanged Url


init : Flags -> Url -> Key -> ( Model, Cmd Msg )
init flags url key =
    let
        route =
            Route.parse url

        page =
            case route of
                Route.Index ->
                    IndexPage <| Index.init flags

                Route.NotFound ->
                    NotFoundPage
    in
    ( Model key route page flags
    , Cmd.none
    )


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        LinkClicked urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model, Nav.pushUrl model.key <| Url.toString url )

                Browser.External href ->
                    ( model, Nav.load href )

        UrlChanged url ->
            ( { model | route = Route.parse url }
            , Cmd.none
            )


view : Model -> Document Msg
view model =
    { title = "salaires.dev"
    , body =
        [ Element.layout
            [ Element.width Element.fill
            , Element.height Element.fill
            ]
          <|
            case ( model.route, model.page ) of
                ( Route.Index, IndexPage pageModel ) ->
                    Index.view pageModel

                ( _, _ ) ->
                    NotFound.view
        ]
    }
