module Pages.Index exposing (..)

import Design.Palette as Palette
import Design.Table as Table
import Element exposing (Element)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input
import Flags exposing (Flags)
import Http
import Models.Company as Company
import Models.Compensation as Compensation
import Models.Date as Date
import Models.Level as Level
import Models.Location as Location
import Models.Salary as Salary exposing (Salary)
import Models.Stock as Stock
import Models.Xp as Xp
import Result exposing (Result(..))
import Services.Salaries as Salaries
import Utils exposing (HttpData(..))


type alias Model =
    { salaries : HttpData (List Salary)
    , sort :
        { column : Table.Column
        , direction : Table.Direction
        }
    }


type Msg
    = GotAllSalaries (Result Http.Error (List Salary))
    | Clicked Table.Column


init : Flags -> ( Model, Cmd Msg )
init flags =
    let
        cmd =
            Salaries.getAll flags GotAllSalaries
    in
    ( { salaries = Loading
      , sort =
            { column = Table.defaultColumn
            , direction = Table.defaultDirection
            }
      }
    , cmd
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotAllSalaries (Ok salaries) ->
            ( { model | salaries = Success salaries }, Cmd.none )

        GotAllSalaries (Err _) ->
            ( { model | salaries = Failure }, Cmd.none )

        Clicked column ->
            let
                sort =
                    { column = column
                    , direction =
                        if Table.equal column model.sort.column then
                            Table.toggle model.sort.direction

                        else
                            Table.defaultDirection
                    }
            in
            ( { model | sort = sort }, Cmd.none )


view : Model -> Element Msg
view model =
    table model


table : Model -> Element Msg
table model =
    case model.salaries of
        Success salaries ->
            Element.indexedTable []
                { data = salaries |> Table.sort model.sort
                , columns =
                    [ { header =
                            Table.Company
                                |> Table.header model.sort
                                |> header Table.Company
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .company
                                    |> Company.toString
                                    |> cell index
                      }
                    , { header =
                            Table.Location
                                |> Table.header model.sort
                                |> header Table.Location
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .location
                                    |> Location.toString
                                    |> cell index
                      }
                    , { header =
                            Table.Level
                                |> Table.header model.sort
                                |> header Table.Level
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .level
                                    |> Maybe.map Level.toString
                                    |> Maybe.withDefault ""
                                    |> cell index
                      }
                    , { header =
                            Table.CompanyXp
                                |> Table.header model.sort
                                |> header Table.CompanyXp
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .companyXp
                                    |> Maybe.map Xp.toString
                                    |> Maybe.withDefault ""
                                    |> cell index
                      }
                    , { header =
                            Table.TotalXp
                                |> Table.header model.sort
                                |> header Table.TotalXp
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .totalXp
                                    |> Maybe.map Xp.toString
                                    |> Maybe.withDefault ""
                                    |> cell index
                      }
                    , { header =
                            Table.Compensation
                                |> Table.header model.sort
                                |> header Table.Compensation
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .compensation
                                    |> Compensation.toString
                                    |> cell index
                      }
                    , { header =
                            Table.Stock
                                |> Table.header model.sort
                                |> header Table.Stock
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .stock
                                    |> Maybe.map Stock.toString
                                    |> Maybe.withDefault ""
                                    |> cell index
                      }
                    , { header =
                            Table.Date
                                |> Table.header model.sort
                                |> header Table.Date
                      , width = Element.fill
                      , view =
                            \index salary ->
                                Salary.toFields salary
                                    |> .date
                                    |> Date.toString
                                    |> cell index
                      }
                    ]
                }

        Loading ->
            Element.none

        Failure ->
            Element.none


header : Table.Column -> String -> Element Msg
header column title =
    Input.button
        [ Element.height <| Element.px 48
        , Element.paddingXY 16 0
        , Font.bold
        , Border.widthEach { top = 0, right = 0, bottom = 2, left = 0 }
        ]
        { onPress = Just <| Clicked column
        , label = Element.text title
        }


cell : Int -> String -> Element msg
cell index text =
    Element.el
        [ Element.height <| Element.px 48
        , Element.paddingXY 16 0
        , Background.color <|
            if modBy 2 index == 0 then
                Palette.lightSand

            else
                Palette.sand
        ]
    <|
        Element.el [ Element.centerY ] <|
            Element.text text
