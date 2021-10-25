module Main exposing (..)

import Browser
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (..)
import Js
import Json.Decode as D
import Key exposing (Key)
import Mode
import Mode.Audition
import Proxy


type alias Model =
    { modifier : Maybe Key.Modifier
    , state : Result D.Error Proxy.State
    }


init : D.Value -> ( Model, Cmd Msg )
init flags =
    ( Model Nothing (D.decodeValue Proxy.dump flags), Cmd.none )


type Direction
    = Down
    | Up


getEvent : Direction -> Proxy.Action -> Proxy.Event
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


getActions : Maybe Key.Modifier -> Proxy.State -> Proxy.Actions
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

                Just (Proxy.Send method data) ->
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
                [ viewRow model state [ Key.Q, Key.W, Key.E, Key.R, Key.T ] [ Key.Y, Key.U, Key.I, Key.O, Key.P ]
                , viewRow model state [ Key.A, Key.S, Key.D, Key.F, Key.G ] [ Key.H, Key.J, Key.K, Key.L, Key.Semicolon ]
                , viewRow model state [ Key.Z, Key.X, Key.C, Key.V, Key.B ] [ Key.N, Key.M, Key.Comma, Key.Period, Key.Slash ]
                ]
    }


viewRow : Model -> Proxy.State -> List Key.Modifier -> List Key.Action -> Html Msg
viewRow model state modifiers actions =
    div [ class "row" ] <|
        List.map (viewModifier model state) modifiers
            ++ List.map (viewAction model state) actions


viewModifier : Model -> Proxy.State -> Key.Modifier -> Html Msg
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


viewAction : Model -> Proxy.State -> Key.Action -> Html Msg
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
