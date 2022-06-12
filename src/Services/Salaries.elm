module Services.Salaries exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode exposing (Decoder, andThen, list, maybe, succeed)
import Json.Decode.Pipeline exposing (required)
import Models.Company as Company exposing (Company)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Date as Date exposing (Date)
import Models.Level as Level exposing (Level)
import Models.Location as Location exposing (Location)
import Models.Salary as Salary exposing (Salary)
import Models.Stock as Stock exposing (Stock)
import Models.Xp as Xp exposing (Xp)
import Services.Supabase as Supabase
import Utils


getAll : Flags -> (Result Http.Error (List Salary) -> msg) -> Cmd msg
getAll flags msg =
    Http.request
        { method = "GET"
        , headers = Supabase.headers flags
        , url = Supabase.getAllSalariesUrl flags
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg decoder
        }


type alias Response =
    { company : Company
    , location : Location
    , compensation : Compensation
    , date : Date
    , stock : Maybe Stock
    , level : Maybe Level
    , companyXp : Maybe Xp
    , totalXp : Maybe Xp
    }


toSalaryDecoder : Response -> Decoder Salary
toSalaryDecoder response =
    Salary.tryNew response |> Utils.resultDecoder


decoder : Decoder (List Salary)
decoder =
    succeed Response
        |> required "company" Company.decoder
        |> required "location" Location.decoder
        |> required "compensation" Compensation.decoder
        |> required "date" Date.decoder
        |> required "stock" (maybe Stock.decoder)
        |> required "level" (maybe Level.decoder)
        |> required "company_xp" (maybe Xp.decoder)
        |> required "total_xp" (maybe Xp.decoder)
        |> andThen toSalaryDecoder
        |> list
