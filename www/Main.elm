module Main exposing (..)

import Array exposing (Array)
import Browser
import Html exposing (..)
import Json.Decode as D
import Param
import Song exposing (Song)
import Track exposing (Track)


type alias State =
    { song : Song
    , tracks : Array Track
    }


type alias Model =
    { state : Result D.Error State
    }


init : D.Value -> ( Model, Cmd Msg )
init flags =
    let
        state =
            D.map2 State
                (D.field "song" (Param.dump Song.decoder))
                (D.field "tracks" (D.array (Param.dump Track.decoder)))
    in
    ( Model (D.decodeValue state flags), Cmd.none )


type Msg
    = KeyboardEvent Char


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


view : Model -> Browser.Document Msg
view model =
    { title = "Typebeat"
    , body = [ text (Debug.toString model) ]
    }


main : Program D.Value Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }
