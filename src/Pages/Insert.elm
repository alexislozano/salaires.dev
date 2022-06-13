module Pages.Insert exposing (..)

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
    { form : Form }


type alias Form =
    { company : { value : String, error : Maybe String }
    , location : { value : String, error : Maybe String }
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
            { company = { value = "", error = Nothing }
            , location = { value = "", error = Nothing }
            , level = ""
            , companyXp = ""
            , totalXp = ""
            , compensation = ""
            , stock = ""
            }
      }
    , cmd
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotAllCompanies _ ->
            ( model, Cmd.none )

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
                            { form | company = { value = value, error = error } }

                        Location ->
                            let
                                error =
                                    case Location.tryNew value of
                                        Ok _ ->
                                            Nothing

                                        Err e ->
                                            Just e
                            in
                            { form | location = { value = value, error = error } }

                        _ ->
                            form
            in
            ( { model | form = newForm }, Cmd.none )


view : Model -> Element Msg
view { form } =
    Element.column
        [ Element.paddingXY 0 32
        , Element.centerX
        , Element.spacing 32
        ]
        [ Element.el [ Font.size 32, Element.paddingXY 0 16 ] <| Element.text "J'ajoute mon salaire"
        , Input.text
            [ Element.below
                (case form.company.error of
                    Just e ->
                        Element.text e

                    Nothing ->
                        Element.none
                )
            ]
            { label = Input.labelAbove [] <| Element.text "Entreprise"
            , onChange = OnFieldChange Company
            , placeholder = Just <| Input.placeholder [] <| Element.text "Google"
            , text = form.company.value
            }
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Localisation"
            , onChange = OnFieldChange Location
            , placeholder = Just <| Input.placeholder [] <| Element.text "Paris"
            , text = form.location.value
            }
        ]
