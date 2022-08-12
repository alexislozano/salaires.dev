module Services.Supabase exposing (..)

import Flags exposing (Flags)
import Http


headers : Flags -> List Http.Header
headers flags =
    [ Http.header "apikey" flags.supabaseKey
    , Http.header "Authorization" <| "Bearer " ++ flags.supabaseKey
    ]


getAllSalariesUrl : Flags -> String
getAllSalariesUrl flags =
    flags.supabaseUrl ++ "salaries?select=*"


getAllCompaniesUrl : Flags -> String
getAllCompaniesUrl flags =
    flags.supabaseUrl ++ "companies?select=*&order=company"


getAllLocationsUrl : Flags -> String
getAllLocationsUrl flags =
    flags.supabaseUrl ++ "locations?select=*&order=location"


insertSalaryUrl : Flags -> String
insertSalaryUrl flags =
    flags.supabaseUrl ++ "salaries"
