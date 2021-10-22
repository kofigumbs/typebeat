module State exposing (Param, Song, State, Track, bool, fromValue, get, int, list)

import Dict exposing (Dict)
import Json.Decode as D



-- PARAM


type Param s a
    = Param String (List String) (D.Decoder a) (List a -> a)


int : String -> Param s Int
int name =
    Param name [] D.int (List.head >> Maybe.withDefault 0)


bool : String -> Param s Bool
bool name =
    Param name [] D.bool (List.head >> Maybe.withDefault False)


list : Int -> Param s a -> Param s (List a)
list length (Param name _ decoder combine) =
    Param
        (name ++ "0")
        (List.map ((++) name << String.fromInt) (List.range 1 (length - 1)))
        (D.map List.singleton decoder)
        List.concat



-- STATE


type State a
    = State (Dict String D.Value)


type Song
    = Song


type Track
    = Track


fromValue : D.Value -> State a
fromValue value =
    D.decodeValue (D.dict D.value) value
        |> Result.withDefault Dict.empty
        |> State


get : Param s a -> State s -> a
get (Param firstName otherNames decoder combine) (State dict) =
    let
        getOne name =
            Dict.get name dict
                |> Maybe.andThen (D.decodeValue decoder >> Result.toMaybe)
    in
    combine (List.filterMap getOne (firstName :: otherNames))
