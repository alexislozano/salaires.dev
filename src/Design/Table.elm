module Design.Table exposing (..)

import Models.CompanyName
import Models.CompanyXp
import Models.Compensation
import Models.Level
import Models.LocationName
import Models.Salary exposing (Salary)
import Models.Stock
import Models.TotalXp
import Utils


type Column
    = CompanyName
    | LocationName
    | Level
    | CompanyXp
    | TotalXp
    | Compensation
    | Stock


type Direction
    = Asc
    | Desc


defaultColumn : Column
defaultColumn =
    CompanyName


defaultDirection : Direction
defaultDirection =
    Asc


title : Column -> String
title column =
    case column of
        CompanyName ->
            "Entreprise"

        LocationName ->
            "Localisation"

        Level ->
            "Niveau"

        CompanyXp ->
            "Expérience entreprise"

        TotalXp ->
            "Expérience totale"

        Compensation ->
            "Compensation"

        Stock ->
            "Stock"


header : { column : Column, direction : Direction } -> Column -> String
header { column, direction } c =
    let
        displayArrow =
            equal column c
    in
    title c
        ++ (if displayArrow then
                case direction of
                    Asc ->
                        "  ↑"

                    Desc ->
                        "  ↓"

            else
                ""
           )


sort :
    { column : Column
    , direction : Direction
    }
    -> List Salary
    -> List Salary
sort { column, direction } salaries =
    let
        sortFn =
            case column of
                CompanyName ->
                    \s1 s2 -> Models.CompanyName.compare s1.companyName s2.companyName

                LocationName ->
                    \s1 s2 -> Models.LocationName.compare s1.locationName s2.locationName

                Level ->
                    \s1 s2 -> Utils.compareMaybe Models.Level.compare s1.level s2.level

                CompanyXp ->
                    \s1 s2 -> Utils.compareMaybe Models.CompanyXp.compare s1.companyXp s2.companyXp

                TotalXp ->
                    \s1 s2 -> Utils.compareMaybe Models.TotalXp.compare s1.totalXp s2.totalXp

                Compensation ->
                    \s1 s2 -> Models.Compensation.compare s1.compensation s2.compensation

                Stock ->
                    \s1 s2 -> Utils.compareMaybe Models.Stock.compare s1.stock s2.stock
    in
    salaries
        |> (case direction of
                Asc ->
                    List.sortWith sortFn

                Desc ->
                    List.sortWith
                        (\s1 s2 ->
                            case sortFn s1 s2 of
                                LT ->
                                    GT

                                EQ ->
                                    EQ

                                GT ->
                                    LT
                        )
           )


equal : Column -> Column -> Bool
equal c1 c2 =
    case ( c1, c2 ) of
        ( CompanyName, CompanyName ) ->
            True

        ( LocationName, LocationName ) ->
            True

        ( Level, Level ) ->
            True

        ( CompanyXp, CompanyXp ) ->
            True

        ( TotalXp, TotalXp ) ->
            True

        ( Compensation, Compensation ) ->
            True

        ( Stock, Stock ) ->
            True

        _ ->
            False


toggle : Direction -> Direction
toggle direction =
    case direction of
        Asc ->
            Desc

        Desc ->
            Asc
