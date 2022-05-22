module Main exposing (..)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation as Nav exposing (Key)
import Element
import Flags exposing (Flags)
import Page
import Url exposing (Url)
import Utils


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
    , flags : Flags
    , page : Page.Model
    }


type Msg
    = LinkClicked UrlRequest
    | UrlChanged Url
    | PageMsg Page.Msg


init : Flags -> Url -> Key -> ( Model, Cmd Msg )
init flags url key =
    Page.init flags url
        |> Utils.map (Model key flags) PageMsg


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
            Page.init model.flags url
                |> Utils.map (Model model.key model.flags) PageMsg

        PageMsg pageMsg ->
            Page.update pageMsg model.page
                |> Utils.map (Model model.key model.flags) PageMsg


view : Model -> Document Msg
view model =
    { title = "salaires.dev"
    , body =
        [ Element.layout
            [ Element.width Element.fill
            , Element.height Element.fill
            ]
          <|
            Page.view model.page
        ]
    }
