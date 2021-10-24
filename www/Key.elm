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
