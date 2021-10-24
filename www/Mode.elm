module Mode exposing (Mode, fromModifier)

import Action exposing (Action)
import Html exposing (Html)
import Key exposing (Modifier(..))
import Mode.Auto
import Mode.Beat
import Mode.Chop
import Mode.EQ
import Mode.Hold
import Mode.Key
import Mode.Loop
import Mode.Mix
import Mode.Mute
import Mode.Note
import Mode.Range
import Mode.Send
import Mode.Sound
import Mode.Tape
import Mode.Track
import State exposing (State)


type alias Mode =
    { name : String
    , visual : State -> Html Action.Event
    , actions : State -> Key.Dict Action
    }


fromModifier : Modifier -> Mode
fromModifier modifier =
    case modifier of
        KeyQ ->
            { name = "Track"
            , visual = Mode.Track.visual
            , actions = Mode.Track.actions
            }

        KeyW ->
            { name = "Sound"
            , visual = Mode.Sound.visual
            , actions = Mode.Sound.actions
            }

        KeyE ->
            { name = "Chop"
            , visual = Mode.Chop.visual
            , actions = Mode.Chop.actions
            }

        KeyR ->
            { name = "Range"
            , visual = Mode.Range.visual
            , actions = Mode.Range.actions
            }

        KeyT ->
            { name = "Note"
            , visual = Mode.Note.visual
            , actions = Mode.Note.actions
            }

        KeyA ->
            { name = "Beat"
            , visual = Mode.Beat.visual
            , actions = Mode.Beat.actions
            }

        KeyS ->
            { name = "Loop"
            , visual = Mode.Loop.visual
            , actions = Mode.Loop.actions
            }

        KeyD ->
            { name = "Hold"
            , visual = Mode.Hold.visual
            , actions = Mode.Hold.actions
            }

        KeyF ->
            { name = "EQ"
            , visual = Mode.EQ.visual
            , actions = Mode.EQ.actions
            }

        KeyG ->
            { name = "Mix"
            , visual = Mode.Mix.visual
            , actions = Mode.Mix.actions
            }

        KeyZ ->
            { name = "Key"
            , visual = Mode.Key.visual
            , actions = Mode.Key.actions
            }

        KeyX ->
            { name = "Auto"
            , visual = Mode.Auto.visual
            , actions = Mode.Auto.actions
            }

        KeyC ->
            { name = "Send"
            , visual = Mode.Send.visual
            , actions = Mode.Send.actions
            }

        KeyV ->
            { name = "Tape"
            , visual = Mode.Tape.visual
            , actions = Mode.Tape.actions
            }

        KeyB ->
            { name = "Mute"
            , visual = Mode.Mute.visual
            , actions = Mode.Mute.actions
            }
