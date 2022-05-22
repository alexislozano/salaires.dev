module Utils exposing (..)


map :
    (model1 -> model2)
    -> (msg1 -> msg2)
    -> ( model1, Cmd msg1 )
    -> ( model2, Cmd msg2 )
map toModel toCmd ( model, cmd ) =
    ( toModel model, Cmd.map toCmd cmd )
