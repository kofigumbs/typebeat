module State exposing (Song, State, Track, activeTrack, decoder)

import Array exposing (Array)
import Json.Decode as D
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


decoder : D.Decoder State
decoder =
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
