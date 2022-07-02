module Pages.Insert exposing (..)

import Design.Select as Select
import Element exposing (Element)
import Element.Font as Font
import Flags exposing (Flags)
import Http
import Models.Company as Company exposing (Company)
import Models.Location as Location exposing (Location)
import Services.Companies as Companies
import Services.Locations as Locations


type alias Model =
    { form : Form
    , companies : List Company
    , locations : List Location
    }


type alias Form =
    { company : { value : String, field : Select.Model, error : Maybe String }
    , location : { value : String, field : Select.Model, error : Maybe String }
    , level : String
    , companyXp : String
    , totalXp : String
    , compensation : String
    , stock : String
    }


type Field
    = Company
    | Location
    | Level
    | CompanyXp
    | TotalXp
    | Compensation
    | Stock


type X
    = Optional
    | Required


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
            { company = { value = "", field = Select.init, error = Nothing }
            , location = { value = "", field = Select.init, error = Nothing }
            , level = ""
            , companyXp = ""
            , totalXp = ""
            , compensation = ""
            , stock = ""
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
                            let
                                error =
                                    case Company.tryNew value of
                                        Ok _ ->
                                            Nothing

                                        Err e ->
                                            Just e
                            in
                            { form
                                | company =
                                    { value = value
                                    , field = form.company.field
                                    , error = error
                                    }
                            }

                        Location ->
                            let
                                error =
                                    case Location.tryNew value of
                                        Ok _ ->
                                            Nothing

                                        Err e ->
                                            Just e
                            in
                            { form
                                | location =
                                    { value = value
                                    , field = form.location.field
                                    , error = error
                                    }
                            }

                        _ ->
                            form
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
                                    , field = Select.update subMsg form.company.field
                                    , error = form.company.error
                                    }
                            }

                        Location ->
                            { form
                                | location =
                                    { value = form.location.value
                                    , field = Select.update subMsg form.location.field
                                    , error = form.location.error
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
        , Element.spacing 32
        ]
        [ Element.el [ Font.size 32, Element.paddingXY 0 16 ] <| Element.text "J'ajoute mon salaire"
        , Select.view
            form.company.field
            { label = "Entreprise"
            , onChange = OnFieldChange Company
            , options = List.map Company.toString companies
            , toMsg = SelectMsg Company
            , placeholder = "Google"
            , value = form.company.value
            }
        , Select.view
            form.location.field
            { label = "Localisation"
            , onChange = OnFieldChange Location
            , options = List.map Location.toString locations
            , toMsg = SelectMsg Location
            , placeholder = "Paris"
            , value = form.location.value
            }
        ]
