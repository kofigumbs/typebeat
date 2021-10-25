module Main exposing (..)

import Action
import Browser
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (..)
import Js
import Json.Decode as D
import Key exposing (Action(..), Key, Modifier(..))
import Mode
import Mode.Audition
import State exposing (State)


type alias Model =
    { modifier : Maybe Modifier
    , state : Result D.Error State
    }


init : D.Value -> ( Model, Cmd Msg )
init flags =
    ( Model Nothing (D.decodeValue State.decoder flags), Cmd.none )


type Direction
    = Down
    | Up


getEvent : Direction -> Action.Action -> Action.Event
getEvent direction action =
    case direction of
        Down ->
            action.onDown

        Up ->
            action.onUp


keyboardEventDirections : Dict String Direction
keyboardEventDirections =
    Dict.fromList [ ( "keydown", Down ), ( "keyup", Up ) ]


type Msg
    = KeyboardEvent (Result D.Error ( Direction, Key ))


getActions : Maybe Modifier -> State -> Key.Dict Action.Action
getActions =
    Maybe.map (Mode.fromModifier >> .actions)
        >> Maybe.withDefault Mode.Audition.actions


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        KeyboardEvent (Err _) ->
            ( model, Cmd.none )

        KeyboardEvent (Ok ( Down, Key.Modifier modifier )) ->
            let
                newModifier =
                    if model.modifier == Just modifier then
                        Nothing

                    else
                        Just modifier
            in
            ( { model | modifier = newModifier }, Cmd.none )

        KeyboardEvent (Ok ( Up, Key.Modifier modifier )) ->
            ( model, Cmd.none )

        KeyboardEvent (Ok ( direction, key )) ->
            case
                Result.toMaybe model.state
                    |> Maybe.map (getActions model.modifier)
                    |> Maybe.andThen (Dict.get (Key.code key))
                    |> Maybe.map (getEvent direction)
            of
                Nothing ->
                    ( model, Cmd.none )

                Just (Action.Send method data) ->
                    ( model, Js.send { method = method, data = data } )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ Js.keyboardEvent (KeyboardEvent << D.decodeValue keyboardEventDecoder)
        ]


keyboardEventDecoder : D.Decoder ( Direction, Key )
keyboardEventDecoder =
    D.map2 Tuple.pair
        (D.field "type" D.string |> inDict keyboardEventDirections)
        (D.field "code" D.string |> inDict Key.all)


inDict : Dict comparable value -> D.Decoder comparable -> D.Decoder value
inDict dict =
    D.andThen <|
        \key ->
            case Dict.get key dict of
                Nothing ->
                    D.fail "No key/value in Dict"

                Just value ->
                    D.succeed value


view : Model -> Browser.Document Msg
view model =
    { title = "Typebeat"
    , body =
        case model.state of
            Err _ ->
                []

            Ok state ->
                [ viewRow model state [ KeyQ, KeyW, KeyE, KeyR, KeyT ] [ KeyY, KeyU, KeyI, KeyO, KeyP ]
                , viewRow model state [ KeyA, KeyS, KeyD, KeyF, KeyG ] [ KeyH, KeyJ, KeyK, KeyL, Semicolon ]
                , viewRow model state [ KeyZ, KeyX, KeyC, KeyV, KeyB ] [ KeyN, KeyM, Comma, Period, Slash ]
                ]
    }


viewRow : Model -> State -> List Modifier -> List Action -> Html Msg
viewRow model state modifiers actions =
    div [ class "row" ] <|
        List.map (viewModifier model state) modifiers
            ++ List.map (viewAction model state) actions


viewModifier : Model -> State -> Modifier -> Html Msg
viewModifier model state modifier =
    let
        { name, visual } =
            Mode.fromModifier modifier
    in
    button [ class "key mode", classList [ ( "active", model.modifier == Just modifier ) ] ]
        [ node "custom-element-tare"
            [ attribute "aria-label" name
            , style "width" "100%"
            , style "height" "100%"
            ]
            []
        , div [ class "visual" ] [ Html.map never (visual state) ]
        ]


viewAction : Model -> State -> Action -> Html Msg
viewAction model state action =
    let
        name =
            getActions model.modifier state
                |> Dict.get (Key.code (Key.Action action))
                |> Maybe.map .label
                |> Maybe.withDefault ""
    in
    button [ class "key action" ]
        [ node "custom-element-mono" [ attribute "aria-label" name ] []
        ]


main : Program D.Value Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }
