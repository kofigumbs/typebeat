module Action exposing (Action, Dict, Event(..), all)

import Dict
import Key exposing (Key)


type Event
    = Send String Int


type alias Action =
    { label : String
    , title : Bool
    , onDown : Event
    , onUp : Event
    }


type alias Dict =
    Key.Dict Action


all : (Key.Action -> Int -> Action) -> Dict
all f =
    Dict.fromList
        [ ( Key.code (Key.Action Key.KeyY), f Key.KeyY 10 )
        , ( Key.code (Key.Action Key.KeyU), f Key.KeyU 11 )
        , ( Key.code (Key.Action Key.KeyI), f Key.KeyI 12 )
        , ( Key.code (Key.Action Key.KeyO), f Key.KeyO 13 )
        , ( Key.code (Key.Action Key.KeyP), f Key.KeyP 14 )
        , ( Key.code (Key.Action Key.KeyH), f Key.KeyH 5 )
        , ( Key.code (Key.Action Key.KeyJ), f Key.KeyJ 6 )
        , ( Key.code (Key.Action Key.KeyK), f Key.KeyK 7 )
        , ( Key.code (Key.Action Key.KeyL), f Key.KeyL 8 )
        , ( Key.code (Key.Action Key.Semicolon), f Key.Semicolon 9 )
        , ( Key.code (Key.Action Key.KeyN), f Key.KeyN 0 )
        , ( Key.code (Key.Action Key.KeyM), f Key.KeyM 1 )
        , ( Key.code (Key.Action Key.Comma), f Key.Comma 2 )
        , ( Key.code (Key.Action Key.Period), f Key.Period 3 )
        , ( Key.code (Key.Action Key.Slash), f Key.Slash 4 )
        ]
