module Key exposing (..)

import Dict


type Modifier
    = KeyQ
    | KeyW
    | KeyE
    | KeyR
    | KeyT
    | KeyA
    | KeyS
    | KeyD
    | KeyF
    | KeyG
    | KeyZ
    | KeyX
    | KeyC
    | KeyV
    | KeyB


type Action
    = KeyY
    | KeyU
    | KeyI
    | KeyO
    | KeyP
    | KeyH
    | KeyJ
    | KeyK
    | KeyL
    | Semicolon
    | KeyN
    | KeyM
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
        Modifier KeyQ ->
            "KeyQ"

        Modifier KeyW ->
            "KeyW"

        Modifier KeyE ->
            "KeyE"

        Modifier KeyR ->
            "KeyR"

        Modifier KeyT ->
            "KeyT"

        Action KeyY ->
            "KeyY"

        Action KeyU ->
            "KeyU"

        Action KeyI ->
            "KeyI"

        Action KeyO ->
            "KeyO"

        Action KeyP ->
            "KeyP"

        Modifier KeyA ->
            "KeyA"

        Modifier KeyS ->
            "KeyS"

        Modifier KeyD ->
            "KeyD"

        Modifier KeyF ->
            "KeyF"

        Modifier KeyG ->
            "KeyG"

        Action KeyH ->
            "KeyH"

        Action KeyJ ->
            "KeyJ"

        Action KeyK ->
            "KeyK"

        Action KeyL ->
            "KeyL"

        Action Semicolon ->
            "Semicolon"

        Modifier KeyZ ->
            "KeyZ"

        Modifier KeyX ->
            "KeyX"

        Modifier KeyC ->
            "KeyC"

        Modifier KeyV ->
            "KeyV"

        Modifier KeyB ->
            "KeyB"

        Action KeyN ->
            "KeyN"

        Action KeyM ->
            "KeyM"

        Action Comma ->
            "Comma"

        Action Period ->
            "Period"

        Action Slash ->
            "Slash"


all : Dict Key
all =
    Dict.fromList
        [ ( code (Modifier KeyQ), Modifier KeyQ )
        , ( code (Modifier KeyW), Modifier KeyW )
        , ( code (Modifier KeyE), Modifier KeyE )
        , ( code (Modifier KeyR), Modifier KeyR )
        , ( code (Modifier KeyT), Modifier KeyT )
        , ( code (Action KeyY), Action KeyY )
        , ( code (Action KeyU), Action KeyU )
        , ( code (Action KeyI), Action KeyI )
        , ( code (Action KeyO), Action KeyO )
        , ( code (Action KeyP), Action KeyP )
        , ( code (Modifier KeyA), Modifier KeyA )
        , ( code (Modifier KeyS), Modifier KeyS )
        , ( code (Modifier KeyD), Modifier KeyD )
        , ( code (Modifier KeyF), Modifier KeyF )
        , ( code (Modifier KeyG), Modifier KeyG )
        , ( code (Action KeyH), Action KeyH )
        , ( code (Action KeyJ), Action KeyJ )
        , ( code (Action KeyK), Action KeyK )
        , ( code (Action KeyL), Action KeyL )
        , ( code (Action Semicolon), Action Semicolon )
        , ( code (Modifier KeyZ), Modifier KeyZ )
        , ( code (Modifier KeyX), Modifier KeyX )
        , ( code (Modifier KeyC), Modifier KeyC )
        , ( code (Modifier KeyV), Modifier KeyV )
        , ( code (Modifier KeyB), Modifier KeyB )
        , ( code (Action KeyN), Action KeyN )
        , ( code (Action KeyM), Action KeyM )
        , ( code (Action Comma), Action Comma )
        , ( code (Action Period), Action Period )
        , ( code (Action Slash), Action Slash )
        ]
