module Param exposing (Decoder, Primitive, apply, list, bool, dump, int, succeed)

import Dict exposing (Dict)
import Json.Decode as D



-- Decode params


type Decoder s a
    = Decoder s (D.Decoder a)


type Primitive a
    = Primitive String (D.Decoder a)


int : String -> Decoder (Primitive Int) Int
int name =
    Decoder (Primitive name D.int) (D.field name D.int)


bool : String -> Decoder (Primitive Bool) Bool
bool name =
    Decoder (Primitive name D.bool) (D.field name D.bool)


list : Int -> Decoder (Primitive a) a -> Decoder () (List a)
list length (Decoder (Primitive name decoder) _) =
    let
        decodePrimitive i =
            D.field (name ++ String.fromInt i) decoder
    in
    List.range 0 (length - 1)
        |> List.foldr (decodePrimitive >> D.map2 (::)) (D.succeed [])
        |> Decoder ()



-- Combining decoders


succeed : a -> Decoder () a
succeed a =
    Decoder () (D.succeed a)


apply : Decoder s a -> Decoder () (a -> b) -> Decoder () b
apply (Decoder _ a) (Decoder _ f) =
    Decoder () (D.map2 identity f a)



-- Running decoders


dump : Decoder s a -> D.Decoder a
dump (Decoder _ decoder) =
    decoder
