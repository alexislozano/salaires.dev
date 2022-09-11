module Main exposing (..)

import Browser exposing (Document, UrlRequest)
import Browser.Navigation as Nav exposing (Key)
import Css
import Css.Global as Global
import Design.Link as Link
import Design.Palette as Palette
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import I18n
import Notification
import Page
import Route
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
    , notification : Maybe Notification.Msg
    , page : Page.Model
    }


type Msg
    = LinkClicked UrlRequest
    | UrlChanged Url
    | HideNotification
    | PageMsg Page.Msg


init : Flags -> Url -> Key -> ( Model, Cmd Msg )
init flags url key =
    Page.init flags url
        |> Utils.map (Model key flags Nothing) PageMsg


subscriptions : Model -> Sub Msg
subscriptions model =
    Page.subscriptions model.page |> Sub.map PageMsg


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
                |> Utils.map (Model model.key model.flags model.notification) PageMsg

        HideNotification ->
            ( { model | notification = Nothing }
            , Cmd.none
            )

        PageMsg pageMsg ->
            case Page.extractNotification pageMsg of
                Just notification ->
                    ( { model | notification = Just notification }
                    , Notification.hide HideNotification
                    )

                Nothing ->
                    Page.update model.flags pageMsg model.page
                        |> Utils.map (Model model.key model.flags model.notification) PageMsg


view : Model -> Document Msg
view model =
    { title = "salaires.dev"
    , body =
        [ header
        , Page.view model.flags model.page model.notification |> Html.map PageMsg
        , Global.global
            [ Global.body
                [ Css.backgroundColor Palette.sand
                , Css.fontFamilies [ "Open Sans", "Helvetica", "Verdana", Css.sansSerif.value ]
                , Css.fontSize (Css.px 14)
                , Css.margin Css.zero
                , Css.height (Css.vh 100)
                , Css.displayFlex
                , Css.flexDirection Css.column
                ]
            ]
        ]
            |> List.map Html.toUnstyled
    }


header : Html msg
header =
    Html.nav
        [ Attributes.css
            [ Css.backgroundColor Palette.peach
            , Css.borderBottom2 (Css.px 2) Css.solid
            , Css.displayFlex
            , Css.justifyContent Css.spaceBetween
            , Css.padding (Css.px 8)
            ]
        ]
        [ Html.div
            [ Attributes.css
                [ Css.displayFlex
                , Css.property "gap" "8px"
                ]
            ]
            [ Link.view []
                { label = "salaires.dev"
                , url = Route.toString (Route.Index Nothing)
                }
            , Link.view []
                { label = "Github"
                , url = "https://github.com/alexislozano/salaires.dev"
                }
            ]
        , Link.view []
            { label = I18n.translate I18n.French I18n.IAddMySalary
            , url = Route.toString Route.Insert
            }
        ]
