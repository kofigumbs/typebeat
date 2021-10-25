module Key exposing (..)

import Dict


type Modifier
    = Q
    | W
    | E
    | R
    | T
    | A
    | S
    | D
    | F
    | G
    | Z
    | X
    | C
    | V
    | B


type Action
    = Y
    | U
    | I
    | O
    | P
    | H
    | J
    | K
    | L
    | Semicolon
    | N
    | M
    | Comma
    | Period
    | Slash


type Key
    = Modifier Modifier
    | Action Action


{-| Type alias to communicate that a Dict should only be used with types from this module
-}
type alias Dict a =
    Dict.Dict String a


code : Key -> String
code key =
    case key of
        Modifier Q ->
            "KeyQ"

        Modifier W ->
            "KeyW"

        Modifier E ->
            "KeyE"

        Modifier R ->
            "KeyR"

        Modifier T ->
            "KeyT"

        Action Y ->
            "KeyY"

        Action U ->
            "KeyU"

        Action I ->
            "KeyI"

        Action O ->
            "KeyO"

        Action P ->
            "KeyP"

        Modifier A ->
            "KeyA"

        Modifier S ->
            "KeyS"

        Modifier D ->
            "KeyD"

        Modifier F ->
            "KeyF"

        Modifier G ->
            "KeyG"

        Action H ->
            "KeyH"

        Action J ->
            "KeyJ"

        Action K ->
            "KeyK"

        Action L ->
            "KeyL"

        Action Semicolon ->
            "Semicolon"

        Modifier Z ->
            "KeyZ"

        Modifier X ->
            "KeyX"

        Modifier C ->
            "KeyC"

        Modifier V ->
            "KeyV"

        Modifier B ->
            "KeyB"

        Action N ->
            "KeyN"

        Action M ->
            "KeyM"

        Action Comma ->
            "Comma"

        Action Period ->
            "Period"

        Action Slash ->
            "Slash"


all : Dict Key
all =
    let
        toPair key =
            ( code key, key )
    in
    Dict.fromList
        [ toPair (Modifier Q)
        , toPair (Modifier W)
        , toPair (Modifier E)
        , toPair (Modifier R)
        , toPair (Modifier T)
        , toPair (Action Y)
        , toPair (Action U)
        , toPair (Action I)
        , toPair (Action O)
        , toPair (Action P)
        , toPair (Modifier A)
        , toPair (Modifier S)
        , toPair (Modifier D)
        , toPair (Modifier F)
        , toPair (Modifier G)
        , toPair (Action H)
        , toPair (Action J)
        , toPair (Action K)
        , toPair (Action L)
        , toPair (Action Semicolon)
        , toPair (Modifier Z)
        , toPair (Modifier X)
        , toPair (Modifier C)
        , toPair (Modifier V)
        , toPair (Modifier B)
        , toPair (Action N)
        , toPair (Action M)
        , toPair (Action Comma)
        , toPair (Action Period)
        , toPair (Action Slash)
        ]
