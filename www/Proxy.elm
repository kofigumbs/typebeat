module Proxy exposing
    ( Action
    , Actions
    , Change(..)
    , Event(..)
    , Song
    , State
    , Track
    , activeTrack
    , apply
    , bindAll
    , bindNone
    , bindTabs
    , defaultAction
    , dump
    , note
    )

import Array exposing (Array)
import Dict exposing (Dict)
import Json.Decode as D
import Key
import Param
import Song
import Track


type alias Song =
    Song.Song


type alias Track =
    Track.Track


type alias State =
    { local : Dict String String
    , song : Song
    , tracks : Array Track
    }


dump : D.Decoder State
dump =
    D.map2 (State Dict.empty)
        (D.field "song" (Param.dump Song.decoder))
        (D.field "tracks" (D.array (Param.dump Track.decoder)))


activeTrack : State -> Maybe Track
activeTrack state =
    Array.get state.song.activeTrackId state.tracks


type Change
    = Change ( Int, String, D.Value )


apply : Change -> State -> State
apply (Change ( id, name, value )) state =
    case id of
        0 ->
            { state | song = Param.change name value Song.decoder state.song }

        _ ->
            { state | tracks = updateAt (id - 1) (Param.change name value Track.decoder) state.tracks }


updateAt : Int -> (a -> a) -> Array a -> Array a
updateAt target update =
    Array.indexedMap <|
        \i a ->
            if i == target then
                update a

            else
                a


type Event
    = Send String Int
    | Local String String


type alias Action =
    { label : String
    , title : Bool
    , onDown : Maybe Event
    , onUp : Maybe Event
    }


defaultAction : Action
defaultAction =
    Action "" False Nothing Nothing


type alias Actions =
    Dict String Action


bindNone : Actions
bindNone =
    Dict.empty


bindAll : (Int -> Action) -> Actions
bindAll toAction =
    [ Key.N, Key.M, Key.Comma, Key.Period, Key.Slash, Key.H, Key.J, Key.K, Key.L, Key.Semicolon, Key.Y, Key.U, Key.I, Key.O, Key.P ]
        |> List.indexedMap (\i action -> ( Key.code (Key.Action action), toAction i ))
        |> Dict.fromList


bindTabs : String -> List ( Key.Action, String, Int ) -> State -> Actions
bindTabs category tabs state =
    let
        local =
            Dict.get category state.local
    in
    List.indexedMap
        (\i ( key, label, value ) ->
            let
                action =
                    { label = label
                    , title = (local == Just label) || (local == Nothing && i == 0)
                    , onDown = Just (Local category label)
                    , onUp = Nothing
                    }

                actions =
                    Dict.singleton (Key.code (Key.Action key)) action
            in
            if not action.title then
                actions

            else
                Dict.union actions (bindNudge label value)
        )
        tabs
        |> List.foldl Dict.union Dict.empty


bindNudge : String -> Int -> Actions
bindNudge method value =
    let
        nudge label i =
            { label = label
            , title = False
            , onDown = Just (Send method i)
            , onUp = Nothing
            }
    in
    Dict.fromList
        [ ( Key.code (Key.Action Key.H), nudge "-10" 0 )
        , ( Key.code (Key.Action Key.J), nudge "-1" 1 )
        , ( Key.code (Key.Action Key.K), Action (String.fromInt value) True Nothing Nothing )
        , ( Key.code (Key.Action Key.L), nudge "+1" 2 )
        , ( Key.code (Key.Action Key.Semicolon), nudge "+10" 3 )
        ]


note : Int -> Maybe String
note key =
    Array.get (modBy 12 key) noteNames
        |> Maybe.map (\name -> name ++ String.fromInt (key // 12 - 1))


noteNames : Array String
noteNames =
    Array.fromList [ "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ]
