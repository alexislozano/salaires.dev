module Flags exposing (..)


type alias Flags =
    { apiUrl : String
    , hCaptchaKey : String
    , maintenance : Bool
    , noInsert : Bool
    }
