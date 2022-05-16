module Services.Salaries exposing (..)

import Flags exposing (Flags)
import Http
import Json.Decode as Decode
import Models.Salary as Salary exposing (Salary)
import Services.Supabase as Supabase


getAll : Flags -> (Result Http.Error (List Salary) -> msg) -> Cmd msg
getAll flags msg =
    Http.request
        { method = "GET"
        , headers = Supabase.headers flags
        , url = Supabase.getAllSalariesUrl flags
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        , expect = Http.expectJson msg (Decode.list Salary.decode)
        }
