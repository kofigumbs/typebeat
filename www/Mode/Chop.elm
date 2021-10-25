module Mode.Chop exposing (actions, visual)

import Array exposing (Array)
import Dict
import Proxy exposing (Track)
import Svg exposing (..)
import Svg.Attributes exposing (..)
import Svg.Lazy


visual : Proxy.State -> Svg msg
visual state =
    case Proxy.activeTrack state of
        Nothing ->
            text ""

        Just activeTrack ->
            Svg.Lazy.lazy visual_ activeTrack.waveform


visual_ : Array Int -> Svg msg
visual_ waveform =
    svg [] (List.indexedMap path (Array.toList waveform))


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
