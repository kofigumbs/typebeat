module Main exposing (..)

import Array exposing (Array)
import Browser
import Html exposing (..)
import State exposing (Song, State, Track)
import Json.Decode as D


main =
    Browser.document
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }


type alias Flags =
    { song : D.Value
    , tracks : Array D.Value
    }


type alias Model =
    { song : State Song
    , tracks : Array (State Track)
    }


init : Flags -> ( Model, Cmd Msg )
init flags =
    ( Model (State.fromValue flags.song) (Array.map State.fromValue flags.tracks)
    , Cmd.none
    )


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
