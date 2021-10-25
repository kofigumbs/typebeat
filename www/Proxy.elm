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
    { song : Song
    , tracks : Array Track
    }


dump : D.Decoder State
dump =
    D.map2 State
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
            { state | tracks = Array.indexedMap (Param.replaceAt (id - 1) (Param.change name value Track.decoder)) state.tracks }


type Event
    = NoOp
    | Send String Int


type alias Action =
    { label : String
    , title : Bool
    , onDown : Event
    , onUp : Event
    }


defaultAction : Action
defaultAction =
    Action "" False NoOp NoOp


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


note : Int -> Maybe String
note key =
    Array.get (modBy 12 key) noteNames
        |> Maybe.map (\name -> name ++ String.fromInt (key // 12 - 1))


noteNames : Array String
noteNames =
    Array.fromList [ "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ]
