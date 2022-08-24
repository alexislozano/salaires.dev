module Pages.Index exposing (..)

import Css
import Design.Palette as Palette
import Design.Table as Table
import Flags exposing (Flags)
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import Html.Styled.Events as Events
import Http
import I18n
import Models.Company as Company
import Models.Compensation as Compensation
import Models.Date as Date
import Models.Level as Level
import Models.Location as Location
import Models.Salary as Salary exposing (Salary)
import Models.Stock as Stock
import Models.Title as Title
import Models.Xp as Xp
import Result exposing (Result(..))
import Services.Salaries as Salaries
import Utils exposing (HttpData(..))


type alias Model =
    { salaries : HttpData (List Salary)
    , sort : Sort
    }


type alias Sort =
    { column : Table.Column
    , direction : Table.Direction
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


view : Model -> List (Html Msg)
view model =
    case model.salaries of
        Success salaries ->
            [ Html.table
                [ Attributes.css
                    [ Css.borderSpacing Css.zero
                    , Css.width (Css.pct 100)
                    ]
                ]
                [ head model
                , salaries
                    |> Table.sort model.sort
                    |> body
                ]
            ]

        _ ->
            [ Html.div
                [ Attributes.css
                    [ Css.displayFlex
                    , Css.height (Css.pct 100)
                    , Css.alignItems Css.center
                    , Css.justifyContent Css.center
                    ]
                ]
                [ Html.text (I18n.translate I18n.French I18n.LoadingData) ]
            ]


head : Model -> Html Msg
head { sort } =
    Html.thead
        [ Attributes.css
            [ Css.position Css.sticky
            , Css.top Css.zero
            ]
        ]
        [ Html.tr []
            [ Table.Company
                |> Table.header sort
                |> header Table.Company
            , Table.Title
                |> Table.header sort
                |> header Table.Title
            , Table.Location
                |> Table.header sort
                |> header Table.Location
            , Table.Compensation
                |> Table.header sort
                |> header Table.Compensation
            , Table.Stock
                |> Table.header sort
                |> header Table.Stock
            , Table.CompanyXp
                |> Table.header sort
                |> header Table.CompanyXp
            , Table.TotalXp
                |> Table.header sort
                |> header Table.TotalXp
            , Table.Level
                |> Table.header sort
                |> header Table.Level
            , Table.Date
                |> Table.header sort
                |> header Table.Date
            ]
        ]


header : Table.Column -> { title : String, subtitle : String } -> Html Msg
header column { title, subtitle } =
    Html.th
        [ Attributes.css
            [ Css.padding Css.zero
            , Css.backgroundColor Palette.sand
            , Css.borderBottom2 (Css.px 2) Css.solid
            ]
        ]
        [ Html.button
            [ Events.onClick <| Clicked column
            , Attributes.css
                [ Css.height (Css.px 48)
                , Css.width (Css.pct 100)
                , Css.border Css.zero
                , Css.backgroundColor Css.transparent
                , Css.padding2 (Css.px 8) (Css.px 16)
                , Css.textAlign Css.start
                , Css.cursor Css.pointer
                , Css.fontSize Css.inherit
                , Css.fontFamily Css.inherit
                , Css.whiteSpace Css.noWrap
                , Css.displayFlex
                , Css.flexDirection Css.column
                ]
            ]
            [ Html.span
                [ Attributes.css
                    [ Css.fontWeight Css.bold
                    ]
                ]
                [ Html.text title ]
            , Html.span
                [ Attributes.css
                    [ Css.fontSize (Css.px 12)
                    ]
                ]
                [ Html.text subtitle ]
            ]
        ]


body : List Salary -> Html Msg
body salaries =
    salaries
        |> List.indexedMap row
        |> Html.tbody []


row : Int -> Salary -> Html Msg
row index salary =
    Html.tr
        [ Attributes.css
            [ Css.backgroundColor <|
                if modBy 2 index == 0 then
                    Palette.lightSand

                else
                    Palette.sand
            ]
        ]
        [ Salary.toFields salary
            |> .company
            |> Company.toString
            |> cell
        , Salary.toFields salary
            |> .title
            |> Maybe.map Title.toString
            |> Maybe.withDefault ""
            |> cell
        , Salary.toFields salary
            |> .location
            |> Location.toString
            |> cell
        , Salary.toFields salary
            |> .compensation
            |> Compensation.toString
            |> cell
        , Salary.toFields salary
            |> .stock
            |> Maybe.map Stock.toString
            |> Maybe.withDefault ""
            |> cell
        , Salary.toFields salary
            |> .companyXp
            |> Maybe.map Xp.toString
            |> Maybe.withDefault ""
            |> cell
        , Salary.toFields salary
            |> .totalXp
            |> Maybe.map Xp.toString
            |> Maybe.withDefault ""
            |> cell
        , Salary.toFields salary
            |> .level
            |> Maybe.map Level.toString
            |> Maybe.withDefault ""
            |> cell
        , Salary.toFields salary
            |> .date
            |> Date.toString
            |> cell
        ]


cell : String -> Html msg
cell value =
    Html.td
        [ Attributes.css
            [ Css.height (Css.px 48)
            , Css.padding2 Css.zero (Css.px 16)
            , Css.whiteSpace Css.noWrap
            ]
        ]
        [ Html.text value ]
