module Pages.Insert exposing (..)

import Design.Select as Select
import Element exposing (Element)
import Element.Font as Font
import Element.Input as Input
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
    { company : { value : String, field : Select.Model }
    , location : { value : String, field : Select.Model }
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
            { company = { value = "", field = Select.init }
            , location = { value = "", field = Select.init }
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
                            { form
                                | company =
                                    { value = value
                                    , field = form.company.field
                                    }
                            }

                        Location ->
                            { form
                                | location =
                                    { value = value
                                    , field = form.location.field
                                    }
                            }

                        Compensation ->
                            { form | compensation = value }

                        Stock ->
                            { form | stock = value }

                        Level ->
                            { form | level = value }

                        CompanyXp ->
                            { form | companyXp = value }

                        TotalXp ->
                            { form | totalXp = value }
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
                                    }
                            }

                        Location ->
                            { form
                                | location =
                                    { value = form.location.value
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
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Compensation"
            , onChange = OnFieldChange Compensation
            , placeholder = Just <| Input.placeholder [] <| Element.text "40000"
            , text = form.compensation
            }
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Stock"
            , onChange = OnFieldChange Stock
            , placeholder = Just <| Input.placeholder [] <| Element.text "10000"
            , text = form.stock
            }
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Niveau"
            , onChange = OnFieldChange Level
            , placeholder = Just <| Input.placeholder [] <| Element.text "2"
            , text = form.level
            }
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Expérience entreprise"
            , onChange = OnFieldChange CompanyXp
            , placeholder = Just <| Input.placeholder [] <| Element.text "2"
            , text = form.companyXp
            }
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Expérience totale"
            , onChange = OnFieldChange TotalXp
            , placeholder = Just <| Input.placeholder [] <| Element.text "10"
            , text = form.totalXp
            }
        ]
