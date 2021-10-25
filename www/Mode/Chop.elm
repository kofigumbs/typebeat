module Mode.Chop exposing (actions, visual)

import Dict
import State exposing (State, Track)
import Svg exposing (..)
import Svg.Attributes exposing (..)


visual : State -> Svg msg
visual state =
    let
        activeTrack =
            State.activeTrack state
    in
    svg [] (List.indexedMap path activeTrack.waveform)


path : Int -> Int -> Svg msg
path i value =
    let
        y =
            toFloat value / 5.0 + 1.0
    in
    Svg.path
        [ strokeWidth "2"
        , d <|
            "M "
                ++ String.fromFloat (toFloat i * 4.0 + 3.0)
                ++ " "
                ++ String.fromFloat (23.0 - y)
                ++ " v "
                ++ String.fromFloat (y * 2.0)
        ]
        []


actions _ =
    Dict.empty
