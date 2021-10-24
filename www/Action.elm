module Action exposing (Action, Event(..))


type Event
    = Event Event


type alias Action =
    { label : String
    , title : Bool
    }
