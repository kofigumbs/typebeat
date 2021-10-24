module State exposing (State, decoder)

import Array exposing (Array)
import Json.Decode as D
import Param
import Song exposing (Song)
import Track exposing (Track)


type alias State =
    { song : Song
    , tracks : Array Track
    }


decoder : D.Decoder State
decoder =
    D.map2 State
        (D.field "song" (Param.dump Song.decoder))
        (D.field "tracks" (D.array (Param.dump Track.decoder)))
