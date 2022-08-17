module Pages.Login exposing (..)

import Design.Banner as Banner
import Design.Button as Button
import Design.Form as Form
import Design.Input as Input
import Design.Link as Link
import Element exposing (Element)
import Flags exposing (Flags)
import Html.Attributes exposing (value)
import Http
import I18n
import Models.Email as Email exposing (Email)
import Route
import Services.Tokens as Tokens


type alias Model =
    { form : Form
    , status : Status
    }


type alias Form =
    { email : { value : String, parsed : Result String Email }
    }


body : Form -> Maybe Tokens.Body
body form =
    case form.email.parsed of
        Ok email ->
            Just
                { email = email
                }

        _ ->
            Nothing


type Status
    = Init
    | Loading
    | Success
    | Error


type Field
    = Email


type Msg
    = Sent (Result Http.Error ())
    | OnFieldChange Field String
    | Send


init : Flags -> ( Model, Cmd Msg )
init _ =
    ( { form =
            { email = { value = "", parsed = Err " " }
            }
      , status = Init
      }
    , Cmd.none
    )


update : Flags -> Msg -> Model -> ( Model, Cmd Msg )
update flags msg model =
    case msg of
        Sent (Ok _) ->
            ( { model | status = Success }
            , Cmd.none
            )

        Sent _ ->
            ( { model | status = Error }
            , Cmd.none
            )

        Send ->
            ( { model | status = Loading }
            , case body model.form of
                Nothing ->
                    Cmd.none

                Just b ->
                    Tokens.post flags Sent b
            )

        OnFieldChange field value ->
            let
                form =
                    model.form

                newForm =
                    case field of
                        Email ->
                            { form
                                | email =
                                    { value = value
                                    , parsed = Email.tryFromString value
                                    }
                            }
            in
            ( { model | form = newForm, status = Init }, Cmd.none )


view : Model -> Element Msg
view { form, status } =
    Form.view
        { title = I18n.translate I18n.French I18n.GetAToken }
        [ Banner.view
            [ Element.paddingEach
                { top = 0
                , left = 0
                , right = 0
                , bottom = 32
                }
            ]
            { text = I18n.translate I18n.French I18n.LoginBanner }
        , Input.view
            { error = error form.email.parsed
            , label = I18n.translate I18n.French I18n.Email
            , onChange = OnFieldChange Email
            , placeholder = "moi@exemple.fr"
            , required = True
            , value = form.email.value
            }
        , Button.view
            { disabled = disabled form
            , label =
                I18n.translate I18n.French <|
                    case status of
                        Init ->
                            I18n.GetAToken

                        Loading ->
                            I18n.Sending

                        Error ->
                            I18n.Error

                        Success ->
                            I18n.Sent
            , onClick = Send
            }
        , Link.view
            [ Element.width Element.fill ]
            { label = I18n.translate I18n.French I18n.IGotAToken
            , url = Route.toString Route.Insert
            }
        ]


error : Result String a -> Maybe String
error state =
    case state of
        Err e ->
            Just e

        _ ->
            Nothing


disabled : Form -> Bool
disabled form =
    case body form of
        Just _ ->
            False

        Nothing ->
            True
