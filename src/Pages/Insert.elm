module Pages.Insert exposing (..)

import Element exposing (Element)
import Element.Font as Font
import Element.Input as Input
import Flags exposing (Flags)
import Http
import Models.Company exposing (Company)
import Models.Location exposing (Location)
import Services.Companies as Companies
import Services.Locations as Locations


type alias Model =
    { form : Form }


type alias Form =
    { company : String
    , location : String
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
            { company = ""
            , location = ""
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
                            { form | company = value }

                        Location ->
                            { form | location = value }

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
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Entreprise"
            , onChange = OnFieldChange Company
            , placeholder = Just <| Input.placeholder [] <| Element.text "Google"
            , text = form.company
            }
        , Input.text []
            { label = Input.labelAbove [] <| Element.text "Localisation"
            , onChange = OnFieldChange Location
            , placeholder = Just <| Input.placeholder [] <| Element.text "Paris"
            , text = form.location
            }
        ]
