module Pages.Login exposing (..)

import Css
import Design.Banner as Banner
import Design.Button as Button
import Design.Challenge as Challenge
import Design.Form as Form
import Design.Input as Input
import Design.Link as Link
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Http
import I18n
import Models.Captcha as Captcha exposing (Captcha)
import Models.Challenge exposing (Challenge)
import Models.Email as Email exposing (Email)
import Notification
import Route
import Services.Challenges as Challenges
import Services.Tokens as Tokens


type alias Model =
    { form : Form
    , status : Status
    , challenge : Maybe Challenge
    }


type alias Form =
    { email : { value : String, parsed : Result String Email }
    , captcha : { value : String, parsed : Result String Captcha }
    }


body : Form -> Maybe Tokens.Body
body form =
    case ( form.email.parsed, form.captcha.parsed ) of
        ( Ok email, Ok captcha ) ->
            Just
                { email = email
                , captcha = captcha
                }

        _ ->
            Nothing


type Status
    = Init
    | Loading


type Field
    = Email
    | Captcha


type Msg
    = Sent (Result Http.Error ())
    | OnFieldChange Field String
    | Send
    | NotificationMsg Notification.Msg
    | GotChallenge (Result Http.Error Challenge)


init : Flags -> ( Model, Cmd Msg )
init flags =
    ( { form =
            { email = { value = "", parsed = Err " " }
            , captcha = { value = "", parsed = Err " " }
            }
      , status = Init
      , challenge = Nothing
      }
    , Challenges.compute flags GotChallenge
    )


update : Flags -> Msg -> Model -> ( Model, Cmd Msg )
update flags msg model =
    case msg of
        GotChallenge (Ok challenge) ->
            ( { model | challenge = Just challenge }, Cmd.none )

        GotChallenge _ ->
            ( model, Cmd.none )

        Sent (Ok _) ->
            ( { model | status = Init }
            , Notification.send NotificationMsg Notification.EmailSent
            )

        Sent _ ->
            ( { model | status = Init }
            , Notification.send NotificationMsg Notification.EmailSendingError
            )

        Send ->
            ( { model | status = Loading }
            , case body model.form of
                Nothing ->
                    Cmd.none

                Just b ->
                    Tokens.post flags Sent b
            )

        NotificationMsg _ ->
            ( model, Cmd.none )

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

                        Captcha ->
                            { form
                                | captcha =
                                    { value = value
                                    , parsed = Captcha.tryFromString value
                                    }
                            }
            in
            ( { model | form = newForm, status = Init }, Cmd.none )


extractNotification : Msg -> Maybe Notification.Msg
extractNotification msg =
    case msg of
        NotificationMsg subMsg ->
            Just subMsg

        _ ->
            Nothing


view : Model -> List (Html Msg)
view { form, status, challenge } =
    case challenge of
        Just rawChallenge ->
            [ Form.view
                { title = I18n.translate I18n.French I18n.GetAToken }
                [ Banner.view [ Css.marginBottom (Css.px 16) ]
                    { text = I18n.translate I18n.French I18n.LoginBanner }
                , Input.view
                    { error = error form.email.parsed
                    , label = I18n.translate I18n.French I18n.Email
                    , sublabel = Nothing
                    , onChange = OnFieldChange Email
                    , placeholder = "moi@exemple.fr"
                    , required = True
                    , value = form.email.value
                    }
                , Challenge.view
                    { challenge = rawChallenge
                    , error = error form.captcha.parsed
                    , onChange = OnFieldChange Captcha
                    , required = True
                    , value = form.captcha.value
                    }
                , Button.view
                    { disabled = disabled status form
                    , label =
                        I18n.translate I18n.French <|
                            case status of
                                Init ->
                                    I18n.GetAToken

                                Loading ->
                                    I18n.Sending
                    , onClick = Send
                    }
                , Link.view [ Css.width (Css.pct 100) ]
                    { label = I18n.translate I18n.French I18n.IGotAToken
                    , url = Route.toString Route.Insert
                    }
                ]
            ]

        Nothing ->
            []


error : Result String a -> Maybe String
error state =
    case state of
        Err e ->
            Just e

        _ ->
            Nothing


disabled : Status -> Form -> Bool
disabled status form =
    case ( status, body form ) of
        ( Init, Just _ ) ->
            False

        _ ->
            True
