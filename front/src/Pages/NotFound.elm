module Pages.NotFound exposing (..)

import Css
import Html.Styled as Html exposing (Html)
import Html.Styled.Attributes as Attributes
import I18n


view : List (Html msg)
view =
    [ Html.div
        [ Attributes.css
            [ Css.displayFlex
            , Css.height (Css.pct 100)
            , Css.alignItems Css.center
            , Css.justifyContent Css.center
            ]
        ]
        [ Html.text (I18n.translate I18n.French I18n.ThisPageDoesNotExist) ]
    ]
