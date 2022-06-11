module Design.Table exposing (..)

import Models.Company
import Models.CompanyXp
import Models.Compensation
import Models.Level
import Models.Location
import Models.Salary exposing (Salary)
import Models.Stock
import Models.TotalXp
import Utils


type Column
    = Company
    | Location
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
    Company


defaultDirection : Direction
defaultDirection =
    Asc


title : Column -> String
title column =
    case column of
        Company ->
            "Entreprise"

        Location ->
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
                Company ->
                    \s1 s2 -> Models.Company.compare s1.company s2.company

                Location ->
                    \s1 s2 -> Models.Location.compare s1.location s2.location

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
        ( Company, Company ) ->
            True

        ( Location, Location ) ->
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
