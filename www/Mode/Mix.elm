module Mode.Mix exposing (actions, visual)

import Proxy
import Svg exposing (Svg)


visual : Proxy.State -> Svg msg
visual state =
    Svg.text ""


actions : Proxy.State -> Proxy.Actions
actions state =
    Proxy.bindNone
