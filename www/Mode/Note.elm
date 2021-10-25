module Mode.Note exposing (actions, visual)

import Array
import Proxy
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
            let
                notes =
                    Array.fromList activeTrack.note
            in
            Proxy.bindAll <|
                \i ->
                    { label =
                        Array.get i notes
                            |> Maybe.andThen Proxy.note
                            |> Maybe.withDefault ""
                    , title = i == activeTrack.activeKey
                    , onDown = Proxy.Send "noteDown" i
                    , onUp = Proxy.Send "noteUp" i
                    }
