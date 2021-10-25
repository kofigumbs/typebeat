port module Js exposing (..)

import Json.Decode as D


port keyboardEvent : (D.Value -> msg) -> Sub msg


port change : (( Int, String, D.Value ) -> msg) -> Sub msg


port send : { method : String, data : Int } -> Cmd msg
