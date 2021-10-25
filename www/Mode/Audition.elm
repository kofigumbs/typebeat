module Mode.Audition exposing (actions)

import Proxy


actions_ : Proxy.Actions
actions_ =
    Proxy.bindAll <|
        \i ->
            { label = ""
            , title = False
            , onDown = Proxy.Send "auditionDown" i
            , onUp = Proxy.Send "auditionUp" i
            }


actions : state -> Proxy.Actions
actions _ =
    actions_
