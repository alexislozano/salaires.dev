module Services.Salaries exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode as Decode exposing (Decoder, andThen, list, maybe, succeed)
import Json.Decode.Pipeline exposing (required)
import Json.Encode exposing (Value, null, object)
import Models.Captcha as Captcha exposing (Captcha)
import Models.Company as Company exposing (Company)
import Models.Compensation as Compensation exposing (Compensation)
import Models.Date as Date exposing (Date)
import Models.Level as Level exposing (Level)
import Models.Location as Location exposing (Location)
import Models.Salary as Salary exposing (Salary)
import Models.Stock as Stock exposing (Stock)
import Models.Title as Title exposing (Title)
import Models.Xp as Xp exposing (Xp)


getAll : Flags -> (Result Http.Error (List Salary) -> msg) -> Cmd msg
getAll flags msg =
    Http.request
        { method = "GET"
        , headers = []
        , url = flags.apiUrl ++ "/salaries"
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg (list decoder)
        }


post : Flags -> (Result Http.Error () -> msg) -> Body -> Cmd msg
post flags msg body =
    Http.request
        { method = "POST"
        , headers = []
        , url = flags.apiUrl ++ "/salaries"
        , body = Http.jsonBody <| encode body
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectWhatever msg
        }


type alias Body =
    { company : Company
    , location : Location
    , compensation : Compensation
    , stock : Maybe Stock
    , level : Maybe Level
    , companyXp : Maybe Xp
    , totalXp : Maybe Xp
    , title : Maybe Title
    , captcha : Captcha
    }


encode : Body -> Value
encode body =
    object
        [ ( "company", Company.encode body.company )
        , ( "title", body.title |> Maybe.map Title.encode |> Maybe.withDefault null )
        , ( "location", Location.encode body.location )
        , ( "compensation", Compensation.encode body.compensation )
        , ( "stock", body.stock |> Maybe.map Stock.encode |> Maybe.withDefault null )
        , ( "level", body.level |> Maybe.map Level.encode |> Maybe.withDefault null )
        , ( "company_xp", body.companyXp |> Maybe.map Xp.encode |> Maybe.withDefault null )
        , ( "total_xp", body.totalXp |> Maybe.map Xp.encode |> Maybe.withDefault null )
        , ( "captcha", Captcha.encode body.captcha )
        ]


type alias Response =
    { company : Company
    , title : Maybe Title
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
    Salary.new response |> Decode.succeed


decoder : Decoder Salary
decoder =
    succeed Response
        |> required "company" Company.decoder
        |> required "title" (maybe Title.decoder)
        |> required "location" Location.decoder
        |> required "compensation" Compensation.decoder
        |> required "date" Date.decoder
        |> required "stock" (maybe Stock.decoder)
        |> required "level" (maybe Level.decoder)
        |> required "company_xp" (maybe Xp.decoder)
        |> required "total_xp" (maybe Xp.decoder)
        |> andThen toSalaryDecoder
