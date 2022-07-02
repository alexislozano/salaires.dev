module Pages.Insert exposing (..)

import Design.Input as Input
import Design.Select as Select
import Element exposing (Element)
import Element.Font as Font
import Flags exposing (Flags)
import Html.Attributes exposing (value)
import Http
import I18n
import Models.Company as Company exposing (Company)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Level as Level exposing (Level)
import Models.Location as Location exposing (Location)
import Models.Stock as Stock exposing (Stock)
import Models.Xp as Xp exposing (Xp)
import Services.Companies as Companies
import Services.Locations as Locations


type alias Model =
    { form : Form
    , companies : List Company
    , locations : List Location
    }


type FieldState a
    = Empty
    | Filled (Result String a)


type alias Form =
    { company : { value : String, state : FieldState Company, field : Select.Model }
    , location : { value : String, state : FieldState Location, field : Select.Model }
    , compensation : { value : String, state : FieldState Compensation }
    , stock : { value : String, state : FieldState Stock }
    , level : { value : String, state : FieldState Level }
    , companyXp : { value : String, state : FieldState Xp }
    , totalXp : { value : String, state : FieldState Xp }
    }


type Field
    = Company
    | Location
    | Level
    | CompanyXp
    | TotalXp
    | Compensation
    | Stock


type Msg
    = GotAllCompanies (Result Http.Error (List Company))
    | GotAllLocations (Result Http.Error (List Location))
    | OnFieldChange Field String
    | SelectMsg Field Select.Msg


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
            { company = { value = "", state = Empty, field = Select.init }
            , location = { value = "", state = Empty, field = Select.init }
            , level = { value = "", state = Empty }
            , companyXp = { value = "", state = Empty }
            , totalXp = { value = "", state = Empty }
            , compensation = { value = "", state = Empty }
            , stock = { value = "", state = Empty }
            }
      , companies = []
      , locations = []
      }
    , cmd
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotAllCompanies (Ok companies) ->
            ( { model | companies = companies }, Cmd.none )

        GotAllCompanies _ ->
            ( model, Cmd.none )

        GotAllLocations (Ok locations) ->
            ( { model | locations = locations }, Cmd.none )

        GotAllLocations _ ->
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
                                    , state = Filled (Company.tryFromString value)
                                    , field = form.company.field
                                    }
                            }

                        Location ->
                            { form
                                | location =
                                    { value = value
                                    , state = Filled (Location.tryFromString value)
                                    , field = form.location.field
                                    }
                            }

                        Compensation ->
                            { form
                                | compensation =
                                    { value = value
                                    , state = Filled (Compensation.tryFromString value)
                                    }
                            }

                        Stock ->
                            { form
                                | stock =
                                    { value = value
                                    , state =
                                        if String.isEmpty value then
                                            Empty

                                        else
                                            Filled (Stock.tryFromString value)
                                    }
                            }

                        Level ->
                            { form
                                | level =
                                    { value = value
                                    , state =
                                        if String.isEmpty value then
                                            Empty

                                        else
                                            Filled (Level.tryFromString value)
                                    }
                            }

                        CompanyXp ->
                            { form
                                | companyXp =
                                    { value = value
                                    , state =
                                        if String.isEmpty value then
                                            Empty

                                        else
                                            Filled (Xp.tryFromString value)
                                    }
                            }

                        TotalXp ->
                            { form
                                | totalXp =
                                    { value = value
                                    , state =
                                        if String.isEmpty value then
                                            Empty

                                        else
                                            Filled (Xp.tryFromString value)
                                    }
                            }
            in
            ( { model | form = newForm }, Cmd.none )

        SelectMsg field subMsg ->
            let
                form =
                    model.form

                newForm =
                    case field of
                        Company ->
                            { form
                                | company =
                                    { value = form.company.value
                                    , state = form.company.state
                                    , field = Select.update subMsg form.company.field
                                    }
                            }

                        Location ->
                            { form
                                | location =
                                    { value = form.location.value
                                    , state = form.location.state
                                    , field = Select.update subMsg form.location.field
                                    }
                            }

                        _ ->
                            form
            in
            ( { model | form = newForm }, Cmd.none )


view : Model -> Element Msg
view { form, companies, locations } =
    Element.column
        [ Element.paddingXY 0 32
        , Element.centerX
        , Element.spacing 16
        ]
        [ Element.el [ Font.size 32, Element.paddingXY 0 16 ] <| Element.text (I18n.translate I18n.French I18n.IAddMySalary)
        , Select.view
            form.company.field
            { error = error form.company.state
            , label = I18n.translate I18n.French I18n.Company
            , onChange = OnFieldChange Company
            , options = List.map Company.toString companies
            , placeholder = "Google"
            , required = True
            , toMsg = SelectMsg Company
            , value = form.company.value
            }
        , Select.view
            form.location.field
            { error = error form.location.state
            , label = I18n.translate I18n.French I18n.Location
            , onChange = OnFieldChange Location
            , options = List.map Location.toString locations
            , placeholder = "Paris"
            , required = True
            , toMsg = SelectMsg Location
            , value = form.location.value
            }
        , Input.view
            { error = error form.compensation.state
            , label = I18n.translate I18n.French I18n.Compensation
            , onChange = OnFieldChange Compensation
            , placeholder = "40000"
            , required = True
            , value = form.compensation.value
            }
        , Input.view
            { error = error form.stock.state
            , label = I18n.translate I18n.French I18n.Stock
            , onChange = OnFieldChange Stock
            , placeholder = "10000"
            , required = False
            , value = form.stock.value
            }
        , Input.view
            { error = error form.level.state
            , label = I18n.translate I18n.French I18n.Level
            , onChange = OnFieldChange Level
            , placeholder = "2"
            , required = False
            , value = form.level.value
            }
        , Input.view
            { error = error form.companyXp.state
            , label = I18n.translate I18n.French I18n.CompanyXp
            , onChange = OnFieldChange CompanyXp
            , placeholder = "2"
            , required = False
            , value = form.companyXp.value
            }
        , Input.view
            { error = error form.totalXp.state
            , label = I18n.translate I18n.French I18n.TotalXp
            , onChange = OnFieldChange TotalXp
            , placeholder = "10"
            , required = False
            , value = form.totalXp.value
            }
        ]


error : FieldState a -> String
error state =
    case state of
        Filled (Err e) ->
            e

        _ ->
            " "
