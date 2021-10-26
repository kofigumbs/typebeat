module Mode.Mix exposing (actions, visual)

import Proxy
import Dict
import Key
import Svg exposing (Svg)


visual : Proxy.State -> Svg msg
visual state =
    Svg.text ""


actions : Proxy.State -> Proxy.Actions
actions state =
    case Proxy.activeTrack state of
        Nothing ->
            Proxy.bindNone

        Just activeTrack ->
            Proxy.bindTabs "mix"
                [ ( Key.Y, "main", activeTrack.main )
                , ( Key.U, "pan", activeTrack.pan )
                , ( Key.I, "reverb", activeTrack.reverb )
                , ( Key.O, "echo", activeTrack.echo )
                , ( Key.P, "drive", activeTrack.drive )
                ]
                state
