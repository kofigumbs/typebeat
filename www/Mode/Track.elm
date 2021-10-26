module Mode.Track exposing (actions, visual)

import Html
import Proxy


visual _ =
    Html.text ""


actions : Proxy.State -> Proxy.Actions
actions state =
    Proxy.bindAll <|
        \i ->
            { label =
                if i == state.song.activeTrackId then
                    "active"

                else
                    ""
            , title = not state.song.playing
            , onDown = Just (Proxy.Send "activeTrack" i)
            , onUp = Nothing
            }
