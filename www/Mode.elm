module Mode exposing (Mode, fromModifier)

import Key
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
import Proxy
import Svg exposing (Svg)


type alias Mode =
    { name : String
    , visual : Proxy.State -> Svg Never
    , actions : Proxy.State -> Proxy.Actions
    }


fromModifier : Key.Modifier -> Mode
fromModifier modifier =
    case modifier of
        Key.Q ->
            { name = "Track"
            , visual = Mode.Track.visual
            , actions = Mode.Track.actions
            }

        Key.W ->
            { name = "Sound"
            , visual = Mode.Sound.visual
            , actions = Mode.Sound.actions
            }

        Key.E ->
            { name = "Chop"
            , visual = Mode.Chop.visual
            , actions = Mode.Chop.actions
            }

        Key.R ->
            { name = "Range"
            , visual = Mode.Range.visual
            , actions = Mode.Range.actions
            }

        Key.T ->
            { name = "Note"
            , visual = Mode.Note.visual
            , actions = Mode.Note.actions
            }

        Key.A ->
            { name = "Beat"
            , visual = Mode.Beat.visual
            , actions = Mode.Beat.actions
            }

        Key.S ->
            { name = "Loop"
            , visual = Mode.Loop.visual
            , actions = Mode.Loop.actions
            }

        Key.D ->
            { name = "Hold"
            , visual = Mode.Hold.visual
            , actions = Mode.Hold.actions
            }

        Key.F ->
            { name = "EQ"
            , visual = Mode.EQ.visual
            , actions = Mode.EQ.actions
            }

        Key.G ->
            { name = "Mix"
            , visual = Mode.Mix.visual
            , actions = Mode.Mix.actions
            }

        Key.Z ->
            { name = "Key"
            , visual = Mode.Key.visual
            , actions = Mode.Key.actions
            }

        Key.X ->
            { name = "Auto"
            , visual = Mode.Auto.visual
            , actions = Mode.Auto.actions
            }

        Key.C ->
            { name = "Send"
            , visual = Mode.Send.visual
            , actions = Mode.Send.actions
            }

        Key.V ->
            { name = "Tape"
            , visual = Mode.Tape.visual
            , actions = Mode.Tape.actions
            }

        Key.B ->
            { name = "Mute"
            , visual = Mode.Mute.visual
            , actions = Mode.Mute.actions
            }
