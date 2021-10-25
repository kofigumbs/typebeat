module Mode.Audition exposing (actions)

import Action exposing (Action)
import State exposing (State)


actions_ : Action.Dict
actions_ =
    Action.all <|
        \_ i ->
            { label = ""
            , title = False
            , onDown = Action.Send "auditionDown" i
            , onUp = Action.Send "auditionUp" i
            }


actions : State -> Action.Dict
actions _ =
    actions_
