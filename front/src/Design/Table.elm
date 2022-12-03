module Design.Table exposing (..)

import I18n
import Models.Company
import Models.Compensation
import Models.Date
import Models.Level
import Models.Location
import Models.Salary as Salary exposing (Salary(..))
import Models.Title
import Models.Xp
import Utils


type Column
    = Company
    | Location
    | Level
    | Title
    | CompanyXp
    | TotalXp
    | Compensation
    | Date


type Direction
    = Asc
    | Desc


defaultColumn : Column
defaultColumn =
    Date


defaultDirection : Direction
defaultDirection =
    Desc


title : Column -> String
title column =
    case column of
        Company ->
            I18n.translate I18n.French I18n.Company

        Location ->
            I18n.translate I18n.French I18n.Location

        Level ->
            I18n.translate I18n.French I18n.Level

        Title ->
            I18n.translate I18n.French I18n.Title

        CompanyXp ->
            I18n.translate I18n.French I18n.CompanyXp

        TotalXp ->
            I18n.translate I18n.French I18n.TotalXp

        Compensation ->
            I18n.translate I18n.French I18n.Compensation

        Date ->
            I18n.translate I18n.French I18n.Date


subtitle : Column -> String
subtitle column =
    case column of
        Company ->
            ""

        Location ->
            ""

        Level ->
            ""

        Title ->
            ""

        CompanyXp ->
            I18n.translate I18n.French I18n.InYears

        TotalXp ->
            I18n.translate I18n.French I18n.InYears

        Compensation ->
            I18n.translate I18n.French I18n.CompensationHelp

        Date ->
            ""


header : { column : Column, direction : Direction } -> Column -> { title : String, subtitle : String }
header { column, direction } c =
    let
        displayArrow =
            equal column c
    in
    { title =
        title c
            ++ (if displayArrow then
                    case direction of
                        Asc ->
                            "  ↑"

                        Desc ->
                            "  ↓"

                else
                    "   "
               )
    , subtitle = subtitle c
    }


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

                Title ->
                    \s1 s2 -> Utils.compareMaybe Models.Title.compare s1.title s2.title

                CompanyXp ->
                    \s1 s2 -> Utils.compareMaybe Models.Xp.compare s1.companyXp s2.companyXp

                TotalXp ->
                    \s1 s2 -> Utils.compareMaybe Models.Xp.compare s1.totalXp s2.totalXp

                Compensation ->
                    \s1 s2 -> Models.Compensation.compare s1.compensation s2.compensation

                Date ->
                    \s1 s2 -> Models.Date.compare s1.date s2.date
    in
    salaries
        |> List.map Salary.toFields
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
        |> List.map Salary


equal : Column -> Column -> Bool
equal c1 c2 =
    case ( c1, c2 ) of
        ( Company, Company ) ->
            True

        ( Location, Location ) ->
            True

        ( Level, Level ) ->
            True

        ( Title, Title ) ->
            True

        ( CompanyXp, CompanyXp ) ->
            True

        ( TotalXp, TotalXp ) ->
            True

        ( Compensation, Compensation ) ->
            True

        ( Date, Date ) ->
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
