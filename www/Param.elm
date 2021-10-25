module Param exposing (Decoder, apply, change, dump, field, list, replaceAt, succeed)

import Dict exposing (Dict)
import Json.Decode as D



-- Decode params


type Decoder s a
    = Decoder (Dict String (D.Value -> s -> s)) (D.Decoder a)


field : (a -> s -> s) -> D.Decoder a -> String -> Decoder s a
field setter decoder name =
    Decoder
        (Dict.singleton name <|
            \value state ->
                case D.decodeValue decoder value of
                    Err _ ->
                        state

                    Ok value_ ->
                        setter value_ state
        )
        (D.field name decoder)


list : Int -> (s -> List a) -> (List a -> s -> s) -> D.Decoder a -> String -> Decoder s (List a)
list length getter setter decoder name =
    let
        indexes =
            List.range 0 (length - 1)

        nameAt i =
            name ++ String.fromInt i

        changeAt i value state =
            D.decodeValue decoder value
                |> Result.map (\x -> setter (List.indexedMap (replaceAt i (\_ -> x)) (getter state)) state)
                |> Result.withDefault state
    in
    Decoder
        (Dict.fromList (List.map (\i -> ( nameAt i, changeAt i )) indexes))
        (List.foldr (\i -> D.map2 (::) (D.field (nameAt i) decoder)) (D.succeed []) indexes)


replaceAt : Int -> (a -> a) -> Int -> a -> a
replaceAt target replace i value =
    if i == target then
        replace value

    else
        value



-- Combining decoders


succeed : a -> Decoder s a
succeed a =
    Decoder Dict.empty (D.succeed a)


apply : Decoder s a -> Decoder s (a -> b) -> Decoder s b
apply (Decoder changeA a) (Decoder changeF f) =
    Decoder (Dict.union changeA changeF) (D.map2 (|>) a f)



-- Running decoders


dump : Decoder s a -> D.Decoder a
dump (Decoder _ decoder) =
    decoder


change : String -> D.Value -> Decoder s s -> s -> s
change name value (Decoder changes _) state =
    case Dict.get name changes of
        Nothing ->
            state

        Just change_ ->
            change_ value state
