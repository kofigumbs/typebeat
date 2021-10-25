module Proxy exposing (Action, Actions, Event(..), Song, State, Track, actions, activeTrack, dump)

import Array exposing (Array)
import Dict
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
    { song : Song
    , tracks : Array Track
    }


dump : D.Decoder State
dump =
    D.map2 State
        (D.field "song" (Param.dump Song.decoder))
        (D.field "tracks" (D.array (Param.dump Track.decoder)))


activeTrack : State -> Track
activeTrack state =
    case Array.get state.song.activeTrackId state.tracks of
        Just track ->
            track

        Nothing ->
            activeTrack state


type Event
    = Send String Int


type alias Action =
    { label : String
    , title : Bool
    , onDown : Event
    , onUp : Event
    }


type alias Actions =
    Key.Dict Action


actions : (Key.Action -> Int -> Action) -> Actions
actions f =
    Dict.fromList
        [ ( Key.code (Key.Action Key.Y), f Key.Y 10 )
        , ( Key.code (Key.Action Key.U), f Key.U 11 )
        , ( Key.code (Key.Action Key.I), f Key.I 12 )
        , ( Key.code (Key.Action Key.O), f Key.O 13 )
        , ( Key.code (Key.Action Key.P), f Key.P 14 )
        , ( Key.code (Key.Action Key.H), f Key.H 5 )
        , ( Key.code (Key.Action Key.J), f Key.J 6 )
        , ( Key.code (Key.Action Key.K), f Key.K 7 )
        , ( Key.code (Key.Action Key.L), f Key.L 8 )
        , ( Key.code (Key.Action Key.Semicolon), f Key.Semicolon 9 )
        , ( Key.code (Key.Action Key.N), f Key.N 0 )
        , ( Key.code (Key.Action Key.M), f Key.M 1 )
        , ( Key.code (Key.Action Key.Comma), f Key.Comma 2 )
        , ( Key.code (Key.Action Key.Period), f Key.Period 3 )
        , ( Key.code (Key.Action Key.Slash), f Key.Slash 4 )
        ]
