module Mode.Note exposing (actions, visual)

import Array
import Proxy
import Svg exposing (Svg)
import Svg.Attributes
import Svg.Lazy


visual : Proxy.State -> Svg msg
visual state =
    Proxy.activeTrack state
        |> Maybe.andThen activeNote
        |> Maybe.withDefault -1
        |> Svg.Lazy.lazy visual_

activeNote : Proxy.Track -> Maybe Int
activeNote track =
    Array.get track.activeKey track.note
        |> Maybe.map (modBy 12)


visual_ : Int -> Svg msg
visual_ activeKey =
    Svg.svg []
        [ key white 0 (activeKey == 0)
        , key white 1 (activeKey == 2)
        , key white 2 (activeKey == 4)
        , key white 3 (activeKey == 5)
        , key white 4 (activeKey == 7)
        , key white 5 (activeKey == 9)
        , key white 6 (activeKey == 11)
        , key black 1 (activeKey == 1)
        , key black 2 (activeKey == 3)
        , key black 4 (activeKey == 6)
        , key black 5 (activeKey == 8)
        , key black 6 (activeKey == 10)
        ]


white =
    { width = 14
    , xOffset = 0
    , height = 49
    }


black =
    { width = 10
    , xOffset = 5
    , height = white.height // 2
    }


key : { width : Int, xOffset : Int, height : Int } -> Int -> Bool -> Svg msg
key { width, xOffset, height } x active =
    Svg.rect
        [ Svg.Attributes.x (String.fromInt (x * white.width - xOffset - 2))
        , Svg.Attributes.y "-2"
        , Svg.Attributes.width (String.fromInt width)
        , Svg.Attributes.height (String.fromInt height)
        , Svg.Attributes.strokeWidth "2"
        , if active then
            Svg.Attributes.fill "var(--secondary)"

          else
            Svg.Attributes.fill "var(--key_background)"
        ]
        []


actions : Proxy.State -> Proxy.Actions
actions state =
    case Proxy.activeTrack state of
        Nothing ->
            Proxy.bindNone

        Just activeTrack ->
            Proxy.bindAll <|
                \i ->
                    { label =
                        Array.get i activeTrack.note
                            |> Maybe.andThen Proxy.note
                            |> Maybe.withDefault ""
                    , title = i == activeTrack.activeKey
                    , onDown = Just (Proxy.Send "noteDown" i)
                    , onUp = Just (Proxy.Send "noteUp" i)
                    }
