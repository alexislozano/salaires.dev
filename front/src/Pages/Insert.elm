module Pages.Insert exposing (..)

import Design.Button as Button
import Design.Form as Form
import Design.Input as Input
import Design.Select as Select
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Http
import I18n
import Models.Company as Company exposing (Company)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Level as Level exposing (Level)
import Models.Location as Location exposing (Location)
import Models.Stock as Stock exposing (Stock)
import Models.Title as Title exposing (Title)
import Models.Token as Token exposing (Token)
import Models.Xp as Xp exposing (Xp)
import Notification
import Services.Companies as Companies
import Services.Locations as Locations
import Services.Salaries as Salaries


type alias Model =
    { form : Form
    , status : Status
    , companies : List Company
    , locations : List Location
    }


type alias Form =
    { token : { value : String, parsed : Result String Token }
    , company : { value : String, parsed : Result String Company }
    , location : { value : String, parsed : Result String Location }
    , compensation : { value : String, parsed : Result String Compensation }
    , stock : { value : String, parsed : Result String (Maybe Stock) }
    , level : { value : String, parsed : Result String (Maybe Level) }
    , companyXp : { value : String, parsed : Result String (Maybe Xp) }
    , totalXp : { value : String, parsed : Result String (Maybe Xp) }
    , title : { value : String, parsed : Result String (Maybe Title) }
    }


body : Form -> Maybe Salaries.Body
body form =
    case ( form.company.parsed, form.location.parsed, form.compensation.parsed ) of
        ( Ok company, Ok location, Ok compensation ) ->
            case ( form.stock.parsed, form.level.parsed, form.companyXp.parsed ) of
                ( Ok stock, Ok level, Ok companyXp ) ->
                    case ( form.totalXp.parsed, form.title.parsed, form.token.parsed ) of
                        ( Ok totalXp, Ok title, Ok token ) ->
                            Just
                                { company = company
                                , location = location
                                , compensation = compensation
                                , token = token
                                , stock = stock
                                , level = level
                                , companyXp = companyXp
                                , totalXp = totalXp
                                , title = title
                                }

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
    = Company
    | Location
    | Compensation
    | Token
    | Stock
    | CompanyXp
    | TotalXp
    | Level
    | Title


type Msg
    = GotAllCompanies (Result Http.Error (List Company))
    | GotAllLocations (Result Http.Error (List Location))
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
                ]
    in
    ( { form =
            { company = { value = "", parsed = Err " " }
            , location = { value = "", parsed = Err " " }
            , compensation = { value = "", parsed = Err " " }
            , token = { value = "", parsed = Err " " }
            , level = { value = "", parsed = Ok Nothing }
            , companyXp = { value = "", parsed = Ok Nothing }
            , totalXp = { value = "", parsed = Ok Nothing }
            , stock = { value = "", parsed = Ok Nothing }
            , title = { value = "", parsed = Ok Nothing }
            }
      , status = Init
      , companies = []
      , locations = []
      }
    , cmd
    )


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

        Sent (Ok _) ->
            ( { model | status = Init }
            , Cmd.batch
                [ Companies.getAll flags GotAllCompanies
                , Locations.getAll flags GotAllLocations
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
                        Company ->
                            { form
                                | company =
                                    { value = value
                                    , parsed = Company.tryFromString value
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

                        Token ->
                            { form
                                | token =
                                    { value = value
                                    , parsed = Token.tryFromString value
                                    }
                            }

                        Stock ->
                            { form
                                | stock =
                                    { value = value
                                    , parsed =
                                        if String.isEmpty value then
                                            Ok Nothing

                                        else
                                            Stock.tryFromString value |> Result.map Just
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
view { form, status, companies, locations } =
    [ Form.view
        { title = I18n.translate I18n.French I18n.IAddMySalary }
        [ Input.view
            { error = error form.token.parsed
            , label = I18n.translate I18n.French I18n.Token
            , sublabel = Nothing
            , onChange = OnFieldChange Token
            , placeholder = "123456"
            , required = True
            , value = form.token.value
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
            { error = error form.stock.parsed
            , label = I18n.translate I18n.French I18n.Stock
            , sublabel = Just <| I18n.translate I18n.French I18n.InEuros
            , onChange = OnFieldChange Stock
            , placeholder = "10000"
            , required = False
            , value = form.stock.value
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
            { error = error form.title.parsed
            , label = I18n.translate I18n.French I18n.Title
            , sublabel = Nothing
            , onChange = OnFieldChange Title
            , placeholder = "Fullstack Developer"
            , required = False
            , value = form.title.value
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
