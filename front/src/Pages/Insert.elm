module Pages.Insert exposing (..)

import Design.Banner as Banner
import Design.Button as Button
import Design.Form as Form
import Design.HCaptcha as HCaptcha
import Design.Input as Input
import Design.Select as Select
import Flags exposing (Flags)
import Html.Styled exposing (Html)
import Http
import I18n
import Models.Captcha as Captcha exposing (Captcha)
import Models.Company as Company exposing (Company)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Email as Email exposing (Email)
import Models.Level as Level exposing (Level)
import Models.Location as Location exposing (Location)
import Models.Title as Title exposing (Title)
import Models.Xp as Xp exposing (Xp)
import Notification
import Ports
import Services.Companies as Companies
import Services.Locations as Locations
import Services.Salaries as Salaries
import Services.Titles as Titles


type alias Model =
    { form : Form
    , status : Status
    , companies : List Company
    , locations : List Location
    , titles : List Title
    }


type alias Form =
    { email : { value : String, parsed : Result String Email }
    , company : { value : String, parsed : Result String Company }
    , title : { value : String, parsed : Result String (Maybe Title) }
    , location : { value : String, parsed : Result String Location }
    , compensation : { value : String, parsed : Result String Compensation }
    , level : { value : String, parsed : Result String (Maybe Level) }
    , companyXp : { value : String, parsed : Result String (Maybe Xp) }
    , totalXp : { value : String, parsed : Result String (Maybe Xp) }
    , captcha : Maybe Captcha
    }


body : Form -> Maybe Salaries.Body
body form =
    case ( form.company.parsed, form.location.parsed, form.compensation.parsed ) of
        ( Ok company, Ok location, Ok compensation ) ->
            case ( form.level.parsed, form.companyXp.parsed ) of
                ( Ok level, Ok companyXp ) ->
                    case ( form.totalXp.parsed, form.title.parsed, form.captcha ) of
                        ( Ok totalXp, Ok title, Just captcha ) ->
                            case form.email.parsed of
                                Ok email ->
                                    Just
                                        { email = email
                                        , company = company
                                        , title = title
                                        , location = location
                                        , compensation = compensation
                                        , level = level
                                        , companyXp = companyXp
                                        , totalXp = totalXp
                                        , captcha = captcha
                                        }

                                _ ->
                                    Nothing

                        _ ->
                            Nothing

                _ ->
                    Nothing

        _ ->
            Nothing


type Status
    = Init
    | Loading


type Field
    = Email
    | Company
    | Location
    | Compensation
    | CompanyXp
    | TotalXp
    | Level
    | Title
    | Captcha


type Msg
    = GotAllCompanies (Result Http.Error (List Company))
    | GotAllLocations (Result Http.Error (List Location))
    | GotAllTitles (Result Http.Error (List Title))
    | Sent (Result Http.Error ())
    | OnFieldChange Field String
    | Send
    | NotificationMsg Notification.Msg


init : Flags -> ( Model, Cmd Msg )
init flags =
    let
        cmd =
            Cmd.batch
                [ Companies.getAll flags GotAllCompanies
                , Locations.getAll flags GotAllLocations
                , Titles.getAll flags GotAllTitles
                , Ports.renderCaptcha ()
                ]
    in
    ( { form =
            { email = { value = "", parsed = Err " " }
            , company = { value = "", parsed = Err " " }
            , title = { value = "", parsed = Ok Nothing }
            , location = { value = "", parsed = Err " " }
            , compensation = { value = "", parsed = Err " " }
            , level = { value = "", parsed = Ok Nothing }
            , companyXp = { value = "", parsed = Ok Nothing }
            , totalXp = { value = "", parsed = Ok Nothing }
            , captcha = Nothing
            }
      , status = Init
      , companies = []
      , locations = []
      , titles = []
      }
    , cmd
    )


subscriptions : Sub Msg
subscriptions =
    Ports.captchaReceived (OnFieldChange Captcha)


update : Flags -> Msg -> Model -> ( Model, Cmd Msg )
update flags msg model =
    case msg of
        GotAllCompanies (Ok companies) ->
            ( { model | companies = companies }, Cmd.none )

        GotAllCompanies _ ->
            ( model, Cmd.none )

        GotAllLocations (Ok locations) ->
            ( { model | locations = locations }, Cmd.none )

        GotAllLocations _ ->
            ( model, Cmd.none )

        GotAllTitles (Ok titles) ->
            ( { model | titles = titles }, Cmd.none )

        GotAllTitles _ ->
            ( model, Cmd.none )

        Sent (Ok _) ->
            ( { model | status = Init }
            , Cmd.batch
                [ Companies.getAll flags GotAllCompanies
                , Locations.getAll flags GotAllLocations
                , Titles.getAll flags GotAllTitles
                , Notification.send NotificationMsg Notification.SalaryInserted
                ]
            )

        Sent _ ->
            ( { model | status = Init }
            , Notification.send NotificationMsg Notification.SalaryInsertingError
            )

        Send ->
            ( { model | status = Loading }
            , case body model.form of
                Nothing ->
                    Cmd.none

                Just b ->
                    Salaries.post flags Sent b
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

                        Company ->
                            { form
                                | company =
                                    { value = value
                                    , parsed = Company.tryFromString value
                                    }
                            }

                        Title ->
                            { form
                                | title =
                                    { value = value
                                    , parsed =
                                        if String.isEmpty value then
                                            Ok Nothing

                                        else
                                            Title.tryFromString value |> Result.map Just
                                    }
                            }

                        Location ->
                            { form
                                | location =
                                    { value = value
                                    , parsed = Location.tryFromString value
                                    }
                            }

                        Compensation ->
                            { form
                                | compensation =
                                    { value = value
                                    , parsed = Compensation.tryFromString value
                                    }
                            }

                        Level ->
                            { form
                                | level =
                                    { value = value
                                    , parsed =
                                        if String.isEmpty value then
                                            Ok Nothing

                                        else
                                            Level.tryFromString value |> Result.map Just
                                    }
                            }

                        CompanyXp ->
                            { form
                                | companyXp =
                                    { value = value
                                    , parsed =
                                        if String.isEmpty value then
                                            Ok Nothing

                                        else
                                            Xp.tryFromString value |> Result.map Just
                                    }
                            }

                        TotalXp ->
                            { form
                                | totalXp =
                                    { value = value
                                    , parsed =
                                        if String.isEmpty value then
                                            Ok Nothing

                                        else
                                            Xp.tryFromString value |> Result.map Just
                                    }
                            }

                        Captcha ->
                            { form | captcha = Just <| Captcha.fromString value }
            in
            ( { model | form = newForm, status = Init }, Cmd.none )


extractNotification : Msg -> Maybe Notification.Msg
extractNotification msg =
    case msg of
        NotificationMsg subMsg ->
            Just subMsg

        _ ->
            Nothing


view : Flags -> Model -> List (Html Msg)
view { hCaptchaKey } { form, status, companies, locations, titles } =
    [ Form.view
        { title = I18n.translate I18n.French I18n.IAddMySalary }
        [ Banner.view
            { text = I18n.translate I18n.French I18n.EmailExplanation }
        , Input.view
            { error = error form.email.parsed
            , label = I18n.translate I18n.French I18n.Email
            , sublabel = Nothing
            , onChange = OnFieldChange Email
            , placeholder = "moi@exemple.fr"
            , required = True
            , value = form.email.value
            }
        , Select.view
            { error = error form.company.parsed
            , id = "companies"
            , label = I18n.translate I18n.French I18n.Company
            , onChange = OnFieldChange Company
            , options = List.map Company.toString companies
            , placeholder = "Google"
            , required = True
            , value = form.company.value
            }
        , Select.view
            { error = error form.title.parsed
            , id = "titles"
            , label = I18n.translate I18n.French I18n.Title
            , options = List.map Title.toString titles
            , onChange = OnFieldChange Title
            , placeholder = I18n.translate I18n.French I18n.TitlePlaceholder
            , required = False
            , value = form.title.value
            }
        , Select.view
            { error = error form.location.parsed
            , id = "locations"
            , label = I18n.translate I18n.French I18n.Location
            , onChange = OnFieldChange Location
            , options = List.map Location.toString locations
            , placeholder = "Paris"
            , required = True
            , value = form.location.value
            }
        , Input.view
            { error = error form.compensation.parsed
            , label = I18n.translate I18n.French I18n.Compensation
            , sublabel = Just <| I18n.translate I18n.French I18n.CompensationHelp
            , onChange = OnFieldChange Compensation
            , placeholder = "40000"
            , required = True
            , value = form.compensation.value
            }
        , Input.view
            { error = error form.companyXp.parsed
            , label = I18n.translate I18n.French I18n.CompanyXp
            , sublabel = Just <| I18n.translate I18n.French I18n.InYears
            , onChange = OnFieldChange CompanyXp
            , placeholder = "2"
            , required = False
            , value = form.companyXp.value
            }
        , Input.view
            { error = error form.totalXp.parsed
            , label = I18n.translate I18n.French I18n.TotalXp
            , sublabel = Just <| I18n.translate I18n.French I18n.InYears
            , onChange = OnFieldChange TotalXp
            , placeholder = "10"
            , required = False
            , value = form.totalXp.value
            }
        , Input.view
            { error = error form.level.parsed
            , label = I18n.translate I18n.French I18n.Level
            , sublabel = Nothing
            , onChange = OnFieldChange Level
            , placeholder = "2"
            , required = False
            , value = form.level.value
            }
        , HCaptcha.view { key = hCaptchaKey }
        , Button.view
            { disabled = disabled status form
            , label =
                I18n.translate I18n.French <|
                    case status of
                        Init ->
                            I18n.Send

                        Loading ->
                            I18n.Sending
            , onClick = Send
            }
        ]
    ]


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
