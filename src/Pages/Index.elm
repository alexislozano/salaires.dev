module Pages.Index exposing (..)

import Design.Table as Table
import Element exposing (Element)
import Element.Font as Font
import Element.Input as Input
import Flags exposing (Flags)
import Http
import Models.Company as Company
import Models.CompanyXp as CompanyXp
import Models.Compensation as Compensation
import Models.Date as Date
import Models.Level as Level
import Models.Location as Location
import Models.Salary exposing (Salary)
import Models.Stock as Stock
import Models.TotalXp as TotalXp
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
            Element.table [ Element.paddingXY 0 8 ]
                { data = salaries |> Table.sort model.sort
                , columns =
                    [ { header =
                            Table.Company
                                |> Table.header model.sort
                                |> header Table.Company
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.company
                                    |> Company.toString
                                    |> cell
                      }
                    , { header =
                            Table.Location
                                |> Table.header model.sort
                                |> header Table.Location
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.location
                                    |> Location.toString
                                    |> cell
                      }
                    , { header =
                            Table.Level
                                |> Table.header model.sort
                                |> header Table.Level
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.level
                                    |> Maybe.map Level.toString
                                    |> Maybe.withDefault ""
                                    |> cell
                      }
                    , { header =
                            Table.CompanyXp
                                |> Table.header model.sort
                                |> header Table.CompanyXp
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.companyXp
                                    |> Maybe.map CompanyXp.toString
                                    |> Maybe.withDefault ""
                                    |> cell
                      }
                    , { header =
                            Table.TotalXp
                                |> Table.header model.sort
                                |> header Table.TotalXp
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.totalXp
                                    |> Maybe.map TotalXp.toString
                                    |> Maybe.withDefault ""
                                    |> cell
                      }
                    , { header =
                            Table.Compensation
                                |> Table.header model.sort
                                |> header Table.Compensation
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.compensation
                                    |> Compensation.toString
                                    |> cell
                      }
                    , { header =
                            Table.Stock
                                |> Table.header model.sort
                                |> header Table.Stock
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.stock
                                    |> Maybe.map Stock.toString
                                    |> Maybe.withDefault ""
                                    |> cell
                      }
                    , { header =
                            Table.Date
                                |> Table.header model.sort
                                |> header Table.Date
                      , width = Element.fill
                      , view =
                            \salary ->
                                salary.date
                                    |> Date.toString
                                    |> cell
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
        [ Element.padding 16
        , Font.bold
        ]
        { onPress = Just <| Clicked column
        , label = Element.text title
        }


cell : String -> Element msg
cell text =
    Element.el
        [ Element.padding 16
        ]
    <|
        Element.text text
