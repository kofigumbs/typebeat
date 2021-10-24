module Main exposing (..)

import Action
import Browser
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (..)
import Json.Decode as D
import Key exposing (Action(..), Modifier(..))
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


type Msg
    = Event Action.Event


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


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
    button [ class "key mode" ]
        [ node "custom-element-tare"
            [ attribute "aria-label" name
            , style "width" "100%"
            , style "height" "100%"
            ]
            []
        , div [ class "visual" ] [ Html.map Event (visual state) ]
        ]


viewAction : Model -> State -> Action -> Html Msg
viewAction model state action =
    let
        actions =
            Maybe.map (Mode.fromModifier >> .actions) model.modifier
                |> Maybe.withDefault Mode.Audition.actions

        name =
            Dict.get (Key.code (Key.Action action)) (actions state)
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
